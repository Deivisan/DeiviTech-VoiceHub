#!/usr/bin/env bash
# 🦞 Script de teste do VoiceHub Daemon

set -e

echo "🦞 VoiceHub Daemon - Teste de Integração"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Verificar pré-requisitos
echo "📋 Verificando pré-requisitos..."

# ydotool
if ! command -v ydotool &> /dev/null; then
    echo "❌ ydotool não instalado"
    echo "   Instale: sudo pacman -S ydotool"
    exit 1
fi
echo "✅ ydotool instalado"

# ydotoold service
if ! systemctl --user is-active ydotoold &> /dev/null; then
    if ! sudo systemctl is-active ydotoold &> /dev/null; then
        echo "❌ ydotoold não está rodando"
        echo "   Inicie: sudo systemctl enable --now ydotoold"
        exit 1
    fi
fi
echo "✅ ydotoold rodando"

# input group
if ! groups | grep -q input; then
    echo "⚠️  Usuário não está no grupo 'input'"
    echo "   Adicione: sudo usermod -a -G input $USER"
    echo "   Depois faça logout e login novamente"
fi

echo ""
echo "🚀 Iniciando VoiceHub Daemon..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📍 INSTRUÇÕES:"
echo "   1. Abra um editor de texto (gedit, kate, mousepad, etc.)"
echo "   2. Clique no campo de texto para focar"
echo "   3. Pressione Super+H para iniciar gravação"
echo "   4. Fale algo em português (ex: 'olá mundo teste de voz')"
echo "   5. Pressione Super+H novamente para parar e injetar texto"
echo "   6. Pressione Ctrl+C aqui para sair"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Rodar daemon
RUST_LOG=info ./target/release/voicehub-daemon
