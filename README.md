# 🎤 VoiceHub - COSMIC Applet Edition

> **Applet de ditado de voz profissional para COSMIC Desktop**

![Status](https://img.shields.io/badge/status-UI_ready-green)
![STT](https://img.shields.io/badge/STT-pending-orange)
![Platform](https://img.shields.io/badge/platform-Linux-blue)
![Desktop](https://img.shields.io/badge/desktop-COSMIC-purple)

---

## 📋 Status Atual

### ✅ Funcionalidades Implementadas

- ✅ **Interface nativa COSMIC** - Applet totalmente integrado
- ✅ **System tray icon** - Ícone de microfone no panel
- ✅ **Popup window** - Janela de transcrição flutuante
- ✅ **Botões de ação** - Copiar, Injetar, Limpar
- ✅ **Sistema de configuração** - Via CosmicConfigEntry
- ✅ **Integração com ydotool** - Injeção de texto funcional
- ✅ **Clipboard** - Cópia para área de transferência
- ✅ **Multi-idioma** - Suporte a 8 idiomas (UI pronta)

### 🚧 Em Desenvolvimento

- ⏳ **Reconhecimento de fala (STT)** - Interface pronta, engine pendente
- ⏳ **Captura de áudio** - Microfone não conectado ainda
- ⏳ **Transcrição em tempo real** - Placeholder implementado

---

## 🚀 Instalação Rápida

### Pré-requisitos

```bash
# 1. COSMIC Desktop 1.0.6+
# 2. ydotool para injeção de texto
sudo pacman -S ydotool

# 3. Configurar ydotool
cd ~/Projetos/DeiviTech-VoiceHub
./setup-ydotool.sh
```

### Build e Instalação

```bash
# Clone (se ainda não tiver)
cd ~/Projetos/DeiviTech-VoiceHub

# Build
cargo build --release

# Instalar
sudo cp target/release/cosmic-applet-voicehub /usr/local/bin/

# Desktop entry já está em:
# /usr/share/applications/com.deivisan.voicehub.desktop
```

### Adicionar ao COSMIC Panel

1. Abra **COSMIC Settings**
2. Vá para **Desktop → Panel**
3. Clique em **Applets**
4. Encontre **VoiceHub** e adicione

---

## 🎯 Como Usar

### Interface

```
┌─────────────────────────────────────┐
│  🎤 VoiceHub                     ╳  │
├─────────────────────────────────────┤
│                                     │
│  ┌─────────────────────────────┐   │
│  │ Área de transcrição...      │   │
│  │                             │   │
│  │                             │   │
│  └─────────────────────────────┘   │
│                                     │
│  ⏱️  00:00 | 📝 0 palavras          │
│                                     │
│  [ ⏺️ Gravar ] [ 📋 Copiar ]       │
│  [ ⌨️ Injetar ] [ 🗑️ Limpar ]      │
└─────────────────────────────────────┘
```

### Fluxo de Uso

1. **Clique no ícone** 🎤 no panel → Abre popup
2. **Clique "Gravar"** → ⏺️ muda para ⏹️ (stop)
3. **Fale normalmente** → Texto aparece em tempo real
4. **Clique "Parar"** → Finaliza gravação
5. **Escolha ação**:
   - 📋 **Copiar** → Cola com Ctrl+V
   - ⌨️ **Injetar** → Digita automaticamente
   - 🗑️ **Limpar** → Apaga transcrição

---

## ⚙️ Configuração

### Arquivo de Config

```bash
~/.config/cosmic/com.deivisan.voicehub/v1/config
```

### Opções Disponíveis

```ron
(
    language: "pt-BR",           // Idioma de reconhecimento
    auto_punctuation: true,      // Pontuação automática
    auto_inject: false,          // Injetar ao parar gravação
    save_history: true,          // Salvar histórico
)
```

### Idiomas Suportados

- 🇧🇷 `pt-BR` - Português (Brasil)
- 🇺🇸 `en-US` - English (US)
- 🇪🇸 `es-ES` - Español
- 🇫🇷 `fr-FR` - Français
- 🇩🇪 `de-DE` - Deutsch
- 🇮🇹 `it-IT` - Italiano
- 🇯🇵 `ja-JP` - 日本語
- 🇨🇳 `zh-CN` - 中文

---

## 🏗️ Arquitetura

### Stack Tecnológica

- **UI Framework**: libcosmic 0.1.0
- **Language**: Rust 1.75+
- **Desktop**: COSMIC 1.0.6
- **Platform**: Linux (Wayland)
- **Text Injection**: ydotool
- **Clipboard**: arboard

### Estrutura do Projeto

```
DeiviTech-VoiceHub/
│
├── src/
│   ├── main.rs              # Entry point (7 linhas)
│   ├── app.rs               # Lógica principal (266 linhas)
│   ├── config.rs            # Sistema de config (105 linhas)
│   ├── text_inject.rs       # Integração ydotool (45 linhas)
│   └── i18n/                # Traduções (não ativadas)
│
├── legacy/                  # Código Tauri original
│   ├── src/                 # Web UI (React)
│   └── src-tauri/           # Backend Tauri
│
├── docs/
│   └── COSMIC_APPLET_PROPOSAL.md  # Arquitetura completa
│
├── Cargo.toml               # Dependências
├── INSTALL.md               # Guia de instalação detalhado
├── test-applet.sh           # Script de testes
└── setup-ydotool.sh         # Configuração do ydotool
```

---

## 🔧 Desenvolvimento

### Build

```bash
# Debug (rápido, ~3s)
cargo build

# Release (otimizado, ~35s)
cargo build --release

# Tamanho: ~15MB (stripped)
```

### Debug

```bash
# Logs completos
RUST_LOG=debug /usr/local/bin/cosmic-applet-voicehub

# Logs do COSMIC
journalctl --user -u cosmic-panel -f

# Verificar se applet está rodando
ps aux | grep cosmic-applet-voicehub
```

### Teste

```bash
# Teste automatizado
./test-applet.sh

# Teste manual
/usr/local/bin/cosmic-applet-voicehub
```

---

## 🐛 Troubleshooting

### Applet não aparece no panel

```bash
# 1. Verificar desktop entry
cat /usr/share/applications/com.deivisan.voicehub.desktop

# 2. Atualizar cache
sudo update-desktop-database /usr/share/applications

# 3. Reiniciar panel
pkill cosmic-panel && cosmic-panel &
```

### ydotool não funciona

```bash
# 1. Rodar script de setup
./setup-ydotool.sh

# 2. Ou manualmente:
sudo pkill ydotoold
sudo ydotoold &
sleep 1
sudo chmod 666 /tmp/.ydotool_socket

# 3. Testar
export YDOTOOL_SOCKET=/tmp/.ydotool_socket
ydotool type "teste"
```

### Permissões negadas

```bash
# Adicionar ao grupo input (geralmente não necessário)
sudo usermod -aG input $USER

# Fazer logout e login novamente
```

---

## 🔮 Próximos Passos

### Fase 1: Implementar STT (Prioridade Alta)

**Opção A: Web Speech API** (Recomendado)
- ✅ Gratuito, sem API keys
- ✅ Preciso, multi-idiomas
- ✅ Tempo real
- ❌ Requer internet
- ❌ Dependência do Google

**Opção B: Whisper (Local)**
- ✅ 100% offline
- ✅ Muito preciso
- ❌ Modelos grandes (~500MB)
- ❌ CPU/GPU intensivo

**Opção C: Vosk (Leve)**
- ✅ Offline
- ✅ Modelos pequenos (~50MB)
- ❌ Menos preciso

### Fase 2: Captura de Áudio
- [ ] Integrar PulseAudio/PipeWire
- [ ] Usar crate `cpal` para captura
- [ ] Implementar VAD (Voice Activity Detection)

### Fase 3: Features Avançadas
- [ ] Auto-pontuação com IA
- [ ] Histórico de transcrições
- [ ] Atalhos de teclado customizáveis
- [ ] Exportar para arquivo
- [ ] Comandos de voz (parar, limpar, etc.)

---

## 📚 Documentação

- 📖 **Instalação completa**: [INSTALL.md](INSTALL.md)
- 🏗️ **Arquitetura**: [docs/COSMIC_APPLET_PROPOSAL.md](docs/COSMIC_APPLET_PROPOSAL.md)
- 📜 **Contexto do projeto**: [AGENTS.md](AGENTS.md)
- 🗂️ **Código legado Tauri**: [legacy/README.md](legacy/README.md)

---

## 🤝 Contribuindo

Este é um projeto pessoal, mas sugestões são bem-vindas!

### Áreas que Precisam de Ajuda

1. **Escolha da engine STT** - Qual usar?
2. **Captura de áudio** - Melhor crate?
3. **Performance** - Otimizações?
4. **UX** - Melhorias na interface?

---

## 📜 Licença

MIT License - Use como quiser!

---

## 🎯 Visão

> **"Mãos livres, mente focada. Ditado profissional para Linux."**

VoiceHub nasceu da necessidade de um sistema de ditado de voz **nativo**, **rápido** e **integrado** ao Linux. Diferente de soluções web ou extensões de navegador, VoiceHub é um applet COSMIC de verdade, com acesso direto ao sistema.

### Por Que COSMIC?

- 🚀 **Performance nativa** - Rust puro, sem overhead web
- 🎨 **Integração perfeita** - Segue design system do COSMIC
- ⚡ **Baixa latência** - Acesso direto ao hardware
- 🔒 **Privacidade** - Sem telemetria, dados ficam locais

---

## 📊 Estatísticas

- **Linhas de código**: ~420 (sem contar deps)
- **Binário**: 15MB (com debug symbols)
- **Tempo de build**: 35s (release)
- **Deps diretas**: 5 (libcosmic, tokio, arboard, serde)
- **Avisos de compilação**: 1 (inofensivo)

---

## 🔗 Links

- **GitHub**: [DeiviTech-VoiceHub](https://github.com/deivisan/DeiviTech-VoiceHub) (se publicado)
- **COSMIC Desktop**: [https://system76.com/cosmic](https://system76.com/cosmic)
- **libcosmic**: [https://github.com/pop-os/libcosmic](https://github.com/pop-os/libcosmic)

---

**Desenvolvido com ❤️ por Deivison Santana**  
*Arch Linux + COSMIC Desktop*
