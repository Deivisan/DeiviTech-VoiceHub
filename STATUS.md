# 🎤 VoiceHub - Estado Atual

> **Data:** 15/02/2026
> **Status:** 🟢 DAEMON FUNCIONAL | 🟡 EM DESENVOLVIMENTO (WebView)

---

## 🏗️ Estrutura Atual

```
DeiviTech-VoiceHub/
├── voicehub-daemon/      # 🆕 ✅ DAEMON GLOBAL (NOVO!)
│   ├── src/
│   │   ├── main.rs       # Entry point + event loop
│   │   ├── hotkey.rs     # Hotkey listener (Super+H)
│   │   └── inject.rs     # ydotool text injection
│   └── target/release/voicehub-daemon  # Binário ~2.8MB
│
├── legacy/               # ✅ Servidor web Bun (FUNCIONANDO)
│   ├── src/server.ts    # Servidor HTTP na porta 5001
│   └── src/public/      # Interface HTML/CSS/JS (bugs corrigidos)
│
└── src/                 # ❌ Applet COSMIC (NÃO FUNCIONANDO)
    ├── main.rs          # Entry point Rust
    └── app.rs           # Applet de desktop
```

---

## ✅ Funcionando

### 1. VoiceHub Daemon (NOVO!) 🦞

**O daemon global de ditado está funcionando!**

- **Hotkey Global:** Super+H (funciona em qualquer tela)
- **Wayland Nativo:** Via evdev (sem X11)
- **Cross-Desktop:** COSMIC, GNOME, KDE, i3, Sway, etc.
- **Text Injection:** Via ydotool
- **Tamanho:** ~2.8MB binário

**Como usar:**
```bash
cd ~/Projetos/DeiviTech-VoiceHub/voicehub-daemon
./target/release/voicehub-daemon

# Pressione Super+H para iniciar/parar gravação
# Texto será injetado automaticamente
```

**Status Atual:**
- ✅ Hotkey listener funcionando
- ✅ Event loop async
- ✅ Text injection via ydotool
- 🚧 Web Speech API (próximo passo)
- 🚧 System tray icon

### 2. Servidor Web (legacy)

- **Comando:** `voicehub start`
- **URL:** http://localhost:5001
- **Stack:** Bun + Web Speech API
- **Navegador:** Chrome/Edge (suporte completo)
- **Bugs Corrigidos:**
  - ✅ Texto editado não "ressuscita" mais
  - ✅ Sessões de fala separadas visualmente

```bash
voicehub start   # Iniciar servidor
voicehub stop    # Parar servidor
voicehub status  # Ver status
voicehub log     # Ver logs
```

---

## ❌ Não Funcionando

### Applet COSMIC Desktop
- **Problema:** Compila mas não aparece no painel
- **Stack:** Rust + libcosmic
- **Status:** Baixa prioridade (daemon é melhor solução)

**Para testar manualmente:**
```bash
cd ~/Projetos/DeiviTech-VoiceHub
cargo run --release
```

---

## 📋 Próximos Passos

### Daemon (Prioridade Alta)
1. [ ] **Integrar Web Speech API** - webkit2gtk headless
2. [ ] **System Tray** - Ícone e menu
3. [ ] **Configuração** - Arquivo TOML (idioma, hotkey)
4. [ ] **Instalador** - .deb/.rpm

### Interface Web (Manutenção)
1. [x] ~~Corrigir bug de ressurreição de texto~~ ✅
2. [x] ~~Separação de sessões de fala~~ ✅
3. [ ] Melhorar UI/UX
4. [ ] Adicionar mais idiomas

### Documentação
1. [ ] Tutorial de instalação completo
2. [ ] Vídeo demo
3. [ ] Guia de troubleshooting

---

## 📌 Notas

- **Daemon é a solução principal agora** - Funciona globalmente, não precisa de navegador
- O servidor web continua funcionando para quem preferir interface visual
- O applet COSMIC foi colocado em segundo plano (daemon é mais universal)
- Web Speech API requer HTTPS ou localhost (não funciona em http://IP)
