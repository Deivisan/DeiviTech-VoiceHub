# 🧪 Guia de Testes - VoiceHub Applet

**Última atualização**: 2026-02-13 12:35

---

## 🎯 O Que Testar

### 1. **Ícone no Panel** ✅

#### Como testar:
1. Olhe para o panel COSMIC (onde você adicionou o VoiceHub)
2. Você deve ver um **ícone de microfone cinza** 🎤

#### Comportamento esperado:
- **Cinza**: Applet parado (não gravando)
- **Vermelho**: Applet gravando (ativo)

#### Se o ícone não aparecer:
```bash
# 1. Verificar se o applet está rodando
ps aux | grep cosmic-applet-voicehub

# 2. Se não estiver, o COSMIC vai iniciar automaticamente
# Ou force restart do panel:
pkill cosmic-panel && cosmic-panel &

# 3. Verificar logs
journalctl --user -f | grep voicehub
```

---

### 2. **Abrir Popup** ✅

#### Como testar:
1. **Clique no ícone do microfone** no panel
2. Uma janela popup deve aparecer

#### Conteúdo esperado:
```
┌─────────────────────────────────────┐
│  VoiceHub              0 palavras   │
├─────────────────────────────────────┤
│  ⚪ Pronto                          │
│  ┌─────────────────────────────┐   │
│  │ Clique em 'Iniciar          │   │
│  │ Gravação' para começar...   │   │
│  └─────────────────────────────┘   │
│  ⏱️ 00:00        🌍 pt-BR          │
│  [📋 Copiar] [🗑️ Limpar] [📤 Injetar]│
│  [🎤 Iniciar Gravação]             │
└─────────────────────────────────────┘
```

#### Se não abrir:
- Clique novamente no ícone
- Verifique se há erros no journalctl

---

### 3. **Iniciar Gravação** 🚧

#### Como testar:
1. Com o popup aberto, clique em **"🎤 Iniciar Gravação"**
2. O botão deve mudar para **"⏹️ Parar Gravação"**
3. O indicador deve mudar de **⚪ Pronto** para **🔴 GRAVANDO**
4. O ícone do panel deve mudar para **vermelho/alto**
5. O timer deve começar a contar (00:00, 00:01, 00:02...)

#### Comportamento esperado:
- ✅ Botão muda de texto
- ✅ Indicador muda de cor
- ✅ Ícone do panel muda
- ✅ Timer conta
- ❌ **Transcrição NÃO funciona ainda** (STT não implementado)

---

### 4. **Parar Gravação** 🚧

#### Como testar:
1. Com gravação ativa, clique em **"⏹️ Parar Gravação"**
2. O botão volta para **"🎤 Iniciar Gravação"**
3. O indicador volta para **⚪ Pronto**
4. O ícone do panel volta para cinza
5. O timer volta para 00:00

#### Comportamento esperado:
- ✅ Tudo volta ao estado inicial
- ❌ **Nenhum texto é transcrito** (normal, STT pendente)

---

### 5. **Botão "Copiar"** ✅

#### Como testar:
1. Clique em **"📋 Copiar"**
2. Abra um editor de texto
3. Cole com `Ctrl+V`

#### Comportamento esperado:
- ✅ O texto placeholder deve ser colado: "Clique em 'Iniciar Gravação' para começar..."

#### Se não funcionar:
```bash
# Verificar se arboard está funcionando
echo "teste" | xclip -selection clipboard
xclip -o -selection clipboard
```

---

### 6. **Botão "Limpar"** ✅

#### Como testar:
1. Clique em **"🗑️ Limpar"**
2. A área de transcrição deve ficar vazia
3. Contador de palavras deve ir para 0

#### Comportamento esperado:
- ✅ Texto desaparece
- ✅ Contador zerado

---

### 7. **Botão "Injetar"** ⚠️

#### Pré-requisito:
```bash
# Configurar ydotool primeiro
cd ~/Projetos/DeiviTech-VoiceHub
./setup-ydotool.sh
```

#### Como testar:
1. Abra um editor de texto (gedit, kate, etc.)
2. Posicione o cursor onde quer o texto
3. Clique no popup do VoiceHub
4. Clique em **"📤 Injetar"**
5. Volte rápido para o editor (5 segundos)

#### Comportamento esperado:
- ✅ O texto do placeholder é digitado no editor
- ⏱️ Demora ~2 segundos (digita char por char)

#### Se não funcionar:
```bash
# 1. Verificar ydotool
export YDOTOOL_SOCKET=/tmp/.ydotool_socket
ydotool type "teste"

# 2. Verificar permissões do socket
ls -la /tmp/.ydotool_socket
# Deve mostrar: srw-rw-rw-

# 3. Reconfigurar se necessário
./setup-ydotool.sh
```

