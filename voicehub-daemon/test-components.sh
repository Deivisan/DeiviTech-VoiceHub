#!/usr/bin/env bash
# 🦞 Teste de Componentes do VoiceHub Daemon (sem GUI)

set -e

echo "🦞 VoiceHub Daemon - Teste de Componentes"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 1. Verificar compilação
echo "📦 1. Verificando binário compilado..."
if [ ! -f "./target/release/voicehub-daemon" ]; then
    echo "❌ Binário não encontrado em ./target/release/voicehub-daemon"
    echo "   Compile com: cargo build --release"
    exit 1
fi
echo "✅ Binário encontrado ($(du -h ./target/release/voicehub-daemon | cut -f1))"

# 2. Verificar dependências
echo ""
echo "📋 2. Verificando dependências runtime..."

# ydotool
if ! command -v ydotool &> /dev/null; then
    echo "❌ ydotool não instalado (necessário para injeção de texto)"
    exit 1
fi
echo "✅ ydotool: $(ydotool --version 2>&1 | head -1)"

# ydotoold service
if systemctl --user is-active ydotool &> /dev/null; then
    echo "✅ ydotoold service: ativo (user)"
elif sudo systemctl is-active ydotoold &> /dev/null 2>&1; then
    echo "✅ ydotoold service: ativo (system)"
else
    echo "❌ ydotoold service não está rodando"
    echo "   Inicie: systemctl --user enable --now ydotool"
    exit 1
fi

# input group (para evdev hotkeys)
if groups | grep -q input; then
    echo "✅ Usuário no grupo 'input' (evdev hotkeys funcionarão)"
else
    echo "⚠️  Usuário NÃO está no grupo 'input'"
    echo "   Hotkeys Super+H NÃO funcionarão!"
    echo "   Corrija: sudo usermod -a -G input $USER && logout/login"
fi

# webkit2gtk
if ldconfig -p | grep -q webkit2gtk-4.1; then
    echo "✅ webkit2gtk-4.1: instalado"
else
    echo "❌ webkit2gtk-4.1 não encontrado"
    exit 1
fi

# 3. Testar ydotool (injeção simples)
echo ""
echo "🔧 3. Testando ydotool (injeção de texto)..."
echo "   Injetando texto 'VoiceHub Test' em 3 segundos..."
echo "   Foque um editor de texto AGORA!"
sleep 1
echo "   3..."
sleep 1
echo "   2..."
sleep 1
echo "   1..."
echo "VoiceHub Test ydotool OK" | ydotool type --file - 2>/dev/null
echo "✅ Injeção ydotool executada (verifique se apareceu no editor)"

# 4. Verificar estrutura do código
echo ""
echo "📄 4. Verificando arquivos fonte..."
for file in src/main.rs src/hotkey.rs src/speech.rs src/inject.rs; do
    if [ -f "$file" ]; then
        lines=$(wc -l < "$file")
        echo "✅ $file ($lines linhas)"
    else
        echo "❌ $file não encontrado"
    fi
done

# 5. Símbolos do binário (verificar se hotkey libs estão linkadas)
echo ""
echo "🔍 5. Verificando símbolos do binário..."
if ldd ./target/release/voicehub-daemon 2>/dev/null | grep -q webkit; then
    echo "✅ webkit2gtk linkado"
else
    echo "⚠️  webkit2gtk pode não estar linkado corretamente"
fi

if nm ./target/release/voicehub-daemon 2>/dev/null | grep -q "hotkey\|evdev"; then
    echo "✅ Símbolos de hotkey encontrados"
else
    echo "⚠️  Símbolos de hotkey não detectados (pode ser normal se stripped)"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ TESTE DE COMPONENTES CONCLUÍDO"
echo ""
echo "📍 PRÓXIMO PASSO: Teste end-to-end manual"
echo "   Execute: ./test-daemon.sh"
echo "   Instruções:"
echo "   1. Abra cosmic-edit (ou outro editor)"
echo "   2. Foque no campo de texto"
echo "   3. Pressione Super+H → fale → Super+H"
echo "   4. Texto deve aparecer automaticamente"
echo ""
