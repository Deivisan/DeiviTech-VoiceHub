#!/bin/bash
# 🎤 DeiviTech VoiceHub - Launcher Script

# Verificar se ydotool está disponível
if ! command -v ydotool &> /dev/null; then
    echo "⚠️  ydotool não encontrado. Instale com: sudo pacman -S ydotool"
    exit 1
fi

# Iniciar ydotoold se não estiver rodando
if ! pgrep -x "ydotoold" > /dev/null; then
    echo "🚀 Iniciando ydotoold..."
    sudo ydotoold &
    sleep 1
fi

# Caminho do binário
BINARY="$HOME/Projetos/DeiviTech-VoiceHub/src-tauri/target/release/deivitech-voicehub"

# Verificar se binário existe
if [ ! -f "$BINARY" ]; then
    echo "❌ Binário não encontrado em: $BINARY"
    echo "   Execute o build primeiro: cargo build --release"
    exit 1
fi

echo "🎤 Iniciando DeiviTech VoiceHub..."
echo "   Atalho global: Super+H (inicia/para gravação)"
echo "   Clique no tray icon para mostrar/ocultar"
echo ""

# Executar
exec "$BINARY"
