mod app;
mod config;
mod text_inject;
mod speech_recognition;

fn main() -> cosmic::iced::Result {
    // Inicializar GTK (necessário para WebKit)
    if let Err(e) = gtk::init() {
        eprintln!("⚠️ Aviso: Falha ao inicializar GTK: {}", e);
        eprintln!("   Speech recognition pode não funcionar corretamente.");
    }
    
    app::run()
}
