# 🎤 VoiceHub COSMIC Applet - Proposta de Reimaginação

> Transformar o VoiceHub de app Tauri em um **applet nativo COSMIC** usando `libcosmic`

---

## 🎯 Visão Geral

### Por que um Applet COSMIC?

| Aspecto | App Tauri Atual | Applet COSMIC |
|---------|----------------|---------------|
| **Integração** | System Tray genérico | Panel applet nativo |
| **Hotkeys** | Super+H (global) | Integrado ao COSMIC Settings |
| **UI** | WebView (HTML/CSS/JS) | Iced (Rust nativo) |
| **Memória** | ~180MB (WebView) | ~20MB (Rust nativo) |
| **Tema** | CSS customizado | Segue tema COSMIC automaticamente |
| **Wayland** | Via Tauri | Nativo e otimizado |

---

## 🏗️ Arquitetura Proposta

```
┌─────────────────────────────────────────────────────────────────┐
│                    COSMIC PANEL                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  🎤 VoiceHub Applet                                     │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐                 │   │
│  │  │   🎙️   │  │  🔴    │  │   ⚙️   │  ← Icon Buttons   │   │
│  │  │ Gravar │  │ Parar   │  │ Config  │                 │   │
│  │  └─────────┘  └─────────┘  └─────────┘                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│                              ▼                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Popup/Drawer                                           │   │
│  │  ┌─────────────────────────────────────────────────┐   │   │
│  │  │  📝 Transcrição ao Vivo                        │   │   │
│  │  │                                                 │   │   │
│  │  │  [Texto transcrito aparece aqui...]            │   │   │
│  │  │                                                 │   │   │
│  │  │  ⏱️ 00:45  │  🌍 PT-BR  │  23 palavras        │   │   │
│  │  └─────────────────────────────────────────────────┘   │   │
│  │  ┌─────────────────────────────────────────────────┐   │   │
│  │  │  🔊 Visualizador de Áudio                      │   │   │
│  │  │  ▁▃▅▇███▇▅▃▁▁▃▅▇▆▄▃▁▁▃▅▇                      │   │   │
│  │  └─────────────────────────────────────────────────┘   │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐               │   │
│  │  │ 📋 Copiar│ │ 🗑️ Limpar│ │ 📤 Injetar│               │   │
│  │  └──────────┘ └──────────┘ └──────────┘               │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  Backend (Rust)                                                 │
│  ├── Speech Recognition (Web Speech API via WebKit)            │
│  ├── Audio Capture (pipewire/pulse)                             │
│  ├── Text Injection (ydotool/zbus)                              │
│  └── Config (cosmic-config)                                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📦 Estrutura do Projeto

```
cosmic-applet-voicehub/
├── Cargo.toml              # Dependências libcosmic
├── src/
│   ├── main.rs             # Entry point: cosmic::applet::run()
│   ├── app.rs              # Application trait impl
│   ├── speech.rs           # Web Speech API integration
│   ├── audio.rs            # Audio capture (pipewire)
│   ├── config.rs           # cosmic-config
│   └── i18n/               # Traduções (Fluent)
├── resources/
│   ├── icons/              # Ícones SVG
│   └── style.css           # Estilos customizados
└── justfile                # Build scripts
```

---

## 💻 Código de Exemplo

### `Cargo.toml`

```toml
[package]
name = "cosmic-applet-voicehub"
version = "0.1.0"
edition = "2021"

[dependencies]
libcosmic = { git = "https://github.com/pop-os/libcosmic.git", features = ["applet", "wayland", "tokio"] }
cosmic-config = { git = "https://github.com/pop-os/libcosmic.git" }

# Speech Recognition
webkit6 = { version = "0.1", optional = true }  # Web Speech API via WebKitGTK

# Audio
pipewire = "0.8"  # ou pulsectl-rs

# Text Injection
zbus = "4.0"  # D-Bus para ydotool via zbus

# Async
tokio = { version = "1.0", features = ["rt", "sync"] }

# Utils
serde = { version = "1.0", features = ["derive"] }
i18n-embed = { version = "0.15", features = ["fluent"] }
```

### `src/main.rs`

```rust
use cosmic::applet;
use cosmic::iced;
use cosmic::iced::wayland::popup;

mod app;
mod speech;
mod audio;
mod config;

fn main() -> cosmic::iced::Result {
    // Inicializar applet COSMIC
    cosmic::applet::run::<VoiceHubApplet>(())
}

pub struct VoiceHubApplet {
    core: cosmic::app::Core,
    config: config::VoiceHubConfig,
    speech_engine: speech::SpeechEngine,
    audio_capture: audio::AudioCapture,
    is_recording: bool,
    transcript: String,
    popup: Option<cosmic::iced::window::Id>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleRecording,
    TranscriptUpdate(String),
    CopyToClipboard,
    ClearTranscript,
    InjectText,
    TogglePopup,
    PopupClosed,
    ConfigUpdated(config::VoiceHubConfig),
}

