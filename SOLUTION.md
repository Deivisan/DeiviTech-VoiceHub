# 🦞 VoiceHub - Análise Completa e Soluções

> **Data:** 15/02/2026  
> **Analisado por:** DevSan AGI  
> **Status:** 🔍 Análise Completa + Soluções Propostas

---

## 📊 Estado Atual

### ✅ Funcionando
- **Servidor Web** (`localhost:5001`) - Bun + Web Speech API
- **Interface UI** - Transcrição em tempo real
- **Compilação Applet COSMIC** - Binário `cosmic-applet-voicehub` gerado
- **ydotool** - Instalado e configurado

### ❌ Problemas Identificados

#### 1. 🐛 Bug de "Ressurreição" de Texto (Interface Web)

**Problema:**
Quando você edita uma transcrição e continua falando, o texto editado volta ao estado anterior.

**Causa Raiz:**
```javascript
// Linha 137 em legacy/src/public/app.js
this.accumulatedTranscript = document.getElementById('editor').value || '';
```

A cada nova gravação, `accumulatedTranscript` é setado para o valor **atual** do editor (que pode conter texto editado antigo). Quando você fala de novo, o `lastProcessedIndex` **não** considera que você editou manualmente - ele simplesmente **adiciona** texto novo ao acumulado.

**Como Reproduzir:**
1. Falar: "teste um"
2. Editar manualmente para: "teste dois" 
3. Continuar falando: "teste três"
4. Resultado bugado: "teste dois teste três" + **"teste um"** ressuscita

**Solução:**

```javascript
startRecording() {
    // ...
    
    // FIX: ZERAR o accumulated ao iniciar nova sessão de gravação
    // Se o editor tem conteúdo, ele deve ser PRESERVADO mas NÃO reprocessado
    const editorContent = document.getElementById('editor').value;
    
    if (editorContent.trim()) {
        // Usuário editou manualmente - adicionar separador visual
        this.accumulatedTranscript = editorContent + '\n\n--- Nova Fala ---\n\n';
    } else {
        // Editor vazio - iniciar limpo
        this.accumulatedTranscript = '';
    }
    
    this.lastProcessedIndex = 0; // Reset tracking
    
    // ...
}
```

---

#### 2. 🎯 Separação de Falas Únicas

**Problema Atual:**
Todas as falas ficam no mesmo bloco de texto. Se você pausar e falar de novo, tudo vira um blob gigante.

**Solução:**
Criar **sessões de fala separadas** visualmente:

```javascript
class VoiceHub {
    constructor() {
        // ...
        this.sessionHistory = []; // Array de sessões
        this.currentSessionId = 0;
    }
    
    startRecording() {
        // Criar nova sessão
        this.currentSessionId++;
        this.sessionHistory.push({
            id: this.currentSessionId,
            timestamp: new Date().toLocaleString('pt-BR'),
            transcript: ''
        });
        
        // ...
    }
    
    // Update em onresult
    recognition.onresult = (event) => {
        // ... processar transcript ...
        
        // Salvar na sessão atual
        const currentSession = this.sessionHistory[this.sessionHistory.length - 1];
        currentSession.transcript = this.accumulatedTranscript;
        
        // Renderizar TODAS as sessões
        this.renderSessions();
    }
    
    renderSessions() {
        const editor = document.getElementById('editor');
        const rendered = this.sessionHistory.map(session => 
            `[${session.timestamp}]\n${session.transcript}`
        ).join('\n\n━━━━━━━━━━━━━━━━━━━━━━━\n\n');
        
        editor.value = rendered;
    }
}
```

**Resultado Visual:**
```
[15/02/2026 04:30:15]
Esta é minha primeira fala.

━━━━━━━━━━━━━━━━━━━━━━━

[15/02/2026 04:30:45]
Esta é minha segunda fala, completamente separada.

━━━━━━━━━━━━━━━━━━━━━━━
```

---

#### 3. 🦞 Applet COSMIC Não Aparece no Painel

**Investigação:**

O applet **compila** e **executa** (processo `30049` rodando), mas **não aparece visualmente** no painel COSMIC.

**Possíveis Causas:**

1. **Desktop Entry Incorreto/Ausente**
   - Caminho: `/usr/share/applications/com.deivisan.voicehub.desktop`
   - COSMIC precisa de metadata específico

2. **Registro de Applet Ausente**
   - COSMIC applets precisam de arquivo `.cosmic-applet` em `/usr/share/cosmic/applets/`
   
3. **Ícone Não Encontrado**
   - COSMIC procura ícones em `/usr/share/icons/hicolor/`

