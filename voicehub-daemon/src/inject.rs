use tokio::process::Command;
use std::error::Error;

pub async fn type_text(text: &str) -> Result<(), Box<dyn Error>> {
    log::info!("📤 Injetando texto: {} caracteres", text.len());
    
    // Verificar se ydotool está disponível
    let check = Command::new("which")
        .arg("ydotool")
        .output()
        .await?;
    
    if !check.status.success() {
        log::error!("❌ ydotool não encontrado!");
        log::error!("   Instale: sudo pacman -S ydotool");
        log::error!("   Inicie: sudo systemctl enable --now ydotoold");
        return Err("ydotool not found".into());
    }
    
    // Injetar texto
    let output = Command::new("ydotool")
        .args(["type", "--", text])
        .output()
        .await?;
    
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        log::error!("❌ ydotool falhou: {}", err);
        return Err(format!("ydotool failed: {}", err).into());
    }
    
    log::info!("✅ Texto injetado com sucesso!");
    Ok(())
}

/// Testa se ydotool está funcionando
pub async fn check_ydotool() -> bool {
    match Command::new("which")
        .arg("ydotool")
        .output()
        .await
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
