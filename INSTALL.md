# 🎤 VoiceHub - Instalação e Configuração

> **Applet de ditado de voz para COSMIC Desktop**

---

## ✅ Status da Instalação

- ✅ **Binário compilado**: `/usr/local/bin/cosmic-applet-voicehub` (15MB)
- ✅ **Desktop entry**: `/usr/share/applications/com.deivisan.voicehub.desktop`
- ✅ **Cache atualizado**: `update-desktop-database` executado
- ✅ **Dependências**: libcosmic 0.1.0, ydotool configurado

---

## 🚀 Como Adicionar ao COSMIC Panel

### Método 1: Via COSMIC Settings (Recomendado)

1. **Abra COSMIC Settings**
   ```bash
   cosmic-settings
   ```

2. **Navegue até Desktop → Panel**

3. **Clique em "Applets"**

4. **Procure por "VoiceHub"** na lista de applets disponíveis

5. **Clique em "Add" ou arraste para o panel**

6. **Posicione onde preferir** (centro, direita, esquerda)

### Método 2: Edição Manual (Avançado)

Se o applet não aparecer automaticamente:

1. **Reinicie o COSMIC Panel**:
   ```bash
   pkill cosmic-panel
   cosmic-panel &
   ```

2. **Verifique se o desktop entry está correto**:
   ```bash
   cat /usr/share/applications/com.deivisan.voicehub.desktop
   ```

3. **Execute o applet manualmente para testar**:
   ```bash
   /usr/local/bin/cosmic-applet-voicehub
   ```

---

## 🔧 Configuração

### Configurações Disponíveis

O VoiceHub salva configurações em:
```
~/.config/cosmic/com.deivisan.voicehub/
```

**Opções configuráveis**:
- 🌍 **Idioma** (padrão: `pt-BR`)
- ✏️ **Pontuação automática** (padrão: `true`)
- ⌨️ **Injeção automática de texto** (padrão: `false`)
- 💾 **Salvar histórico** (padrão: `true`)

### Editar Configuração Manualmente

```bash
# Ver config atual
cat ~/.config/cosmic/com.deivisan.voicehub/v1/config

# Exemplo de edição
nano ~/.config/cosmic/com.deivisan.voicehub/v1/config
```

---

## 🎙️ Como Usar

### Interface do Applet

1. **Clique no ícone do microfone** no panel para abrir o popup

2. **Botão "Gravar"** → Inicia/para a gravação (⏺️/⏹️)

3. **Área de texto** → Mostra transcrição em tempo real

4. **Estatísticas** → Duração da gravação e contagem de palavras

5. **Botões de ação**:
   - 📋 **Copiar** → Copia texto para área de transferência
   - ⌨️ **Injetar** → Digita texto no app ativo (via ydotool)
   - 🗑️ **Limpar** → Apaga transcrição atual

### Atalhos

- **Clique no ícone do panel**: Abre/fecha popup
- **Esc**: Fecha popup
- **Ctrl+C** (no popup): Copia texto

---

## ⚠️ Pré-requisitos

### ydotool (Injeção de Texto)

O VoiceHub usa `ydotool` para digitar texto automaticamente. Certifique-se de que está configurado:

```bash
# Instalar (se necessário)
sudo pacman -S ydotool

# Ativar serviço
sudo systemctl enable --now ydotoold

# Verificar status
systemctl status ydotoold
```

**Permissões**: Adicione seu usuário ao grupo `input`:
```bash
sudo usermod -aG input $USER
# Faça logout e login novamente
```

### Microfone Configurado

Verifique se o microfone está funcionando:
```bash
# Listar dispositivos de áudio
pactl list sources short

# Testar gravação (5 segundos)
arecord -d 5 /tmp/test.wav && aplay /tmp/test.wav
```

---

## 🐛 Troubleshooting

### Applet não aparece na lista

```bash
# 1. Verificar se o binário existe
ls -lh /usr/local/bin/cosmic-applet-voicehub

# 2. Verificar desktop entry
cat /usr/share/applications/com.deivisan.voicehub.desktop

# 3. Atualizar cache
sudo update-desktop-database /usr/share/applications

# 4. Reiniciar panel
pkill cosmic-panel && cosmic-panel &
```

### Applet não inicia

```bash
# Executar com logs de debug
RUST_LOG=debug /usr/local/bin/cosmic-applet-voicehub
```

### ydotool não funciona

```bash
# Verificar serviço
systemctl status ydotoold

# Reiniciar serviço
sudo systemctl restart ydotoold

# Testar manualmente
ydotool type "teste"
```

### Microfone não captura áudio

```bash
# Verificar dispositivo padrão
pactl info | grep "Default Source"

# Ajustar volume
pactl set-source-volume @DEFAULT_SOURCE@ 80%

# Desmutar
pactl set-source-mute @DEFAULT_SOURCE@ 0
```

---

## 🔮 Próximos Passos

### ⚠️ Funcionalidade Ainda Não Implementada

**Reconhecimento de Fala (STT)** está como **placeholder**. Atualmente:
- ❌ Não captura áudio real
- ❌ Não transcreve fala
- ✅ Interface funciona perfeitamente
- ✅ Injeção de texto funciona
- ✅ Cópia para clipboard funciona

**Opções para implementar STT**:

#### Opção A: Web Speech API (Google)
```bash
# Adicionar webkit2gtk ao Cargo.toml
webkit2gtk = "2.0"

# Criar WebView embarcado
# Usar JavaScript SpeechRecognition API
# Ponte JS ↔ Rust
```

**Prós**: Gratuito, preciso, multi-idiomas  
**Contras**: Requer internet, dependência do Google

#### Opção B: Whisper (OpenAI - Local)
```bash
# Adicionar whisper-rs ao Cargo.toml
whisper-rs = "0.10"

# Download do modelo (~500MB)
# Transcrição offline
```

**Prós**: 100% offline, privado  
**Contras**: Modelos grandes, CPU intensivo

#### Opção C: Vosk (Offline)
```bash
# Adicionar vosk ao Cargo.toml
vosk = "0.3"

# Modelos menores (~50MB)
# Transcrição local
```

**Prós**: Leve, offline, rápido  
**Contras**: Menos preciso que Whisper

---

## 📚 Arquitetura

```
VoiceHub
│
├── src/main.rs              # Entry point
├── src/app.rs               # Lógica principal do applet
├── src/config.rs            # Sistema de configuração
├── src/text_inject.rs       # Integração com ydotool
│
└── Próximas implementações:
    ├── src/audio_capture.rs    # Captura de áudio (cpal)
    ├── src/speech_recognition.rs # Engine STT
    └── src/history.rs           # Persistência de transcrições
```

---

## 📝 Logs

```bash
# Logs do COSMIC
journalctl --user -u cosmic-panel -f

# Logs do applet (quando executado manualmente)
RUST_LOG=info /usr/local/bin/cosmic-applet-voicehub
```

---

## 🔗 Links Úteis

- **Repositório**: `~/Projetos/DeiviTech-VoiceHub/`
- **Docs arquitetura**: `docs/COSMIC_APPLET_PROPOSAL.md`
- **Código legado Tauri**: `legacy/`

---

> **Status**: ✅ Instalado | 🚧 STT pendente | 🎯 Pronto para testes de UI
