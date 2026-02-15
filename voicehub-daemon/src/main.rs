mod hotkey;
mod inject;
mod speech;

use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    
    log::info!("🦞 VoiceHub Daemon v0.1.0");
    log::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Verificar pré-requisitos
    if !inject::check_ydotool().await {
        log::error!("❌ ydotool não encontrado!");
        log::error!("   Instale: sudo pacman -S ydotool");
        log::error!("   Inicie: sudo systemctl enable --now ydotoold");
        return Err("ydotool not available".into());
    }
    log::info!("✅ ydotool disponível");
    
    // Estado compartilhado
    let is_recording = Arc::new(Mutex::new(false));
    let current_transcript = Arc::new(Mutex::new(String::new()));
    
    // Canais de comunicação
    let (hotkey_tx, mut rx) = mpsc::unbounded_channel();
    let (transcript_tx, mut transcript_rx) = mpsc::unbounded_channel();
    
    // Inicializar Speech Recognizer
    log::info!("🎤 Inicializando Web Speech API...");
    let recognizer = speech::SpeechRecognizer::new(transcript_tx);
    
    // Aguardar recognizer ficar pronto (max 5s)
    let start = std::time::Instant::now();
    while !recognizer.is_ready() && start.elapsed().as_secs() < 5 {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    if !recognizer.is_ready() {
        log::error!("❌ Web Speech API não ficou pronto em 5 segundos");
        return Err("Speech API timeout".into());
    }
    log::info!("✅ Web Speech API pronto");
    
    log::info!("🎯 Iniciando hotkey listener (Super+H)...");
    log::info!("   Pressione Ctrl+C para sair");
    log::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Hotkey Listener (Super+H)
    let hotkey_tx_clone = hotkey_tx.clone();
    let _hotkey_handle = tokio::spawn(async move {
        hotkey::listen("Super+H", hotkey_tx_clone).await;
    });
    
    // Event Loop Principal
    loop {
        tokio::select! {
            // Eventos de hotkey (Super+H toggle)
            Some(event) = rx.recv() => {
                match event {
                    HotkeyEvent::Toggle => {
                        let mut recording = is_recording.lock().unwrap();
                        *recording = !*recording;
                        
                        if *recording {
                            log::info!("🎤 INICIANDO GRAVAÇÃO...");
                            log::info!("   Fale agora e pressione Super+H quando terminar");
                            
                            // Limpar transcrição anterior
                            let mut transcript = current_transcript.lock().unwrap();
                            *transcript = String::new();
                            drop(transcript); // Release lock
                            
                            // Iniciar Web Speech API
                            recognizer.start();
                            
                        } else {
                            log::info!("⏹️  PARANDO GRAVAÇÃO...");
                            
                            // Parar Web Speech API
                            recognizer.stop();
                            
                            // Aguardar um pouco para última transcrição chegar
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            
                            // Pegar transcrição acumulada
                            let transcript = current_transcript.lock().unwrap().clone();
                            
                            if transcript.trim().is_empty() {
                                log::warn!("⚠️  Nenhuma transcrição capturada");
                            } else {
                                log::info!("   Injetando {} caracteres...", transcript.len());
                                if let Err(e) = inject::type_text(&transcript).await {
                                    log::error!("❌ Falha ao injetar texto: {}", e);
                                } else {
                                    log::info!("✅ Texto injetado com sucesso");
                                }
                            }
                            
                            log::info!("✅ Pronto para nova gravação");
                        }
                    }
                }
            }
            
            // Transcrições do Web Speech API
            Some(text) = transcript_rx.recv() => {
                log::debug!("📝 Transcrição recebida: {}", text);
                let mut transcript = current_transcript.lock().unwrap();
                *transcript = text;
            }
            
            // Ctrl+C handling
            _ = tokio::signal::ctrl_c() => {
                log::info!("\n👋 Encerrando VoiceHub Daemon...");
                break;
            }
        }
    }
    
    log::info!("✅ Daemon encerrado com sucesso");
    Ok(())
}

#[derive(Debug, Clone)]
pub enum HotkeyEvent {
    Toggle,
}
