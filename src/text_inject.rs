use std::process::Command;

pub async fn inject_text(text: String) -> Result<(), String> {
    // Verificar se ydotool está disponível
    if Command::new("which").arg("ydotool").output().map(|o| !o.status.success()).unwrap_or(true) {
        return Err("ydotool não encontrado. Instale com: sudo pacman -S ydotool".to_string());
    }
    
    // Verificar se ydotoold está rodando
    if !is_ydotoold_running() {
        // Tentar iniciar
        let _ = Command::new("ydotoold").spawn();
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    // Injetar texto
    let output = Command::new("ydotool")
        .arg("type")
        .arg("--")
        .arg(&text)
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(format!("ydotool falhou: {}", stderr))
            }
        }
        Err(e) => Err(format!("Erro ao executar ydotool: {}", e)),
    }
}

fn is_ydotoold_running() -> bool {
    Command::new("pgrep")
        .arg("-x")
        .arg("ydotoold")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
