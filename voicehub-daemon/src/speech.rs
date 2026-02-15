use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

/// Comandos para controlar o recognizer
#[derive(Debug, Clone)]
pub enum SpeechCommand {
    Start,
    Stop,
}

/// Gerenciador de reconhecimento de voz usando Web Speech API
pub struct SpeechRecognizer {
    command_tx: std::sync::mpsc::Sender<SpeechCommand>,
    is_ready: Arc<Mutex<bool>>,
}

impl SpeechRecognizer {
    /// Cria novo recognizer e inicializa WebView headless
    pub fn new(transcript_tx: mpsc::UnboundedSender<String>) -> Self {
        let is_ready = Arc::new(Mutex::new(false));
        let is_ready_clone = Arc::clone(&is_ready);

        // Canal para enviar comandos para thread GTK
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<SpeechCommand>();

        // Inicializar GTK em thread dedicada
        std::thread::spawn(move || {
            Self::gtk_thread(transcript_tx, cmd_rx, is_ready_clone);
        });

        SpeechRecognizer {
            command_tx: cmd_tx,
            is_ready,
        }
    }

    /// Thread GTK que roda o WebView
    fn gtk_thread(
        transcript_tx: mpsc::UnboundedSender<String>,
        cmd_rx: std::sync::mpsc::Receiver<SpeechCommand>,
        _is_ready: Arc<Mutex<bool>>,
    ) {
        use webkit2gtk::{UserContentManagerExt, WebContext, WebView, WebViewExt};

        if let Err(e) = gtk::init() {
            log::error!("❌ Falha ao inicializar GTK: {}", e);
            return;
        }

        let context = WebContext::default().unwrap();
        let view = WebView::with_context(&context);

        // Configurar JavaScript bridge para receber transcrições
        let user_content_manager = view.user_content_manager().unwrap();
        user_content_manager.register_script_message_handler("transcriptHandler");

        let tx_clone = transcript_tx.clone();
        user_content_manager.connect_script_message_received(
            None,
            move |_, msg: &webkit2gtk::JavascriptResult| {
                if let Some(js_value) = msg.js_value() {
                    let text = js_value.to_string();
                    log::debug!("📝 Transcrição recebida do JS: {}", text);
                    let _ = tx_clone.send(text);
                }
            },
        );

        // Carregar HTML com Web Speech API
        view.load_html(Self::get_speech_html(), None);

        // Aguardar carregamento
        let is_ready_clone = _is_ready.clone();
        view.connect_load_changed(move |_, event| {
            if event == webkit2gtk::LoadEvent::Finished {
                log::info!("✅ Web Speech API carregado");
                *is_ready_clone.lock().unwrap() = true;
            }
        });

        // Pool de comandos em background usando glib::timeout_add
        let view_clone = view.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            // Processar todos os comandos pendentes
            while let Ok(cmd) = cmd_rx.try_recv() {
                match cmd {
                    SpeechCommand::Start => {
                        view_clone.run_javascript(
                            "startRecognition();",
                            None::<&gio::Cancellable>,
                            |_| {},
                        );
                        log::debug!("🎤 JavaScript: startRecognition() chamado");
                    }
                    SpeechCommand::Stop => {
                        view_clone.run_javascript(
                            "stopRecognition();",
                            None::<&gio::Cancellable>,
                            |_| {},
                        );
                        log::debug!("⏹️  JavaScript: stopRecognition() chamado");
                    }
                }
            }
            glib::ControlFlow::Continue
        });

        // Event loop GTK
        gtk::main();
    }

    /// Verifica se o recognizer está pronto
    pub fn is_ready(&self) -> bool {
        *self.is_ready.lock().unwrap()
    }

    /// Inicia gravação de voz
    pub fn start(&self) {
        if !self.is_ready() {
            log::warn!("⚠️  Web Speech API ainda não está pronto");
            return;
        }

        let _ = self.command_tx.send(SpeechCommand::Start);
    }

    /// Para gravação de voz
    pub fn stop(&self) {
        let _ = self.command_tx.send(SpeechCommand::Stop);
    }

    /// Retorna HTML com Web Speech API embutido
    fn get_speech_html() -> &'static str {
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>VoiceHub Speech Recognizer</title>
</head>
<body>
<script>
let recognition = null;
let isRecording = false;
let accumulatedTranscript = '';
let lastProcessedIndex = 0;

// Inicializar Web Speech API
if ('webkitSpeechRecognition' in window) {
    recognition = new webkitSpeechRecognition();
    recognition.continuous = true;
    recognition.interimResults = true;
    recognition.lang = 'pt-BR';
    
    recognition.onresult = (event) => {
        let interimTranscript = '';
        let finalTranscript = '';
        
        // Processar apenas novos resultados
        for (let i = lastProcessedIndex; i < event.results.length; i++) {
            const result = event.results[i];
            const transcript = result[0].transcript;
            
            if (result.isFinal) {
                // Resultado final - adicionar pontuação automática
                finalTranscript += transcript + ' ';
                lastProcessedIndex = i + 1;
            } else {
                // Resultado parcial
                interimTranscript += transcript;
            }
        }
        
        // Acumular transcrição final
        if (finalTranscript) {
            accumulatedTranscript += finalTranscript;
        }
        
        // Enviar transcrição acumulada + parcial para Rust
        const fullText = accumulatedTranscript + interimTranscript;
        if (fullText.trim()) {
            window.webkit.messageHandlers.transcriptHandler.postMessage(fullText);
        }
    };
    
    recognition.onerror = (event) => {
        console.error('Speech recognition error:', event.error);
    };
    
    recognition.onend = () => {
        if (isRecording) {
            // Reiniciar automaticamente se ainda estiver gravando
            recognition.start();
        }
    };
    
    console.log('✅ Web Speech API initialized');
} else {
    console.error('❌ Web Speech API not supported');
}

// Funções expostas para Rust chamar
function startRecognition() {
    if (!recognition) {
        console.error('❌ Recognition not initialized');
        return;
    }
    
    if (isRecording) {
        console.warn('⚠️ Already recording');
        return;
    }
    
    // Reset state
    accumulatedTranscript = '';
    lastProcessedIndex = 0;
    isRecording = true;
    
    try {
        recognition.start();
        console.log('🎤 Recording started');
    } catch (e) {
        console.error('❌ Failed to start recognition:', e);
        isRecording = false;
    }
}

function stopRecognition() {
    if (!recognition || !isRecording) {
        return;
    }
    
    isRecording = false;
    recognition.stop();
    
    // Enviar transcrição final
    if (accumulatedTranscript.trim()) {
        window.webkit.messageHandlers.transcriptHandler.postMessage(accumulatedTranscript);
    }
    
    console.log('⏹️ Recording stopped');
}

console.log('📜 VoiceHub Speech Script loaded');
</script>
</body>
</html>
        "#
    }
}
