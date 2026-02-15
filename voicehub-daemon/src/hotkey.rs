use hotkey_listener::{parse_hotkey, HotkeyListenerBuilder, HotkeyEvent as HLEvent};
use tokio::sync::mpsc::UnboundedSender;
use std::time::Duration;
use crate::HotkeyEvent;

pub async fn listen(hotkey_str: &str, tx: UnboundedSender<HotkeyEvent>) {
    log::info!("🎯 Iniciando hotkey listener para: {}", hotkey_str);
    
    let hotkey = match parse_hotkey(hotkey_str) {
        Ok(h) => h,
        Err(e) => {
            log::error!("❌ Falha ao parsear hotkey '{}': {:?}", hotkey_str, e);
            return;
        }
    };
    
    let listener = match HotkeyListenerBuilder::new()
        .add_hotkey(hotkey)
        .build()
    {
        Ok(l) => l,
        Err(e) => {
            log::error!("❌ Falha ao criar hotkey listener: {:?}", e);
            log::error!("   Certifique-se de que você está no grupo 'input':");
            log::error!("   sudo usermod -aG input $USER");
            return;
        }
    };
    
    // Iniciar o listener
    let handle = match listener.start() {
        Ok(h) => h,
        Err(e) => {
            log::error!("❌ Falha ao iniciar hotkey listener: {:?}", e);
            return;
        }
    };
    
    log::info!("✅ Hotkey listener ativo: {}", hotkey_str);
    log::info!("   Pressione {} para começar/parar ditado", hotkey_str);
    
    loop {
        match handle.recv_timeout(Duration::from_millis(100)) {
            Ok(HLEvent::Pressed(_idx)) => {
                log::debug!("🎹 Hotkey pressionado");
                let _ = tx.send(HotkeyEvent::Toggle);
            }
            Ok(HLEvent::Released(_idx)) => {
                // Ignorar release para evitar duplos disparos
                log::trace!("🎹 Hotkey liberado");
            }
            Err(_) => {
                // Timeout - normal, continuar loop
                tokio::task::yield_now().await;
            }
        }
    }
}