impl cosmic::app::Application for VoiceHubApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.deivisan.voicehub";

    fn init(core: cosmic::app::Core, _flags: ()) -> (Self, cosmic::app::Task<Message>) {
        let config = config::VoiceHubConfig::load();
        
        (
            VoiceHubApplet {
                core,
                config,
                speech_engine: speech::SpeechEngine::new(),
                audio_capture: audio::AudioCapture::new(),
                is_recording: false,
                transcript: String::new(),
                popup: None,
            },
            cosmic::app::Task::none(),
        )
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn update(&mut self, message: Message) -> cosmic::app::Task<Message> {
        match message {
            Message::ToggleRecording => {
                self.is_recording = !self.is_recording;
                
                if self.is_recording {
                    self.speech_engine.start();
                    self.audio_capture.start();
                } else {
                    self.speech_engine.stop();
                    self.audio_capture.stop();
                }
            }
            
            Message::TranscriptUpdate(text) => {
                self.transcript.push_str(&text);
            }
            
            Message::CopyToClipboard => {
                return cosmic::app::clipboard::write(self.transcript.clone());
            }
            
            Message::ClearTranscript => {
                self.transcript.clear();
            }
            
            Message::InjectText => {
                // Injetar via ydotool/zbus
                return cosmic::app::Task::perform(
                    inject_text(self.transcript.clone()),
                    |_| Message::ToggleRecording,
                );
            }
            
            Message::TogglePopup => {
                if let Some(popup) = self.popup {
                    return cosmic::app::window::close(popup);
                } else {
                    let new_popup = cosmic::iced::window::Id::unique();
                    self.popup = Some(new_popup);
                    
                    return cosmic::app::Task::batch([
                        cosmic::app::window::get_iced_id(new_popup),
                        cosmic::app::window::set_title(new_popup, "VoiceHub".to_string()),
                    ]);
                }
            }
            
            Message::PopupClosed => {
                self.popup = None;
            }
            
            Message::ConfigUpdated(config) => {
                self.config = config;
            }
        }
        
        cosmic::app::Task::none()
    }

    fn view(&self) -> cosmic::Element<Message> {
        // View do applet no panel (compacto)
        let button = cosmic::widget::button(
            cosmic::widget::icon::from_name(
                if self.is_recording { "audio-input-microphone-symbolic" } else { "audio-input-microphone-muted-symbolic" }
            )
            .size(16)
        )
        .on_press(Message::ToggleRecording)
        .padding(8)
        .style(cosmic::theme::Button::AppletIcon);

        button.into()
    }

    fn view_window(&self, _id: cosmic::iced::window::Id) -> cosmic::Element<Message> {
        // View do popup (expandido)
        cosmic::widget::column()
            .push(cosmic::widget::text("🎤 VoiceHub").size(18).bold())
            .push(cosmic::widget::divider::horizontal::default())
            .push(
                cosmic::widget::scrollable(
                    cosmic::widget::text(&self.transcript).size(14)
                )
                .height(200)
            )
            .push(
                cosmic::widget::row()
                    .push(cosmic::widget::button("📋 Copiar").on_press(Message::CopyToClipboard))
                    .push(cosmic::widget::button("🗑️ Limpar").on_press(Message::ClearTranscript))
                    .push(cosmic::widget::button("📤 Injetar").on_press(Message::InjectText))
                    .spacing(8)
            )
            .padding(16)
            .spacing(12)
            .into()
    }
}

