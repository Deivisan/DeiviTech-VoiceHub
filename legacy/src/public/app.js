// 🦞 DeiviTech VoiceHub - Web Speech API Only (Bug-Free)

class VoiceHub {
    constructor() {
        this.isRecording = false;
        this.stream = null;
        this.recognition = null;
        this.startTime = null;
        this.durationInterval = null;
        this.audioContext = null;
        this.analyser = null;
        this.visualizerAnimationId = null;
        
        // FIX: Track processed results to avoid repetition
        this.lastProcessedIndex = 0;
        this.accumulatedTranscript = '';
        
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.updateStats();
        this.checkBrowserSupport();
        this.loadSession();
        this.setupTauriListeners();
    }

    setupTauriListeners() {
        // Check if running in Tauri (desktop app)
        if (window.__TAURI__) {
            const { listen } = window.__TAURI__.event;
            
            // Listen for global hotkey toggle event from Rust backend
            listen('toggle-recording', () => {
                if (this.isRecording) {
                    this.stopRecording();
                } else {
                    this.startRecording();
                }
            });

            // Show inject_text button only in Tauri desktop app
            const injectBtn = document.getElementById('injectBtn');
            if (injectBtn) {
                injectBtn.style.display = 'flex';
            }

            console.log('✅ Tauri global hotkey listener registered (Super+H)');
            console.log('✅ Inject text button enabled');
        }
    }

