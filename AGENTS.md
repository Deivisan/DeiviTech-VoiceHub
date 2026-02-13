# 🎤 DeiviTech VoiceHub - Contexto do Projeto

> **📍 Repo**: `~/Projetos/DeiviTech-VoiceHub/`  
> **🎯 Propósito**: Sistema profissional de ditado de voz em tempo real para Linux  
> **📅 Atualizado**: 2026-02-13  

---

## 🎙️ O Que É Este Projeto?

VoiceHub é uma aplicação desktop Tauri que permite **ditado de voz em tempo real** usando a Web Speech API (100% gratuito, sem API keys).

### ✨ Features Principais

- ✅ **Transcrição em tempo real** - Web Speech API
- ✅ **100% gratuito** - sem limites de taxa
- ✅ **Pontuação automática** - vírgulas, pontos, interrogações
- ✅ **Multi-idiomas** - 8 idiomas suportados
- ✅ **Tauri Desktop** - binário nativo ~10MB
- ✅ **System Tray** - ícone e menu (mostrar/ocultar)
- ✅ **Injeção de texto** - via `ydotool`

---

## 🏗️ Arquitetura

```
DeiviTech-VoiceHub/
├── src/                    ← Frontend React + TypeScript
│   ├── components/         ← Componentes React
│   └── hooks/             ← Hooks customizados
├── src-tauri/             ← Backend Rust (Tauri)
│   └── src/
│       └── main.rs        ← Comandos Tauri
├── package.json           ← Dependências Bun
└── README.md              ← Documentação completa
```

**Stack**:
- 🖥️ **Frontend**: React + TypeScript + Bun
- ⚙️ **Backend**: Rust (Tauri)
- 🎤 **STT**: Web Speech API (browser)
- 🐧 **Target**: Linux (Arch)

---

## 🚀 Comandos Essenciais

```bash
# Instalar dependências
bun install

# Rodar em desenvolvimento
bun run tauri dev

# Build para produção
bun run tauri build

# O binário será gerado em:
# src-tauri/target/release/DeiviTech-VoiceHub
```

---

## 🎯 Uso do VoiceHub

### Pré-requisitos
```bash
# Instalar ydotool (para injeção de texto)
sudo pacman -S ydotool

# Iniciar serviço ydotool
sudo systemctl enable --now ydotoold
```

### Executar
```bash
# Via script (se disponível)
./start-voicehub.sh

# Ou direto
~/Projetos/DeiviTech-VoiceHub/src-tauri/target/release/DeiviTech-VoiceHub
```

---

## 🛠️ Desenvolvimento

### Estrutura de Comandos Tauri

| Comando | Descrição |
|---------|-----------|
| `get_env_var` | Lê variável de ambiente |
| `check_ydotool` | Verifica se ydotool está disponível |
| `type_text` | Injeta texto via ydotool |
| `get_audio_devices` | Lista dispositivos de áudio |

### Arquivos Importantes

- `src/hooks/useSpeechRecognition.ts` - Hook principal STT
- `src/components/TranscriptionPanel.tsx` - Painel de transcrição
- `src-tauri/src/main.rs` - Comandos Rust
- `src-tauri/tauri.conf.json` - Config Tauri

---

## 📋 Roadmap

- ✅ **Fase 1**: Web App completo
- ✅ **Fase 2**: Desktop Tauri básico
- ✅ **Fase 3**: System tray e ydotool
- 🚧 **Fase 4**: Otimização e polish

---

## 🔗 Links

- **Repo**: `~/Projetos/DeiviTech-VoiceHub/`
- **Banco-API**: `~/Projetos/Prompts/Docs/banco-api.md`
- **Documentação**: `~/Projetos/DeiviTech-VoiceHub/README.md`

---

> **"Mãos livres, mente focada. Ditado profissional para Linux."**
