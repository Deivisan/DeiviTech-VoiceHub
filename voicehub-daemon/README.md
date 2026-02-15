# 🎤 VoiceHub Daemon - Ditado Global para Linux

> Daemon universal de ditado de voz com hotkey global (Super+H) para qualquer ambiente desktop Linux.

---

## ✨ Features

- ✅ **Hotkey Global**: Super+H funciona em qualquer tela, qualquer aplicativo
- ✅ **Web Speech API**: Reconhecimento de voz 100% gratuito (sem API keys)
- ✅ **Cross-Desktop**: COSMIC, GNOME, KDE, i3, Sway, Hyprland, etc.
- ✅ **Wayland Nativo**: Usa evdev diretamente (sem X11)
- ✅ **Injeção de Texto**: Via ydotool (Wayland-friendly)
- ✅ **Português BR**: Suporte nativo para português brasileiro
- ✅ **Leve**: Binário de ~3MB
- ✅ **Async**: Rust + Tokio para performance

---

## 🚀 Instalação

### Pré-requisitos

```bash
# 1. Instalar ydotool
sudo pacman -S ydotool

# 2. Instalar webkit2gtk (para Web Speech API)
sudo pacman -S webkit2gtk-4.1

# 3. Iniciar serviço ydotool
sudo systemctl enable --now ydotoold

# 4. Adicionar usuário ao grupo input
sudo usermod -aG input $USER

# 5. Logout e login para aplicar grupo
```

### Compilar

```bash
cargo build --release
# OU com bun
bun run --bun cargo build --release
```

O binário será gerado em `target/release/voicehub-daemon`

---

## 📋 Uso

### Teste Rápido

```bash
# Rodar script de teste (recomendado)
./test-daemon.sh
```

### Executar Daemon

```bash
# Direto
./target/release/voicehub-daemon

# Ou com logs detalhados
RUST_LOG=debug ./target/release/voicehub-daemon
```

### Como Usar

1. **Abra um editor de texto** (gedit, kate, mousepad, VSCode, etc.)
2. **Clique no campo de texto** para focar
3. **Pressione Super+H** - Inicia gravação de voz
4. **Fale naturalmente em português** - O texto será transcrito em tempo real
5. **Pressione Super+H novamente** - Para gravação e injeta texto no campo ativo

O texto transcrito será automaticamente inserido onde o cursor está.

---

## 🛠️ Tecnologias

- **hotkey-listener** 0.3 - Wayland-native global hotkeys via evdev
- **webkit2gtk** 2.0 - Headless WebView para Web Speech API
- **gtk** 0.18 - GTK bindings para Rust
- **ydotool** - Text injection (Wayland)
- **tokio** - Async runtime
- **tray-icon** 0.18 - System tray (cross-desktop) - *futuro*

---

## 🐛 Troubleshooting

### "Permission denied" ao acessar /dev/input

Certifique-se de que você está no grupo `input`:

```bash
groups $USER | grep input
```

Se não estiver, adicione:

```bash
sudo usermod -aG input $USER
# Logout e login
```

### ydotool não funciona

Verifique se o serviço está rodando:

```bash
systemctl --user status ydotoold
```

Se não estiver:

```bash
sudo systemctl enable --now ydotoold
```

### Hotkey não detectado

Verifique os logs:

```bash
RUST_LOG=debug ./target/release/voicehub-daemon
```

Pressione Super+H e veja se aparecem eventos no log.

---

## 📌 Status Atual

- ✅ Hotkey listener (Super+H)
- ✅ Text injection (ydotool)
- ✅ Event loop async
- 🚧 Web Speech API integration (próximo)
- 🚧 System tray icon (próximo)
- 🚧 Configuração (idioma, hotkey customizável)

---

## 🎯 Próximos Passos

1. **Integrar Web Speech API** via webkit2gtk headless
2. **System Tray** com ícone e menu
3. **Configuração** - Arquivo TOML para customizar hotkey e idioma
4. **Instalador** - .deb e .rpm para distros

---

**"Mãos livres, mente focada. Ditado profissional para Linux."** 🦞