    setupEventListeners() {
        // Controls
        document.getElementById('recordBtn').addEventListener('click', () => this.startRecording());
        document.getElementById('stopBtn').addEventListener('click', () => this.stopRecording());
        document.getElementById('copyBtn').addEventListener('click', () => this.copyToClipboard());
        document.getElementById('clearBtn').addEventListener('click', () => this.clearEditor());
        document.getElementById('injectBtn').addEventListener('click', () => this.injectText());
        document.getElementById('confirmBtn').addEventListener('click', () => this.confirmCurrentSpeech());

        // Settings toggle - FIX: Actually toggle visibility
        document.getElementById('settingsToggle').addEventListener('click', () => {
            const panel = document.getElementById('settingsPanel');
            panel.classList.toggle('visible');
        });

        // Settings
        document.getElementById('language').addEventListener('change', () => {
            this.saveSettings();
        });

        // Editor updates
        document.getElementById('editor').addEventListener('input', () => {
            this.updateWordCount();
            this.saveSession();
        });

        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            // Ctrl/Cmd + Enter: Toggle recording
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                e.preventDefault();
                if (this.isRecording) {
                    this.stopRecording();
                } else {
                    this.startRecording();
                }
            }
            // Ctrl/Cmd + Shift + C: Copy
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'C') {
                e.preventDefault();
                this.copyToClipboard();
            }
            // Ctrl/Cmd + Shift + X: Clear (NO CONFIRMATION)
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'X') {
                e.preventDefault();
                this.clearEditor();
            }
            // Ctrl/Cmd + D: Confirmar fala atual (snapshot)
            if ((e.ctrlKey || e.metaKey) && e.key === 'd') {
                e.preventDefault();
                this.confirmCurrentSpeech();
            }
        });
    }

    checkBrowserSupport() {
        const hasSpeechRecognition = 'webkitSpeechRecognition' in window || 'SpeechRecognition' in window;
        
        if (!hasSpeechRecognition) {
            this.showToast('⚠️ Web Speech API não suportada. Use Chrome ou Edge.', 'warning');
        }

        if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
            this.showToast('❌ Microfone não disponível', 'error');
        }
    }

    async startRecording() {
        try {
            this.setStatus('Inicializando...', true);
            
            // Request microphone access
            this.stream = await navigator.mediaDevices.getUserMedia({ 
                audio: {
                    echoCancellation: true,
                    noiseSuppression: true,
                    autoGainControl: true,
                    sampleRate: 16000
                } 
            });

            this.isRecording = true;
            this.startTime = Date.now();
            this.updateDuration();

            // Setup audio visualizer
            this.setupVisualizer();

            // Reset transcript tracking - FIX: Separate sessions properly
            this.lastProcessedIndex = 0;
            
            // FIX: Se há conteúdo no editor, criar nova sessão separada
            const editorContent = document.getElementById('editor').value.trim();
            if (editorContent && !editorContent.endsWith('[interim]')) {
                // Adicionar separador visual entre sessões
                this.accumulatedTranscript = editorContent + '\n\n━━━━━━ Nova Fala ━━━━━━\n\n';
            } else {
                // Editor vazio ou apenas interim - iniciar limpo
                this.accumulatedTranscript = '';
            }

            // Start Web Speech API
            this.startWebSpeechRecording();

            // Update UI
            const recordBtn = document.getElementById('recordBtn');
            recordBtn.classList.add('recording');
            recordBtn.innerHTML = '<span>⏺️</span><span>Gravando...</span>';
            document.getElementById('stopBtn').disabled = false;
            
            this.setStatus('Gravando...', true);
            this.showToast('🎤 Gravando! Fale naturalmente.', 'success');

        } catch (error) {
            console.error('Error starting recording:', error);
            this.showToast('❌ Erro ao acessar microfone: ' + error.message, 'error');
            this.cleanup();
        }
    }

    startWebSpeechRecording() {
        if (!('webkitSpeechRecognition' in window) && !('SpeechRecognition' in window)) {
            this.showToast('❌ Web Speech API não suportada. Use Chrome/Edge.', 'error');
            this.cleanup();
            return;
        }

        const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
        this.recognition = new SpeechRecognition();
        
        const lang = document.getElementById('language').value;
        this.recognition.lang = lang;
        this.recognition.continuous = true;
        this.recognition.interimResults = true;
        this.recognition.maxAlternatives = 1;

        // FIX: Use closure to maintain state across events
        let currentInterim = '';

        this.recognition.onresult = (event) => {
            let newFinalText = '';
            let newInterimText = '';

            // FIX: Only process NEW results from lastProcessedIndex
            for (let i = this.lastProcessedIndex; i < event.results.length; i++) {
                const transcript = event.results[i][0].transcript;

                if (event.results[i].isFinal) {
                    newFinalText += transcript + ' ';
                    this.lastProcessedIndex = i + 1; // FIX: Update tracking index
                } else {
                    newInterimText += transcript;
                }
            }

            // FIX: Only append NEW final text to accumulated
            if (newFinalText) {
                this.accumulatedTranscript += newFinalText;
            }

            // Update editor - show accumulated + current interim
            const editor = document.getElementById('editor');
            if (newInterimText) {
                currentInterim = newInterimText;
                editor.value = this.accumulatedTranscript + '\n[interim] ' + currentInterim;
            } else {
                currentInterim = '';
                editor.value = this.accumulatedTranscript.trim();
            }

            this.updateWordCount();
            editor.scrollTop = editor.scrollHeight;
        };

        this.recognition.onend = () => {
            if (this.isRecording) {
                // Auto-restart if still recording
                this.recognition.start();
            } else {
                // Clean up interim text on final stop
                const editor = document.getElementById('editor');
                editor.value = this.accumulatedTranscript.trim();
                this.updateWordCount();
                this.saveSession();
            }
        };

        this.recognition.onerror = (event) => {
            console.error('Speech recognition error:', event.error);
            
            // Ignore "no-speech" errors (normal when user pauses)
            if (event.error !== 'no-speech' && event.error !== 'aborted') {
                this.showToast(`❌ Erro: ${event.error}`, 'error');
            }
        };

        this.recognition.start();
    }

    stopRecording() {
        // FIX: NO CONFIRMATION - just stop immediately
        this.isRecording = false;
        this.setStatus('Finalizando...', true);

        // Stop speech recognition
        if (this.recognition) {
            this.recognition.stop();
        }

        // Cleanup
        this.cleanup();

        // Update UI
        const recordBtn = document.getElementById('recordBtn');
        recordBtn.classList.remove('recording');
        recordBtn.innerHTML = '<span>⏺️</span><span>GRAVAR</span>';
        document.getElementById('stopBtn').disabled = true;

        this.setStatus('Pronto', false);
        this.showToast('⏹️ Gravação finalizada!', 'success');
        
        // Save final state
        this.saveSession();
    }

    cleanup() {
        // Stop duration timer
        if (this.durationInterval) {
            clearInterval(this.durationInterval);
            this.durationInterval = null;
        }

        // Stop visualizer
        if (this.visualizerAnimationId) {
            cancelAnimationFrame(this.visualizerAnimationId);
            this.visualizerAnimationId = null;
        }

        // Close audio context
        if (this.audioContext && this.audioContext.state !== 'closed') {
            this.audioContext.close();
        }

        // Stop microphone stream
        if (this.stream) {
            this.stream.getTracks().forEach(track => track.stop());
            this.stream = null;
        }

        this.isRecording = false;
    }

    setupVisualizer() {
        const canvas = document.getElementById('visualizer');
        const ctx = canvas.getContext('2d');

        this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
        this.analyser = this.audioContext.createAnalyser();
        
        const source = this.audioContext.createMediaStreamSource(this.stream);
        source.connect(this.analyser);

        this.analyser.fftSize = 256;
        const bufferLength = this.analyser.frequencyBinCount;
        const dataArray = new Uint8Array(bufferLength);

        const draw = () => {
            if (!this.isRecording) return;

            this.visualizerAnimationId = requestAnimationFrame(draw);

            this.analyser.getByteFrequencyData(dataArray);

            ctx.fillStyle = 'rgba(0, 0, 0, 0.2)';
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            const barWidth = (canvas.width / bufferLength) * 2.5;
            let barHeight;
            let x = 0;

            for (let i = 0; i < bufferLength; i++) {
                barHeight = (dataArray[i] / 255) * canvas.height;

                const gradient = ctx.createLinearGradient(0, canvas.height - barHeight, 0, canvas.height);
                gradient.addColorStop(0, '#9b59b6');
                gradient.addColorStop(1, '#4a90e2');

                ctx.fillStyle = gradient;
                ctx.fillRect(x, canvas.height - barHeight, barWidth, barHeight);

                x += barWidth + 1;
            }
        };

        draw();
    }

    updateDuration() {
        this.durationInterval = setInterval(() => {
            if (!this.isRecording) return;

            const elapsed = Math.floor((Date.now() - this.startTime) / 1000);
            const minutes = Math.floor(elapsed / 60);
            const seconds = elapsed % 60;

            document.getElementById('duration').textContent = 
                `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
        }, 1000);
    }

    updateWordCount() {
        const text = document.getElementById('editor').value.replace(/\n\[interim\].*$/, '');
        const words = text.trim().split(/\s+/).filter(w => w.length > 0);
        document.getElementById('wordCount').textContent = words.length;
    }

    updateStats() {
        this.updateWordCount();
        document.getElementById('duration').textContent = '00:00';
    }

    async copyToClipboard() {
        const text = document.getElementById('editor').value.replace(/\n\[interim\].*$/, '');

        if (!text.trim()) {
            this.showToast('⚠️ Nada para copiar', 'warning');
            return;
        }

        try {
            await navigator.clipboard.writeText(text);
            this.showToast('📋 Copiado!', 'success');
        } catch (error) {
            console.error('Copy error:', error);
            this.showToast('❌ Erro ao copiar', 'error');
        }
    }

    clearEditor() {
        const editor = document.getElementById('editor');

        // FIX: Limpar TUDO incluindo acumulado + RESET recognition
        editor.value = '';
        this.accumulatedTranscript = '';
        this.lastProcessedIndex = 0;
        
        // RESET recognition para nova sessão limpa
        if (this.recognition && this.isRecording) {
            this.recognition.stop();
            setTimeout(() => {
                if (this.isRecording) {
                    this.recognition.start();
                }
            }, 100);
        }
        
        this.updateWordCount();
        this.saveSession();
        this.showToast('✅ Editor limpo! Nova sessão iniciada.', 'success');
    }

    confirmCurrentSpeech() {
        // 🎯 SNAPSHOT IMUTÁVEL: Confirma texto atual e NUNCA sobrescreve
        const editor = document.getElementById('editor');
        const currentText = editor.value.replace(/\n\[interim\].*$/, '').trim();
        
        if (!currentText) {
            this.showToast('⚠️ Nenhum texto para confirmar', 'warning');
            return;
        }

        // Salvar texto confirmado (imutável) com separador simples
        this.accumulatedTranscript = currentText + '\n\n______________\n\n';
        
        // Reset recognition para nova sessão
        if (this.recognition && this.isRecording) {
            this.recognition.stop();
            this.lastProcessedIndex = 0;
            
            // Reiniciar recognition imediatamente para nova fala
            setTimeout(() => {
                if (this.isRecording) {
                    this.recognition.start();
                }
            }, 100);
        }
        
        // Atualizar editor com separador
        editor.value = this.accumulatedTranscript;
        this.updateWordCount();
        this.saveSession();
        
        this.showToast('✅ Fala confirmada! Texto anterior protegido.', 'success');
    }

    setStatus(text, active) {
        document.getElementById('statusText').textContent = text;
    }

    showToast(message, type = 'info') {
        const toast = document.getElementById('toast');
        toast.textContent = message;
        toast.className = `toast ${type} show`;

        setTimeout(() => {
            toast.classList.remove('show');
        }, 3000);
    }

    // Session Management (auto-save to localStorage)
    saveSession() {
        const sessionData = {
            text: document.getElementById('editor').value.replace(/\n\[interim\].*$/, ''),
            language: document.getElementById('language').value,
            timestamp: Date.now()
        };
        localStorage.setItem('voicehub_session', JSON.stringify(sessionData));
    }

    loadSession() {
        const saved = localStorage.getItem('voicehub_session');
        if (saved) {
            try {
                const sessionData = JSON.parse(saved);
                document.getElementById('editor').value = sessionData.text || '';
                document.getElementById('language').value = sessionData.language || 'pt-BR';
                this.updateWordCount();
                
                if (sessionData.text) {
                    this.showToast('📄 Sessão restaurada', 'success');
                }
            } catch (error) {
                console.error('Error loading session:', error);
            }
        }
    }

    saveSettings() {
        const settings = {
            language: document.getElementById('language').value
        };
        localStorage.setItem('voicehub_settings', JSON.stringify(settings));
    }

    // Tauri-only: Inject text into system using ydotool
    async injectText() {
        if (!window.__TAURI__) {
            this.showToast('❌ Função disponível apenas no app desktop', 'error');
            return;
        }

        const text = document.getElementById('editor').value.trim();
        
        if (!text) {
            this.showToast('⚠️ Nenhum texto para digitar', 'warning');
            return;
        }

        try {
            const { invoke } = window.__TAURI__.core;
            
            // Call Tauri command inject_text
            const result = await invoke('inject_text', { text });
            
            this.showToast(`✅ ${result}`, 'success');
            console.log('✅ Text injected successfully:', result);
            
        } catch (error) {
            this.showToast(`❌ Erro: ${error}`, 'error');
            console.error('❌ Inject text failed:', error);
        }
    }
}

// Initialize on load
window.addEventListener('DOMContentLoaded', () => {
    window.voiceHub = new VoiceHub();
    console.log('🦞 DeiviTech VoiceHub initialized - Web Speech API Only');
});