**Documentação Oficial:**
Segundo [libcosmic docs](https://pop-os.github.io/libcosmic-book/):
- Applets COSMIC precisam de **plugin descriptor** (`.desktop` file)
- Devem implementar `cosmic::applet::run::<YourApplet>()`
- Ícone deve usar `cosmic::applet::icon_button()`

**Verificação Atual:**

```bash
# Arquivo desktop existe?
ls -la ~/.local/share/applications/com.deivisan.voicehub.desktop
ls -la /usr/share/applications/com.deivisan.voicehub.desktop

# Diretório de applets COSMIC
ls -la ~/.local/share/cosmic/applets/
ls -la /usr/share/cosmic/applets/
```

**Solução:**

Criar arquivo `.desktop` correto:

```desktop
[Desktop Entry]
Type=Application
Name=VoiceHub
Comment=Voice dictation applet for COSMIC
Exec=/home/deivi/Projetos/DeiviTech-VoiceHub/target/release/cosmic-applet-voicehub
Icon=audio-input-microphone-symbolic
Terminal=false
Categories=COSMIC;Applet;Utility;
X-COSMIC-AppletId=com.deivisan.voicehub
```

E criar symlink no diretório de applets:
```bash
mkdir -p ~/.local/share/cosmic/applets/
ln -s /home/deivi/Projetos/DeiviTech-VoiceHub/target/release/cosmic-applet-voicehub \
      ~/.local/share/cosmic/applets/com.deivisan.voicehub
```

**PORÉM:**

Segundo os docs do COSMIC e exemplos de terceiros, applets devem ser **registrados via COSMIC Settings**, não automaticamente. O applet está rodando, mas precisa ser **adicionado manualmente** via:

`COSMIC Settings → Desktop → Panel → Applets → Add`

Se ele **não aparecer na lista**, é porque:
- Desktop entry está em local errado
- Nome do applet está incorreto no código Rust
- COSMIC não recarregou a lista de applets disponíveis

**Teste Rápido:**
```bash
# Recarregar COSMIC Panel
cosmic-panel --replace &
```

---

## 🚀 Solução Universal: Ditado Global Cross-Desktop

**Problema:**
Applet COSMIC é específico do COSMIC DE. Você quer **ditado em qualquer tela, qualquer DE**.

**Arquitetura Proposta:**

```
┌─────────────────────────────────────────────────┐
│  🎤 VoiceHub Daemon (Background)                │
│                                                 │
│  • Servidor HTTP local (porta 5001)            │
│  • WebView headless (Web Speech API)           │
│  • Hotkey listener global (evdev)              │
│  • ydotool injection                           │
│  • System tray icon (cross-desktop)            │
└─────────────────────────────────────────────────┘
           ↓                    ↓
    Hotkey (Super+H)       Tray Click
           ↓                    ↓
    Start Recording      Show/Hide Interface
           ↓                    
    Inject Text (ydotool)
```

**Stack Tecnológica:**

- **Backend Daemon:** Rust (Tauri ou puro)
- **Frontend UI:** Web (React/Svelte) via localhost
- **Hotkey Global:** `evdev` (Linux) ou `global-hotkey` crate
- **System Tray:** `tray-icon` crate (cross-desktop)
- **Text Injection:** `ydotool` (Wayland) + `xdotool` (X11)
- **Speech API:** WebView headless com Web Speech API

**Vantagens:**
✅ Funciona em **qualquer DE** (GNOME, KDE, XFCE, COSMIC, i3, etc.)
✅ Hotkey global **funciona mesmo sem janela aberta**
✅ Interface web acessível via localhost
✅ Zero dependências de desktop-specific APIs

**Implementação:**

```rust
// src/daemon.rs
use tauri::{AppHandle, Manager};
use global_hotkey::{GlobalHotKeyManager, hotkey::HotKey};
use tray_icon::TrayIconBuilder;

pub struct VoiceHubDaemon {
    hotkey_manager: GlobalHotKeyManager,
    is_recording: bool,
}

impl VoiceHubDaemon {
    pub fn new() -> Self {
        let hotkey_manager = GlobalHotKeyManager::new().unwrap();
        
        // Registrar Super+H
        let hotkey = HotKey::new(Some(Modifiers::SUPER), Code::KeyH);
        hotkey_manager.register(hotkey).unwrap();
        
        Self {
            hotkey_manager,
            is_recording: false,
        }
    }
    
    pub async fn handle_hotkey(&mut self) {
        if self.is_recording {
            self.stop_recording().await;
        } else {
            self.start_recording().await;
        }
    }
    
    async fn start_recording(&mut self) {
        self.is_recording = true;
        // Trigger Web Speech API via WebView
        // ...
    }
    
    async fn stop_recording(&mut self) {
        self.is_recording = false;
        // Inject text via ydotool
        let transcript = self.get_transcript();
        ydotool_inject(&transcript).await;
    }
}

async fn ydotool_inject(text: &str) {
    tokio::process::Command::new("ydotool")
        .args(["type", text])
        .spawn()
        .expect("ydotool failed");
}
```

**Execução:**
```bash
# Iniciar daemon (background)
voicehub-daemon &

# Acessar interface web
xdg-open http://localhost:5001

# Usar hotkey global
Super+H # Onde quer que esteja no sistema
```

---

## 📋 Roadmap de Implementação

### Fase 1: Fix Interface Web (2 horas)
- [x] Analisar código atual
- [ ] Corrigir bug de ressurreição de texto
- [ ] Implementar separação de sessões de fala
- [ ] Testar em diferentes idiomas

### Fase 2: Debugging Applet COSMIC (1 hora)
- [ ] Verificar desktop entry
- [ ] Criar arquivo de metadata correto
- [ ] Testar registro manual no COSMIC Settings
- [ ] Se não funcionar, documentar limitações

### Fase 3: Daemon Universal (4-6 horas)
- [ ] Criar projeto Rust standalone
- [ ] Implementar hotkey listener global (evdev)
- [ ] Integrar ydotool injection
- [ ] Criar system tray icon
- [ ] Testar em COSMIC, GNOME e i3

### Fase 4: Polish & Deploy (2 horas)
- [ ] Criar instalador .deb/.rpm
- [ ] Documentação completa
- [ ] Testes em múltiplos DEs
- [ ] Release pública

---

## 🎯 Próximos Passos Imediatos

1. **Corrigir Interface Web** (mais simples, impact imediato)
2. **Criar daemon universal** (solução definitiva)
3. **Applet COSMIC** (opcional, nice-to-have)

Quer que eu implemente qual parte primeiro? 🦞
