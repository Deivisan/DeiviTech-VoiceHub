# 🎤 VoiceHub - Guia Rápido

Sistema profissional de ditado de voz para Linux com **2 modos de uso**.

---

## 🌐 Modo 1: Web App (Navegador)

**Uso:** Ditado no navegador, edição manual, exportar texto

```bash
voicehub start    # Inicia servidor em http://localhost:5001
```

**Acesse:** `http://localhost:5001` no **Chrome ou Edge**

### Como Usar:
1. Clique em **"GRAVAR"** 🔴
2. Fale em português
3. Clique em **"PARAR"** ⏸️
4. **Edite manualmente** se necessário
5. Clique em **"Confirmar Fala"** ✅ (ou `Ctrl+D`) para proteger texto
6. Continue falando → novo texto aparece abaixo do separador `______________`
7. Clique em **"COPIAR"** 📋 para copiar tudo

### Atalhos de Teclado:
- `Ctrl+R` → Gravar/Parar
- `Ctrl+D` → Confirmar Fala (protege texto editado)
- `Ctrl+C` → Copiar texto

### Comandos CLI:
```bash
voicehub start     # Iniciar servidor
voicehub stop      # Parar servidor
voicehub restart   # Reiniciar
voicehub status    # Ver status
voicehub log       # Ver logs em tempo real
```

---

## 🚀 Modo 2: Daemon MAX (System-Wide)

**Uso:** Dite em **QUALQUER aplicação** com Super+H global

```bash
voicehub max      # Roda daemon nativo
```

### Como Usar:
1. Execute `voicehub max` em um terminal
2. Abra **qualquer editor** (cosmic-edit, kate, LibreOffice, etc.)
3. Clique no campo de texto para focar
4. **Pressione `Super+H`** → fale em português
5. **Pressione `Super+H`** novamente → texto injetado automaticamente!
6. `Ctrl+C` no terminal para sair

### Pré-requisitos (Daemon):
```bash
# Verificar se está tudo OK
cd ~/Projetos/DeiviTech-VoiceHub/voicehub-daemon
./test-components.sh
```

**Checklist:**
- ✅ ydotool service ativo (`systemctl --user status ydotool`)
- ✅ Usuário no grupo `input` (`groups | grep input`)
- ✅ webkit2gtk-4.1 instalado (`pacman -Q webkit2gtk-4.1`)
- ✅ Binário compilado (`ls target/release/voicehub-daemon`)

---

## 🔧 Troubleshooting

### Web App não abre:
```bash
voicehub stop
voicehub start
# Se persistir:
lsof -ti :5001 | xargs kill  # Mata processo na porta
voicehub start
```

### Daemon Super+H não funciona:
```bash
# 1. Verificar ydotool
systemctl --user status ydotool

# 2. Verificar grupo input
groups | grep input
# Se não aparecer:
sudo usermod -a -G input $USER
# IMPORTANTE: Faça logout/login depois!

# 3. Testar injeção manual
echo "teste" | ydotool type --file -
# Deve digitar "teste" na janela focada
```

### Microfone não funciona (Web App):
- Use **Chrome ou Edge** (Firefox tem suporte limitado)
- Permita acesso ao microfone quando solicitado
- Verifique microfone em `chrome://settings/content/microphone`

---

## 📊 Comparação: Web vs Daemon

| Feature | Web App | Daemon MAX |
|---------|---------|------------|
| **Onde funciona** | Navegador | Qualquer app |
| **Hotkey** | Botões | Super+H |
| **Edição** | Manual no editor | Injeta direto |
| **Confirmar texto** | ✅ Botão | ❌ Não precisa |
| **Export** | Copiar/Salvar | Texto já no app |
| **Setup** | Zero config | ydotool + evdev |

---

## 🎯 Workflow Recomendado

### Para textos longos (artigos, docs):
1. Use **Web App** (`voicehub start`)
2. Fale → Parar → Editar → Confirmar
3. Repita até terminar
4. Copiar tudo de uma vez

### Para ditado rápido (emails, chat):
1. Use **Daemon MAX** (`voicehub max`)
2. Foque no campo → Super+H → fale → Super+H
3. Texto injetado instantaneamente!

---

## 📚 Mais Informações

- **README completo:** `~/Projetos/DeiviTech-VoiceHub/README.md`
- **Arquitetura daemon:** `~/Projetos/DeiviTech-VoiceHub/voicehub-daemon/ARCHITECTURE.md`
- **Teste componentes:** `~/Projetos/DeiviTech-VoiceHub/voicehub-daemon/test-components.sh`

---

**🦞 Criado por DevSan AGI - 2026**
