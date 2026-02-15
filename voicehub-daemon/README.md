# 🎤 VoiceHub Daemon - Ditado Global para Linux

> Daemon universal de ditado de voz com hotkey global (Super+H) para qualquer ambiente desktop Linux.

---

## ✨ Features

- ✅ **Hotkey Global**: Super+H funciona em qualquer tela, qualquer aplicativo
- ✅ **Cross-Desktop**: COSMIC, GNOME, KDE, i3, Sway, Hyprland, etc.
- ✅ **Wayland Nativo**: Usa evdev diretamente (sem X11)
- ✅ **Injeção de Texto**: Via ydotool (Wayland-friendly)
- ✅ **Leve**: Binário de ~2.8MB
- ✅ **Async**: Rust + Tokio para performance

---

## 🚀 Instalação

### Pré-requisitos

```bash
# 1. Instalar ydotool
sudo pacman -S ydotool

# 2. Iniciar serviço ydotool
sudo systemctl enable --now ydotoold

# 3. Adicionar usuário ao grupo input
sudo usermod -aG input $USER

# 4. Logout e login para aplicar grupo
```

### Compilar

```bash
cargo build --release
```

O binário será gerado em `target/release/voicehub-daemon`

---

## 📋 Uso

### Executar Daemon

```bash
# Direto
./target/release/voicehub-daemon

# Ou com logs detalhados
RUST_LOG=debug ./target/release/voicehub-daemon
```

### Como Usar

1. **Pressione Super+H** - Inicia gravação
2. **Fale naturalmente** - O texto será transcrito
3. **Pressione Super+H novamente** - Para gravação e injeta texto

O texto transcrito será automaticamente inserido no campo de texto ativo (onde o cursor está).

---

## 🛠️ Tecnologias

- **hotkey-listener** 0.3 - Wayland-native global hotkeys via evdev
- **ydotool** - Text injection (Wayland)
- **tokio** - Async runtime
- **tray-icon** - System tray (cross-desktop)

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
