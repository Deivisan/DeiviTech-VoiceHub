#!/bin/bash
# 🚀 Script para iniciar ydotoold com permissões corretas

set -e

SOCKET_PATH="/tmp/.ydotool_socket"

echo "🔧 Configurando ydotoold..."

# Verificar se já está rodando
if pgrep -x ydotoold > /dev/null; then
    echo "⚠️  ydotoold já está rodando. Parando..."
    sudo pkill -x ydotoold
    sleep 1
fi

# Remover socket antigo se existir
if [ -e "$SOCKET_PATH" ]; then
    echo "🗑️  Removendo socket antigo..."
    sudo rm -f "$SOCKET_PATH"
fi

# Iniciar ydotoold
echo "▶️  Iniciando ydotoold..."
sudo ydotoold &

# Aguardar socket ser criado
echo "⏳ Aguardando socket..."
for i in {1..10}; do
    if [ -e "$SOCKET_PATH" ]; then
        break
    fi
    sleep 0.5
done

# Dar permissões ao socket
if [ -e "$SOCKET_PATH" ]; then
    echo "🔓 Configurando permissões..."
    sudo chmod 666 "$SOCKET_PATH"
    echo "✅ ydotoold configurado com sucesso!"
    echo ""
    echo "Socket: $SOCKET_PATH"
    ls -la "$SOCKET_PATH"
    echo ""
    echo "Você pode testar com:"
    echo "  export YDOTOOL_SOCKET=$SOCKET_PATH"
    echo "  ydotool type 'teste'"
else
    echo "❌ Erro: Socket não foi criado"
    exit 1
fi
