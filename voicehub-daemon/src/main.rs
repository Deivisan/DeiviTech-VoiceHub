mod hotkey;
mod inject;

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
    
    // Canal de comunicação
    let (tx, mut rx) = mpsc::unbounded_channel();
    
    log::info!("🎯 Iniciando hotkey listener (Super+H)...");
    log::info!("   Pressione Ctrl+C para sair");
    log::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Hotkey Listener (Super+H)
    let tx_clone = tx.clone();
    let _hotkey_handle = tokio::spawn(async move {
        hotkey::listen("Super+H", tx_clone).await;
    });
    
    // Event Loop Principal
    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                match event {
                    HotkeyEvent::Toggle => {
                        let mut recording = is_recording.lock().unwrap();
                        *recording = !*recording;
                        
                        if *recording {
                            log::info!("🎤 INICIANDO GRAVAÇÃO...");
                            log::info!("   Fale agora e pressione Super+H quando terminar");
                            
                            // TODO: Iniciar Web Speech API
                            // Por enquanto, simular com texto de teste
                            let mut transcript = current_transcript.lock().unwrap();
                            *transcript = String::new();
                            
                        } else {
                            log::info!("⏹️  PARANDO GRAVAÇÃO...");
                            
                            // Pegar transcrição acumulada
                            let transcript = current_transcript.lock().unwrap().clone();
                            
                            if transcript.is_empty() {
                                log::warn!("⚠️  Nenhuma transcrição para injetar");
                                // Testar com texto de exemplo
                                log::info!("   Injetando texto de teste...");
                                if let Err(e) = inject::type_text("Teste VoiceHub funcionando! 🎤").await {
                                    log::error!("❌ Falha ao injetar texto: {}", e);
                                }
                            } else {
                                log::info!("   Injetando {} caracteres...", transcript.len());
                                if let Err(e) = inject::type_text(&transcript).await {
                                    log::error!("❌ Falha ao injetar texto: {}", e);
                                }
                            }
                            
                            log::info!("✅ Pronto para nova gravação");
                        }
                    }
                    HotkeyEvent::Transcript(text) => {
                        log::debug!("📝 Transcrição recebida: {}", text);
                        let mut transcript = current_transcript.lock().unwrap();
                        *transcript = text;
                    }
                }
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
    Transcript(String),
}
