# 🎤 DeiviTech VoiceHub

Sistema profissional de ditado de voz em tempo real para Linux, usando **Web Speech API** (100% gratuito e local).

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Bun](https://img.shields.io/badge/runtime-Bun-yellow.svg)
![Platform](https://img.shields.io/badge/platform-Linux-green.svg)

---

## ✨ Features

### Web App (Fase 1 - ✅ Completa)
- ✅ **Transcrição em tempo real** com Web Speech API (Chrome/Edge)
- ✅ **100% gratuito** - sem API keys, sem limites de taxa
- ✅ **Pontuação automática** (vírgulas, pontos, interrogações)
- ✅ **Multi-idiomas** (8 idiomas suportados)
- ✅ **Interface minimalista** e responsiva (desktop + mobile)
- ✅ **Auto-save** de sessões (localStorage)
- ✅ **Atalhos de teclado** para workflow ágil
- ✅ **Visualizador de áudio** em tempo real
- ✅ **Zero configuração** - funciona out-of-the-box

### Desktop App (Fase 2 - ✅ Completa)
- ✅ **Tauri Desktop App** com WebView nativo
- ✅ **System Tray** com ícone e menu (mostrar/ocultar)
- ✅ **Comandos Tauri** para injeção de texto via `ydotool`
- ✅ **Binário nativo** (~10MB vs 200MB+ do Electron)

### Desktop App (Fase 3 - ✅ Completa)
- ✅ **Global Hotkeys** - `Super+H` inicia/para gravação de qualquer lugar
- ✅ **Build de Produção** - Binário nativo 15MB + instaladores .deb e .rpm
- ⏳ **Auto-start** com systemd (implementação futura)

---

## 🚀 Instalação Rápida

### Opção 1: Instaladores Pré-Compilados (.deb / .rpm)

**Baixe a versão mais recente:**
https://github.com/Deivisan/DeiviTech-VoiceHub/releases/latest

#### Debian/Ubuntu (.deb)
```bash
# Baixe o arquivo .deb da release
wget https://github.com/Deivisan/DeiviTech-VoiceHub/releases/download/v0.0.1-pre-alpha/DeiviTech_VoiceHub_0.0.1-pre-alpha_amd64.deb

# Instale
sudo dpkg -i DeiviTech_VoiceHub_0.0.1-pre-alpha_amd64.deb

# Instale dependências faltantes (se houver)
sudo apt-get install -f
```

#### Fedora/RHEL (.rpm)
```bash
# Baixe o arquivo .rpm da release
wget https://github.com/Deivisan/DeiviTech-VoiceHub/releases/download/v0.0.1-pre-alpha/DeiviTech_VoiceHub-0.0.1-pre-alpha-1.x86_64.rpm

# Instale
sudo rpm -i DeiviTech_VoiceHub-0.0.1-pre-alpha-1.x86_64.rpm
```

#### Arch Linux
```bash
# Extraia o .deb e instale manualmente (ou converta para .pkg.tar.zst)
# OU compile do código-fonte (veja Opção 2)
```

**Importante**: Após instalar, adicione seu usuário ao grupo `input` para ydotool funcionar:
```bash
sudo usermod -aG input $USER
# Faça logout e login novamente
```

### Opção 2: Compilar do Código-Fonte

### Requisitos

- **Bun** 1.0+ (runtime JavaScript ultra-rápido)
- **Rust** 1.77+ (para Tauri desktop app)
- **Chrome** ou **Edge** (Web Speech API)
- **Linux** (Arch, Ubuntu, Fedora, etc.)
- **ydotool** (para injeção de texto no desktop app)

### Instalar Bun (se não tiver)

```bash
curl -fsSL https://bun.sh/install | bash
```

### Instalar Rust (se não tiver)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Instalar dependências do sistema (Arch Linux)

```bash
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator ydotool

# Adicionar usuário ao grupo input (necessário para ydotool)
sudo usermod -aG input $USER
# Faça logout e login novamente para aplicar
```

### Clonar e Rodar

#### Web App (Desenvolvimento)

```bash
git clone https://github.com/deivisan/DeiviTech-VoiceHub.git
cd DeiviTech-VoiceHub
bun run dev
```

Abra [http://localhost:5001](http://localhost:5001) no Chrome/Edge.

#### Desktop App (Tauri)

```bash
cd DeiviTech-VoiceHub
cargo tauri dev
```

Isso abrirá o app desktop nativo com system tray integrado.

---

## 🎯 Como Usar

1. **Clique em "GRAVAR"** (ou pressione `Ctrl+Enter`)
2. **Fale naturalmente** - o texto aparece em tempo real
3. **Clique em "Parar"** quando terminar
4. **Copiar** (`Ctrl+Shift+C`) ou **Limpar** (`Ctrl+Shift+X`)

### Atalhos de Teclado

| Atalho | Ação |
|--------|------|
| `Ctrl/Cmd + Enter` | Iniciar/Parar gravação (web app) |
| `Super + H` | Iniciar/Parar gravação (desktop app - global) |
| `Ctrl/Cmd + Shift + C` | Copiar texto |
| `Ctrl/Cmd + Shift + X` | Limpar editor |

---

## ⌨️ Global Hotkeys (Desktop App)

O app desktop suporta **atalhos globais** que funcionam mesmo quando a janela está minimizada ou em segundo plano.

### Hotkey Padrão

| Atalho | Ação | Escopo |
|--------|------|--------|
| **`Super + H`** | Iniciar/Parar gravação | Sistema inteiro (funciona em qualquer janela) |

**Super** = Tecla Windows/Meta (⊞ no teclado)

### Como Funciona

1. Pressione `Super+H` de **qualquer janela ativa** do sistema
2. O VoiceHub inicia a gravação em background
3. O texto transcrito fica aguardando na janela do app
4. Pressione `Super+H` novamente para parar

### Arquitetura Técnica

```rust
// Backend Rust (src-tauri/src/lib.rs)
use tauri_plugin_global_shortcut::GlobalShortcutExt;

app.global_shortcut().register("Super+H")?;
app.handle().plugin(
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(|app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                // Emit event para frontend
                let _ = app.emit("toggle-recording", ());
            }
        })
        .build(),
)?;
```

```javascript
// Frontend (src/public/app.js)
if (window.__TAURI__) {
    const { listen } = window.__TAURI__.event;
    
    listen('toggle-recording', () => {
        if (this.isRecording) {
            this.stopRecording();
        } else {
            this.startRecording();
        }
    });
}
```

### Customizar Hotkey (Futuro)

Em versões futuras, você poderá configurar hotkeys customizados via settings (ex: `Ctrl+Alt+V`, `Super+Shift+R`, etc.).

---

## 🌍 Idiomas Suportados

- 🇧🇷 Português (Brasil)
- 🇺🇸 English (US)
- 🇪🇸 Español
- 🇫🇷 Français
- 🇩🇪 Deutsch
- 🇮🇹 Italiano
- 🇯🇵 日本語
- 🇨🇳 中文 (简体)

---

## 📂 Estrutura do Projeto

```
DeiviTech-VoiceHub/
├── src/
│   ├── public/
│   │   ├── index.html    # Interface web (497 linhas)
│   │   └── app.js        # Lógica Web Speech API (488 linhas)
│   ├── server.ts         # Servidor Bun HTTP (158 linhas)
│   └── desktop/          # (Reservado para futuras extensões)
├── src-tauri/            # Desktop app Tauri
│   ├── src/
│   │   ├── lib.rs        # Lógica principal (system tray, comandos)
│   │   └── main.rs       # Entry point
│   ├── icons/            # Ícones do app (gerados automaticamente)
│   ├── Cargo.toml        # Dependências Rust
│   └── tauri.conf.json   # Configuração Tauri
├── scripts/              # Scripts de instalação
├── docs/                 # Documentação
├── package.json
└── README.md
```

---

## 🛠️ Scripts Disponíveis

### Web App

```bash
bun run dev      # Iniciar servidor de desenvolvimento
bun run start    # Iniciar servidor de produção
```

### Desktop App (Tauri)

```bash
cargo tauri dev        # Modo desenvolvimento (hot-reload)
cargo tauri build      # Build de produção (gera binário + instaladores)
cargo tauri icon       # Regenerar ícones do app
```

---

## 🐛 Bug Fixes desta Versão

### ✅ Corrigido: Texto Repetindo Infinitamente

**Problema**: Web Speech API estava re-processando todos os resultados a cada evento `onresult`, causando repetição infinita.

**Solução**: Implementado `lastProcessedIndex` para rastrear resultados já processados.

```javascript
// ANTES (bugado)
recognition.onresult = (event) => {
    for (let i = event.resultIndex; i < event.results.length; i++) {
        // Re-processa TUDO toda vez!
    }
};

// DEPOIS (corrigido)
this.lastProcessedIndex = 0;
recognition.onresult = (event) => {
    for (let i = this.lastProcessedIndex; i < event.results.length; i++) {
        if (event.results[i].isFinal) {
            this.lastProcessedIndex = i + 1; // Rastreia progresso
        }
    }
};
```

### ✅ Removido: Diálogos de Confirmação

- **Antes**: `confirm()` ao clicar em "Limpar" ou "Parar"
- **Depois**: Execução imediata (UX mais rápida)

### ✅ Corrigido: Settings Gear Não Funcionava

- Agora a engrenagem ⚙️ **realmente** abre/fecha o painel de configurações

---

## 🔮 Roadmap Futuro

### Fase 1: Refatoração Web ✅ (Completo - 12/02/2026)
- [x] Remover Groq/Hybrid (só Web Speech)
- [x] Fixar bug de repetição de texto
- [x] Remover confirmações
- [x] Interface mobile-responsive
- [x] Settings funcionando

### Fase 2: Linux Desktop App ✅ (Completo - 12/02/2026)
- [x] Tauri desktop app inicializado
- [x] System tray com ícone e menu
- [x] Comandos Tauri para `ydotool` (injeção de texto)
- [x] Configuração completa (Cargo.toml + tauri.conf.json)
- [x] Compilação funcionando

### Fase 3: Global Hotkeys ✅ (Completo - 12/02/2026)
- [x] **Global hotkey Super+H** - inicia/para gravação de qualquer lugar
- [x] Listener Tauri no backend
- [x] Event emitter para frontend
- [x] Documentação atualizada

### Fase 4: Build de Produção + Release ✅ (Completo - 12/02/2026)
- [x] Build de produção otimizada (`cargo tauri build`)
- [x] Binário nativo standalone (15MB)
- [x] Instalador .deb para Debian/Ubuntu (4.3MB)
- [x] Instalador .rpm para Fedora/RHEL (4.3MB)
- [x] Release pública no GitHub (v0.0.1-pre-alpha)
- [x] Repositório público: https://github.com/Deivisan/DeiviTech-VoiceHub
- [ ] Instalador .AppImage (pendente - investigação necessária)

### Fase 5: Melhorias UX (Em Andamento)
- [ ] Botão "Type Text" no frontend (integrar `inject_text` command)
- [ ] Menu completo no system tray (Start/Stop/Settings/Quit)
- [ ] Hotkey configurável via settings (Ctrl+Alt+V, Super+Shift+R, etc.)
- [ ] Auto-start com systemd

### Fase 6: Features Avançadas (Futuro)
- [ ] Multi-sessões com tabs
- [ ] Integração com AI agents (GPT-4o/Claude para refinamento)
- [ ] Export para arquivos (.txt, .md, .docx)
- [ ] Histórico de transcrições
- [ ] Comandos de voz (ex: "novo parágrafo", "apagar última frase")

---

## 🤝 Contribuindo

Pull requests são bem-vindos! Para mudanças grandes, abra uma issue primeiro.

### Desenvolvimento Local

1. Fork o repositório
2. Crie uma branch: `git checkout -b feature/nova-feature`
3. Commit: `git commit -m 'Add: nova feature incrível'`
4. Push: `git push origin feature/nova-feature`
5. Abra um Pull Request

---

## 📜 Licença

MIT License - veja [LICENSE](LICENSE) para detalhes.

---

## 👨‍💻 Autor

**Deivison Santana** ([@deivisan](https://github.com/deivisan))

- 🌐 Arch Linux + COSMIC DE
- 🦞 DevSan AGI - Space Lobster Edition
- ⚡ Bun-first, CLI-first, autonomia total

---

## 🙏 Agradecimentos

- **Web Speech API** (Google) - Motor de transcrição gratuito
- **Bun** - Runtime JavaScript mais rápido do mundo
- **Tauri** - Framework desktop nativo e leve
- **Rust** - Linguagem de sistemas segura e performática
- **COSMIC DE** - Desktop environment moderno para Linux
- **ydotool** - Text injection para Wayland

---

<div align="center">

**🦞 Feito com ❤️ em Arch Linux**

[⭐ Star no GitHub](https://github.com/deivisan/DeiviTech-VoiceHub) • [🐛 Reportar Bug](https://github.com/deivisan/DeiviTech-VoiceHub/issues) • [💡 Sugerir Feature](https://github.com/deivisan/DeiviTech-VoiceHub/issues)

</div>
