# 🎤 VoiceHub - Ditado de Voz para COSMIC Desktop

[![Release](https://img.shields.io/badge/release-v0.0.2--pre--alpha-orange)](https://github.com/deivisan/DeiviTech-VoiceHub/releases)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![COSMIC](https://img.shields.io/badge/COSMIC-1.0+-purple)](https://system76.com/cosmic)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange)](https://www.rust-lang.org)

> **Ditado de voz em tempo real nativo para COSMIC Desktop Environment**  
> Transforme sua fala em texto sem sair do desktop, 100% gratuito e com Web Speech API integrada.

---

## 📋 Índice

- [✨ Features](#-features)
- [🎯 Status do Projeto](#-status-do-projeto)
- [🚀 Instalação](#-instalação)
- [💡 Como Usar](#-como-usar)
- [🏗️ Arquitetura](#️-arquitetura)
- [🛠️ Desenvolvimento](#️-desenvolvimento)
- [📊 Roadmap](#-roadmap)
- [🤝 Contribuindo](#-contribuindo)
- [📄 Licença](#-licença)

---

## ✨ Features

### ✅ Implementado (v0.0.2-pre-alpha)

- ✅ **Applet COSMIC Nativo** - Integração total com o painel COSMIC
- ✅ **Web Speech API** - Reconhecimento de voz via webkit2gtk
- ✅ **Transcrição em Tempo Real** - Veja o texto aparecer enquanto fala
- ✅ **Multi-idiomas** - Suporte a 8+ idiomas (pt-BR padrão)
- ✅ **Interface Minimalista** - UI limpa e intuitiva
- ✅ **Estatísticas ao Vivo** - Contador de palavras e tempo de gravação
- ✅ **Clipboard Integration** - Copie transcrições com um clique
- ✅ **Injeção de Texto** - Injete transcrição diretamente em apps via ydotool
- ✅ **System Tray** - Ícone visual muda de cor ao gravar (🔴/⚪)
- ✅ **100% Gratuito** - Sem limites de uso ou API keys

### 🚧 Em Desenvolvimento

- 🚧 **Pontuação Automática** - Vírgulas, pontos e interrogações automáticas
- 🚧 **Auto-Injeção** - Injetar texto automaticamente ao parar
- 🚧 **Comandos de Voz** - "novo parágrafo", "apagar última frase"
- 🚧 **Histórico** - Salvar transcrições anteriores
- 🚧 **Temas** - Suporte a temas COSMIC nativos

---

## 🎯 Status do Projeto

**Versão Atual**: `v0.0.2-pre-alpha`  
**Status**: 🟡 **Pré-Alpha** - Funcional mas em testes iniciais

### O que funciona

✅ Interface 100% operacional  
✅ Gravação de voz com Web Speech API  
✅ Transcrição em tempo real  
✅ Copiar para clipboard  
✅ Injeção via ydotool  
✅ Configuração persistente  

### Limitações Conhecidas

⚠️ **Web Speech API** requer conexão à internet (usa servidores Google)  
⚠️ Permissões de microfone podem precisar de configuração manual  
⚠️ Primeiro uso pode ter latência maior  
⚠️ Suporte limitado a alguns navegadores WebKit  

---

## 🚀 Instalação

### Pré-requisitos

```bash
# Arch Linux / Manjaro
sudo pacman -S rust gtk3 webkit2gtk ydotool

# Ubuntu / Pop!_OS
sudo apt install rustc cargo libgtk-3-dev libwebkit2gtk-4.0-dev ydotool

# Fedora
sudo dnf install rust gtk3-devel webkit2gtk4.0-devel ydotool
```

### Dependências COSMIC

- **COSMIC Desktop** 1.0 ou superior
- **libcosmic** (instalado automaticamente)

### Instalação via Compilação

```bash
# 1. Clonar repositório
git clone https://github.com/deivisan/DeiviTech-VoiceHub.git
cd DeiviTech-VoiceHub

# 2. Compilar
cargo build --release

# 3. Instalar
sudo cp target/release/cosmic-applet-voicehub /usr/local/bin/
sudo cp res/com.deivisan.voicehub.desktop /usr/share/applications/

# 4. Configurar ydotool
sudo systemctl enable --now ydotoold

# 5. Reiniciar painel COSMIC (ou fazer logout/login)
pkill cosmic-panel
```

### Verificar Instalação

```bash
# O applet deve aparecer automaticamente no painel
# Ou execute manualmente para debug:
/usr/local/bin/cosmic-applet-voicehub
```

---

## 💡 Como Usar

### Interface Básica

1. **Abrir Popup** - Clique no ícone 🎤 no painel COSMIC
2. **Iniciar Gravação** - Clique em "Iniciar Gravação" (ícone fica 🔴)
3. **Falar** - Diga o que deseja transcrever
4. **Ver Transcrição** - Texto aparece em tempo real
5. **Parar** - Clique em "Parar Gravação"

### Ações Disponíveis

| Botão | Descrição |
|-------|-----------|
| **📋 Copiar** | Copia transcrição para clipboard |
| **🗑️ Limpar** | Apaga texto atual |
| **⌨️ Injetar** | Injeta texto no app ativo (via ydotool) |
| **🔴/⚪ Gravar** | Inicia/para gravação |

### Configuração de Idioma

```bash
# Editar configuração (padrão: pt-BR)
~/.config/cosmic/com.deivisan.voicehub/config.toml

# Idiomas disponíveis:
# pt-BR (Português Brasil)
# en-US (Inglês EUA)
# es-ES (Espanhol)
# fr-FR (Francês)
# de-DE (Alemão)
# it-IT (Italiano)
# ja-JP (Japonês)
# zh-CN (Chinês)
```

### Atalhos de Teclado

🚧 *Em desenvolvimento*

---

## 🏗️ Arquitetura

### Diagrama de Sistema

```
┌─────────────────────────────────────────────────┐
│         COSMIC Desktop Environment              │
│  ┌───────────────────────────────────────────┐ │
│  │  VoiceHub Applet (Rust)                   │ │
│  │  ┌─────────────────────────────────────┐  │ │
│  │  │ UI (libcosmic)                      │  │ │
│  │  │  - Popup window                     │  │ │
│  │  │  - Panel icon                       │  │ │
│  │  │  - Stats display                    │  │ │
│  │  └─────────────────────────────────────┘  │ │
│  │                ▲                           │ │
│  │                │ (channel)                 │ │
│  │  ┌─────────────────────────────────────┐  │ │
│  │  │ Speech Recognition (webkit2gtk)     │  │ │
│  │  │  - WebView invisível                │  │ │
│  │  │  - Web Speech API (JavaScript)      │  │ │
│  │  │  - Message handler                  │  │ │
│  │  └─────────────────────────────────────┘  │ │
│  │                ▲                           │ │
│  │                │                           │ │
│  │     Google Speech API (online)            │ │
│  └───────────────────────────────────────────┘ │
│                  │                              │
│                  ▼                              │
│  ┌───────────────────────────────────────────┐ │
│  │ Sistema (ydotool, clipboard)              │ │
│  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

### Stack Tecnológica

| Camada | Tecnologia | Propósito |
|--------|------------|-----------|
| **Desktop** | COSMIC 1.0+ | Ambiente desktop |
| **Framework** | libcosmic | Applet nativo |
| **Backend** | Rust 1.75+ | Lógica principal |
| **Speech** | Web Speech API | Reconhecimento de voz |
| **WebView** | webkit2gtk | Container JavaScript |
| **Async** | Tokio | Runtime assíncrono |
| **Config** | cosmic-config | Persistência |
| **Inject** | ydotool | Injeção de texto |

### Estrutura de Arquivos

```
DeiviTech-VoiceHub/
├── src/
│   ├── main.rs                 # Entry point (GTK init)
│   ├── app.rs                  # Applet COSMIC (UI + lógica)
│   ├── speech_recognition.rs   # WebView + Web Speech API
│   ├── config.rs               # Configuração persistente
│   └── text_inject.rs          # Integração ydotool
├── res/
│   └── com.deivisan.voicehub.desktop
├── Cargo.toml                  # Dependências
├── README.md                   # Este arquivo
└── LICENSE                     # MIT
```

---

## 🛠️ Desenvolvimento

### Compilar em Modo Debug

```bash
cargo build
cargo run
```

### Testes

```bash
# Rodar testes unitários
cargo test

# Testar UI manualmente
./target/debug/cosmic-applet-voicehub
```

### Logs de Debug

```bash
# Ver logs em tempo real
journalctl -f -t cosmic-applet-voicehub

# Ou execute com RUST_LOG
RUST_LOG=debug ./target/debug/cosmic-applet-voicehub
```

### Contribuindo com Código

1. Fork o repositório
2. Crie uma branch (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -am 'feat: adiciona nova feature'`)
4. Push para a branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

**Padrão de Commits**: [Conventional Commits](https://www.conventionalcommits.org/)

---

## 📊 Roadmap

### v0.0.3 (Próxima Release)

- [ ] Pontuação automática inteligente
- [ ] Corretor ortográfico
- [ ] Fallback offline (Whisper.cpp local)
- [ ] Configuração de atalhos de teclado
- [ ] Testes automatizados

### v0.1.0 (Alpha)

- [ ] Comandos de voz ("novo parágrafo", "deletar")
- [ ] Histórico de transcrições
- [ ] Exportar para Markdown/TXT
- [ ] Suporte a mais idiomas
- [ ] Otimização de performance

### v1.0.0 (Stable)

- [ ] Modelo offline 100% local
- [ ] Treinamento de vocabulário personalizado
- [ ] Integração com apps (VSCode, LibreOffice)
- [ ] Reconhecimento de múltiplos falantes
- [ ] API para desenvolvedores

---

## 🐛 Bugs Conhecidos

| ID | Descrição | Status | Workaround |
|----|-----------|--------|------------|
| #1 | Primeira transcrição tem latência | 🟡 Investigando | Esperar 2-3s antes de falar |
| #2 | Permissões microfone não persistem | 🟡 Investigando | Reconfigurar manualmente |
| #3 | Ydotool precisa de sudo | 📝 Documentado | Adicionar user ao grupo ydotool |

**Reportar bugs**: [GitHub Issues](https://github.com/deivisan/DeiviTech-VoiceHub/issues)

---

## 🤝 Contribuindo

Contribuições são muito bem-vindas! 🎉

### Como Ajudar

- 🐛 **Reportar bugs** - Abra uma issue descrevendo o problema
- 💡 **Sugerir features** - Compartilhe suas ideias
- 🌍 **Traduzir** - Ajude com novos idiomas
- 📝 **Documentar** - Melhore a documentação
- 💻 **Codificar** - Implemente features ou corrija bugs

### Diretrizes

- Use `cargo fmt` antes de commitar
- Siga as convenções Rust
- Adicione testes para novas features
- Atualize documentação quando necessário

---

## 📄 Licença

Este projeto está licenciado sob a licença **MIT** - veja o arquivo [LICENSE](LICENSE) para detalhes.

```
MIT License

Copyright (c) 2026 Deivison Santana

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```

---

## 🙏 Agradecimentos

- **System76** - Pelo incrível COSMIC Desktop
- **Rust Community** - Pelas ferramentas excelentes
- **Web Speech API** - Pelo reconhecimento gratuito
- **Você** - Por testar e contribuir! ❤️

---

## 📞 Contato

**Desenvolvedor**: Deivison Santana  
**GitHub**: [@deivisan](https://github.com/deivisan)  
**Issues**: [GitHub Issues](https://github.com/deivisan/DeiviTech-VoiceHub/issues)

---

<div align="center">

**⭐ Se gostou, deixe uma estrela no GitHub! ⭐**

[🐛 Reportar Bug](https://github.com/deivisan/DeiviTech-VoiceHub/issues/new?labels=bug) · 
[💡 Sugerir Feature](https://github.com/deivisan/DeiviTech-VoiceHub/issues/new?labels=enhancement) · 
[📖 Documentação](https://github.com/deivisan/DeiviTech-VoiceHub/wiki)

</div>
