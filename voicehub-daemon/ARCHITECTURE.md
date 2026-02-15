# 🏗️ VoiceHub Daemon - Arquitetura Técnica

## Visão Geral

O VoiceHub Daemon é um sistema de ditado de voz global para Linux que combina:

1. **Hotkey Global Wayland-nativo** (evdev)
2. **Web Speech API** (via WebView headless)
3. **Injeção de Texto Cross-Desktop** (ydotool)

---

## Fluxo de Dados

```
┌─────────────────────────────────────────────────────────────┐
│                      VoiceHub Daemon                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐         ┌──────────────┐                 │
│  │  Hotkey      │         │   Speech     │                 │
│  │  Listener    │──┬──────│  Recognizer  │                 │
│  │ (Super+H)    │  │      │  (WebView)   │                 │
│  └──────────────┘  │      └──────────────┘                 │
│         │          │              │                         │
│         ▼          │              ▼                         │
│  ┌──────────────┐  │      ┌──────────────┐                 │
│  │   Main       │◄─┴──────│ Transcript   │                 │
│  │ Event Loop   │         │   Channel    │                 │
│  │   (tokio)    │         └──────────────┘                 │
│  └──────────────┘                                           │
│         │                                                   │
│         ▼                                                   │
│  ┌──────────────┐                                           │
│  │   Text       │                                           │
│  │  Injector    │──────────► Aplicação Ativa               │
│  │  (ydotool)   │                                           │
│  └──────────────┘                                           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Componentes

### 1. Hotkey Listener (`src/hotkey.rs`)

**Responsabilidade**: Detectar pressão de Super+H globalmente.

**Tecnologia**: `hotkey-listener` crate (acesso direto a `/dev/input/*`)

**Thread**: Separada (blocking I/O para evdev)

**Comunicação**: `mpsc::unbounded_channel` → Main Loop

```rust
// Pseudo-código
loop {
    event = handle.recv_timeout(100ms)
    if event == Pressed(idx) {
        send(HotkeyEvent::Toggle)
    }
}
```

---

### 2. Speech Recognizer (`src/speech.rs`)

**Responsabilidade**: Transcrição de voz em tempo real.

**Tecnologia**: Web Speech API via `webkit2gtk` headless

**Thread**: Separada (GTK event loop não-bloqueante)

**Comunicação**:
- **Rust → JS**: `std::sync::mpsc` para comandos (Start/Stop)
- **JS → Rust**: WebKit script message handlers para transcrições

#### Arquitetura Interna

```
┌──────────────────────────────────────────┐
│         Rust Main Thread                 │
│  ┌────────────────────────────────────┐  │
│  │  SpeechRecognizer::new()           │  │
│  │  - command_tx (Rust → GTK)         │  │
│  │  - transcript_tx (GTK → Main)      │  │
│  └────────────────────────────────────┘  │
└──────────────────────────────────────────┘
            │
            ▼
┌──────────────────────────────────────────┐
│         GTK Thread                       │
│  ┌────────────────────────────────────┐  │
│  │  WebView (headless)                │  │
│  │  - HTML + JavaScript               │  │
│  │  - webkitSpeechRecognition         │  │
│  │  - Script message handlers         │  │
│  └────────────────────────────────────┘  │
│              │                            │
│              ▼                            │
│  ┌────────────────────────────────────┐  │
│  │  glib::timeout_add_local           │  │
│  │  - Pool comandos (100ms)           │  │
│  │  - Chama startRecognition()        │  │
│  │  - Chama stopRecognition()         │  │
│  └────────────────────────────────────┘  │
└──────────────────────────────────────────┘
```

#### JavaScript Embedado

- **Continuous**: `recognition.continuous = true`
- **Interim Results**: `recognition.interimResults = true`
- **Language**: `recognition.lang = 'pt-BR'`
- **Auto-restart**: `onend` reinicia se `isRecording == true`

#### Transcrição Acumulada

Evita texto duplicado usando:
- `lastProcessedIndex` - track de qual resultado já foi processado
- `accumulatedTranscript` - acumula apenas resultados `isFinal`
- Envia `accumulated + interim` para Rust via `postMessage()`

---

### 3. Text Injector (`src/inject.rs`)

**Responsabilidade**: Injetar texto transcrito no campo ativo.

**Tecnologia**: `ydotool` via `Command::new("ydotool")`

**Thread**: Async (tokio::process)

**Wayland-safe**: ✅ Funciona em qualquer compositor

```rust
// Pseudo-código
async fn type_text(text: &str) {
    Command::new("ydotool")
        .arg("type")
        .arg(text)
        .status()
        .await?
}
```

---

### 4. Main Event Loop (`src/main.rs`)

**Responsabilidade**: Orquestrar todos os componentes.

**Runtime**: `tokio` async

**Estado Compartilhado**:
- `is_recording: Arc<Mutex<bool>>` - flag de gravação ativa
- `current_transcript: Arc<Mutex<String>>` - texto acumulado

#### Fluxo de Estados

```
IDLE ──(Super+H)──► RECORDING ──(Super+H)──► INJECTING ──► IDLE
                        │                         │
                        ▼                         ▼
                 [falar texto]           [ydotool type]
                        │
                        ▼
                [transcript_rx.recv()]
```

---

## Concorrência e Threads

### Thread 1: Main (Tokio)
- Event loop principal
- Coordena estado global
- Chama inject::type_text()

### Thread 2: Hotkey Listener
- Blocking I/O em `/dev/input/*`
- Envia eventos via `mpsc` para Main

### Thread 3: GTK (WebView)
- Headless WebView com Web Speech API
- Recebe comandos via `std::sync::mpsc`
- Envia transcrições via `tokio::sync::mpsc`

### Canais de Comunicação

| Canal | Tipo | Direção | Dados |
|-------|------|---------|-------|
| `hotkey_tx/rx` | tokio::mpsc | Hotkey → Main | `HotkeyEvent::Toggle` |
| `transcript_tx/rx` | tokio::mpsc | GTK → Main | `String` (transcrição) |
| `command_tx/rx` | std::sync::mpsc | Main → GTK | `SpeechCommand::{Start,Stop}` |

---

## Requisitos de Sistema

### Bindings Rust → Sistema

- **evdev access**: Usuário no grupo `input`
- **ydotool daemon**: `ydotoold` rodando (system ou user)
- **GTK/WebKit**: `webkit2gtk-4.1` instalado

### Permissões

```bash
# /dev/input/* (hotkey listener)
sudo usermod -aG input $USER

# ydotool socket (text injection)
sudo systemctl enable --now ydotoold
```

---

## Performance

### Binário
- **Tamanho**: ~3MB (release build)
- **Dependências runtime**: GTK, WebKit, ydotool

### Latência
- **Hotkey detection**: <10ms (evdev direto)
- **Speech recognition**: Real-time (Web Speech API streaming)
- **Text injection**: ~50-100ms (ydotool processing)

### Recursos
- **Memória**: ~30-50MB (WebView + GTK)
- **CPU idle**: <1%
- **CPU recording**: ~5-10% (Web Speech API)

---

## Limitações Conhecidas

### Web Speech API

- ✅ **Gratuito**: Sem API keys
- ❌ **Browser-dependent**: Precisa de WebKit/Chromium
- ❌ **Internet**: Algumas implementações precisam de conexão
- ⚠️ **Idiomas**: pt-BR funciona, mas qualidade varia por engine

### Wayland

- ✅ **Hotkey global**: Funciona via evdev
- ✅ **Text injection**: ydotool funciona em qualquer compositor
- ❌ **Window focus**: Não consegue detectar janela ativa (Wayland security)

### Cross-Desktop

- ✅ **COSMIC**: Funciona
- ✅ **GNOME**: Funciona
- ✅ **KDE**: Funciona
- ✅ **Sway/i3**: Funciona
- ⚠️ **TTY**: Não funciona (precisa de display server)

---

## Roadmap Futuro

### V0.2 - System Tray
- [ ] Ícone na bandeja do sistema
- [ ] Menu: Start/Stop/Settings/Quit
- [ ] Indicador visual quando gravando

### V0.3 - Configuração
- [ ] Arquivo de config TOML
- [ ] Hotkey customizável
- [ ] Idioma selecionável
- [ ] Timeout de gravação

### V0.4 - Melhorias STT
- [ ] Fallback para outros engines (Vosk, Whisper local)
- [ ] Pontuação automática inteligente
- [ ] Correção ortográfica

### V1.0 - Produção
- [ ] Instalador .deb/.rpm
- [ ] systemd user service
- [ ] Desktop entry
- [ ] Documentação completa

---

**🦞 DevSan** - Arquitetura documentada em 15/02/2026
