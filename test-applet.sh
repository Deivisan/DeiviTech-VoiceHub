#!/bin/bash
# 🧪 Script de teste do VoiceHub Applet

set -e

echo "🎤 VoiceHub - Teste de Instalação"
echo "=================================="
echo ""

# Cores
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Função de teste
test_check() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $1"
        return 0
    else
        echo -e "${RED}✗${NC} $1"
        return 1
    fi
}

# 1. Verificar binário
echo "📦 Verificando instalação..."
ls /usr/local/bin/cosmic-applet-voicehub &>/dev/null
test_check "Binário instalado"

# 2. Verificar desktop entry
ls /usr/share/applications/com.deivisan.voicehub.desktop &>/dev/null
test_check "Desktop entry criado"

# 3. Verificar COSMIC rodando
pgrep cosmic-panel &>/dev/null
test_check "COSMIC Panel rodando"

# 4. Verificar ydotool
systemctl is-active --quiet ydotoold
test_check "ydotool service ativo"

# 5. Verificar permissões
if groups | grep -q input; then
    echo -e "${GREEN}✓${NC} Usuário no grupo 'input'"
else
    echo -e "${YELLOW}⚠${NC} Usuário NÃO está no grupo 'input' (injeção pode falhar)"
fi

# 6. Testar execução do applet
echo ""
echo "🧪 Testando execução do applet..."
timeout 2 /usr/local/bin/cosmic-applet-voicehub &>/dev/null
if [ $? -eq 124 ]; then
    echo -e "${GREEN}✓${NC} Applet executa sem erros (timeout esperado)"
else
    echo -e "${YELLOW}⚠${NC} Applet terminou inesperadamente"
fi

# 7. Verificar config
if [ -f ~/.config/cosmic/com.deivisan.voicehub/v1/config ]; then
    echo -e "${GREEN}✓${NC} Arquivo de configuração encontrado"
else
    echo -e "${YELLOW}⚠${NC} Config ainda não foi criado (normal no primeiro uso)"
fi

echo ""
echo "📋 Resumo"
echo "========="
echo "Binário: $(ls -lh /usr/local/bin/cosmic-applet-voicehub | awk '{print $5}')"
echo "Desktop Entry: OK"
echo "COSMIC Desktop: $(echo $XDG_CURRENT_DESKTOP)"

echo ""
echo "🎯 Próximos Passos"
echo "=================="
echo "1. Abra COSMIC Settings → Desktop → Panel"
echo "2. Clique em 'Applets'"
echo "3. Procure por 'VoiceHub' e adicione ao panel"
echo "4. Clique no ícone do microfone para testar"
echo ""
echo "ℹ️  NOTA: A funcionalidade de reconhecimento de fala ainda não está"
echo "   implementada. A UI e os botões funcionam, mas não há captura de áudio."
echo ""
echo "📚 Veja INSTALL.md para mais detalhes"