async fn inject_text(text: String) {
    // Usar zbus para comunicar com ydotool via D-Bus
    // ou executar ydotool type diretamente
    let _ = std::process::Command::new("ydotool")
        .args(&["type", "--", &text])
        .output();
}
```

### `src/config.rs`

```rust
use cosmic_config::{Config, ConfigEntry, CosmicConfigEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceHubConfig {
    pub language: String,
    pub auto_punctuation: bool,
    pub auto_inject: bool,
    pub global_hotkey: String,
}

impl Default for VoiceHubConfig {
    fn default() -> Self {
        Self {
            language: "pt-BR".to_string(),
            auto_punctuation: true,
            auto_inject: false,
            global_hotkey: "Super+H".to_string(),
        }
    }
}

impl CosmicConfigEntry for VoiceHubConfig {
    const VERSION: u64 = 1;
    
    fn load() -> Self {
        Config::new("com.deivisan.voicehub", Self::VERSION)
            .map(|config| config.get("config").unwrap_or_default())
            .unwrap_or_default()
    }
    
    fn save(&self) {
        if let Ok(config) = Config::new("com.deivisan.voicehub", Self::VERSION) {
            let _ = config.set("config", self);
        }
    }
}
```

---

## 🔧 Recursos Avançados do Applet

### 1. Integração com COSMIC Settings

```rust
// Configuração aparece automaticamente em Configurações do Sistema > Aplicativos
impl cosmic::app::Application for VoiceHubApplet {
    fn settings(&self) -> Option<cosmic::app::Settings> {
        Some(cosmic::app::Settings {
            // Aparece no COSMIC Settings
            show_in_settings: true,
            settings_title: "VoiceHub".to_string(),
            ..Default::default()
        })
    }
}
```

### 2. Tema Dinâmico

```rust
// Automaticamente segue o tema COSMIC (claro/escuro/accento)
fn view(&self) -> cosmic::Element<Message> {
    let theme = self.core().theme();
    let accent = theme.accent_color();
    
    // Widgets usam o tema automaticamente
    cosmic::widget::button("Gravar")
        .style(cosmic::theme::Button::Suggested)  // Usa cor de destaque do tema
}
```

### 3. Notificações Nativas

```rust
use cosmic::app::notification;

fn show_notification(&self, title: &str, body: &str) {
    notification::show(
        notification::Notification::new()
            .summary(title)
            .body(body)
            .icon("audio-input-microphone-symbolic")
    );
}
```

---

## 🚀 Roadmap de Migração

### Fase 1: Setup (Semana 1)
- [ ] Criar projeto com `cargo generate cosmic-utils/cosmic-applet-template`
- [ ] Configurar `Cargo.toml` com dependências
- [ ] Setup inicial do applet (hello world no panel)

### Fase 2: UI Básica (Semana 2)
- [ ] Implementar view compacta (ícone no panel)
- [ ] Implementar popup com transcrição
- [ ] Integrar com tema COSMIC

### Fase 3: Speech Recognition (Semana 3)
- [ ] Integrar Web Speech API via WebKitGTK
- [ ] Ou usar whisper-rs (offline, 100% Rust)
- [ ] Implementar controles gravar/parar

### Fase 4: Funcionalidades (Semana 4)
- [ ] Copiar para clipboard
- [ ] Injeção via ydotool/zbus
- [ ] Persistência (cosmic-config)

### Fase 5: Polish (Semana 5)
- [ ] i18n (PT, EN, ES)
- [ ] Visualizador de áudio
- [ ] Atalhos globais integrados ao COSMIC

---

## 📊 Comparação: Tauri vs COSMIC Applet

| Feature | Tauri | Applet COSMIC | Vencedor |
|---------|-------|---------------|----------|
| **Tamanho Binário** | 15MB | ~5MB | 🏆 Applet |
| **Memória RAM** | ~180MB | ~20MB | 🏆 Applet |
| **Startup** | ~2s | ~0.5s | 🏆 Applet |
| **Integração COSMIC** | Tray genérico | Panel nativo | 🏆 Applet |
| **Tema** | CSS manual | Automático | 🏆 Applet |
| **Multi-DE** | ✅ Sim | ⚠️ COSMIC only | 🏆 Tauri |
| **Desenvolvimento** | Web (familiar) | Rust/Iced (novo) | 🏆 Tauri |
| **Hot Reload** | ✅ Sim | ⚠️ Limitado | 🏆 Tauri |

**Veredito**: Applet é melhor para uso exclusivo no COSMIC. Tauri é melhor para app universal.

---

## 🎁 Benefícios do Applet

1. **🎨 Tema Automático**: Segue o tema COSMIC (claro/escuro/cores)
2. **⚡ Performance**: Binário menor, memória mínima
3. **🔧 Integração Nativa**: Aparece no COSMIC Settings
4. **🌐 Wayland Nativo**: Sem camadas de compatibilidade
5. **📱 UX Fluida**: Transições e animações nativas do COSMIC

---

## 🚧 Desafios

1. **Web Speech API**: Precisa de WebKitGTK ou implementar whisper-rs
2. **Curva de Aprendizado**: Iced é diferente de React
3. **Documentação**: libcosmic ainda está em desenvolvimento ativo
4. **Debugging**: Menos ferramentas que desenvolvimento web

---

## 💡 Recomendação

**Para uso pessoal no COSMIC**: Migração para applet vale a pena pela integração nativa e performance.

**Para distribuição pública**: Manter versão Tauri para compatibilidade multi-DE.

**Solução híbrida**: Manter core em Rust, criar dois frontends (Tauri para universal, Applet para COSMIC).

---

> **"O futuro do VoiceHub no COSMIC é nativo!"** 🚀