---

### 8. **Fechar Popup** ✅

#### Como testar:
1. **Clique novamente no ícone do panel**
2. OU clique fora do popup
3. OU pressione **Esc**

#### Comportamento esperado:
- ✅ Popup fecha
- ✅ Ícone permanece no panel

---

## 🐛 Problemas Conhecidos

### ❌ **Transcrição não funciona**
**Status**: ⏳ **ESPERADO** - STT não implementado ainda

**Explicação**: A interface está 100% pronta, mas o motor de reconhecimento de fala ainda não foi conectado. Isso é o próximo passo do desenvolvimento.

**Próxima implementação**: Escolher entre Web Speech API ou Whisper.cpp

---

### ⚠️ **Ícone não aparece no panel**

**Causas possíveis**:

1. **Applet não foi adicionado ao panel**
   ```bash
   # Solução: Adicione via COSMIC Settings
   cosmic-settings
   # Desktop → Panel → Applets → Adicionar VoiceHub
   ```

2. **Cache de ícones desatualizado**
   ```bash
   # Solução: Atualizar cache
   sudo update-desktop-database /usr/share/applications
   pkill cosmic-panel && cosmic-panel &
   ```

3. **Ícone symbolic não encontrado**
   ```bash
   # Verificar se existe
   find /usr/share/icons -name "audio-input-microphone-symbolic.svg"
   
   # Se não existir, instalar theme:
   sudo pacman -S adwaita-icon-theme
   ```

---

### ⚠️ **ydotool não injeta texto**

**Solução rápida**:
```bash
cd ~/Projetos/DeiviTech-VoiceHub
./setup-ydotool.sh
```

**Solução manual**:
```bash
# 1. Parar daemon
sudo pkill ydotoold

# 2. Remover socket
sudo rm -f /tmp/.ydotool_socket

# 3. Iniciar daemon
sudo ydotoold &

# 4. Esperar socket
sleep 1

# 5. Dar permissões
sudo chmod 666 /tmp/.ydotool_socket

# 6. Testar
export YDOTOOL_SOCKET=/tmp/.ydotool_socket
ydotool type "teste"
```

---

## ✅ Checklist de Teste Completo

Marque conforme testa:

- [ ] **Ícone aparece no panel** (cinza)
- [ ] **Clique abre popup**
- [ ] **Popup mostra interface completa**
- [ ] **Botão "Iniciar Gravação" funciona**
- [ ] **Ícone muda para vermelho ao gravar**
- [ ] **Indicador mostra "🔴 GRAVANDO"**
- [ ] **Timer conta segundos**
- [ ] **Botão "Parar Gravação" funciona**
- [ ] **Estado volta ao normal após parar**
- [ ] **Botão "Copiar" cola texto no clipboard**
- [ ] **Botão "Limpar" apaga texto**
- [ ] **Botão "Injetar" digita texto (após setup do ydotool)**
- [ ] **Popup fecha ao clicar no ícone novamente**
- [ ] **Popup fecha ao clicar fora**

---

## 📊 Resultado Esperado

### ✅ Funcionando (UI/UX)
- Interface completa
- Botões respondem
- Estados visuais corretos
- Clipboard funcional
- Injeção de texto (com ydotool configurado)

### 🚧 Não Funciona (Normal)
- **Reconhecimento de fala** - Próxima implementação
- **Captura de áudio** - Próxima implementação
- **Transcrição real** - Próxima implementação

---

## 🎯 Logs para Debug

### Ver logs do applet
```bash
# Opção 1: journalctl
journalctl --user -f | grep -i voicehub

# Opção 2: Executar manualmente com logs
RUST_LOG=debug /usr/local/bin/cosmic-applet-voicehub
```

### Ver logs do COSMIC Panel
```bash
journalctl --user -u cosmic-panel -f
```

### Ver processo rodando
```bash
ps aux | grep cosmic-applet-voicehub
```

---

## 🚀 Teste Rápido (1 minuto)

```bash
# 1. Verificar instalação
./test-applet.sh

# 2. Abrir COSMIC Settings e adicionar VoiceHub ao panel

# 3. Clicar no ícone do panel → popup abre

# 4. Clicar "Iniciar Gravação" → indicador fica vermelho

# 5. Clicar "Parar Gravação" → volta ao normal

# 6. Clicar "Copiar" → texto vai para clipboard

# 7. Fechar popup → clique novamente no ícone
```

**Se tudo isso funcionar**: ✅ **APPLET 100% FUNCIONAL (UI)**

**Próximo passo**: Implementar STT! 🎤

---

> **IMPORTANTE**: A falta de transcrição real é **esperada** e **normal**.  
> O applet está funcionando perfeitamente dentro do escopo atual.
