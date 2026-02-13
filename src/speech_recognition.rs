use webkit2gtk::{SettingsExt, UserContentManagerExt, WebContext, WebView, WebViewExt};

pub struct SpeechRecognition {
    webview: WebView,
    _context: WebContext,
    transcript_sender: Option<tokio::sync::mpsc::UnboundedSender<String>>,
}

impl SpeechRecognition {
    pub fn new() -> (Self, tokio::sync::mpsc::UnboundedReceiver<String>) {
        // Inicializar WebKit
        let context = WebContext::new();
        let webview = WebView::with_context(&context);

        // Configurar WebView
        if let Some(settings) = webview.settings() {
            settings.set_enable_javascript(true);
            settings.set_allow_file_access_from_file_urls(true);
            settings.set_allow_universal_access_from_file_urls(true);
        }

        // Canal para comunicação assíncrona
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let sr = SpeechRecognition {
            webview,
            _context: context,
            transcript_sender: Some(tx),
        };

        (sr, rx)
    }

    pub fn start(&self, language: &str) {
        let html = self.create_html(language);
        self.webview.load_html(&html, None);

        // Setup message handler para receber transcrições
        if let Some(tx) = &self.transcript_sender {
            let tx = tx.clone();
            if let Some(manager) = self.webview.user_content_manager() {
                manager.register_script_message_handler("transcript");
                manager.connect_script_message_received(None, move |_, msg| {
                    if let Some(js_value) = msg.js_value() {
                        // Usar to_string do glib
                        let text = js_value.to_string();
                        // Remover aspas se presente
                        let clean = text.trim_matches('"');
                        let _ = tx.send(clean.to_string());
                    }
                });
            }
        }
    }

    pub fn stop(&self) {
        let _ = self
            .webview
            .run_javascript_future("if(typeof stopRecognition === 'function') stopRecognition();");
    }

    fn create_html(&self, language: &str) -> String {
        format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Speech Recognition</title>
</head>
<body>
<script>
let recognition = null;
let finalTranscript = '';

function initRecognition() {{
    const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
    
    if (!SpeechRecognition) {{
        console.error('Web Speech API não suportada');
        window.webkit.messageHandlers.transcript.postMessage('ERRO: API indisponível');
        return;
    }}
    
    recognition = new SpeechRecognition();
    recognition.continuous = true;
    recognition.interimResults = true;
    recognition.lang = '{}';
    
    recognition.onstart = function() {{
        console.log('Gravando...');
        finalTranscript = '';
    }};
    
    recognition.onresult = function(event) {{
        let interimTranscript = '';
        
        for (let i = event.resultIndex; i < event.results.length; i++) {{
            const transcript = event.results[i][0].transcript;
            
            if (event.results[i].isFinal) {{
                finalTranscript += transcript + ' ';
            }} else {{
                interimTranscript += transcript;
            }}
        }}
        
        const fullText = finalTranscript + interimTranscript;
        if (fullText.trim()) {{
            window.webkit.messageHandlers.transcript.postMessage(fullText);
        }}
    }};
    
    recognition.onerror = function(event) {{
        console.error('Erro:', event.error);
        
        if (event.error === 'network' || event.error === 'no-speech') {{
            setTimeout(() => {{
                if (recognition) {{
                    try {{ recognition.start(); }} catch (e) {{}}
                }}
            }}, 1000);
        }}
    }};
    
    try {{
        recognition.start();
    }} catch (e) {{
        console.error('Erro ao iniciar:', e);
    }}
}}

function stopRecognition() {{
    if (recognition) {{
        recognition.stop();
        recognition = null;
    }}
}}

window.addEventListener('load', initRecognition);
</script>
</body>
</html>
        "#,
            language
        )
    }
}

impl Drop for SpeechRecognition {
    fn drop(&mut self) {
        self.stop();
    }
}
