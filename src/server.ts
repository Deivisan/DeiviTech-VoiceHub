#!/usr/bin/env bun

/**
 * 🦞 DeiviTech VoiceHub - Bun HTTP Server
 * Servidor local ultra-rápido para interface Web Speech API
 * 
 * Features:
 * - Serve arquivos estáticos (HTML/CSS/JS)
 * - Hot reload (futuro)
 * - CORS habilitado
 * - Zero dependências externas
 */

const PORT = 5001;
const PUBLIC_DIR = import.meta.dir + '/public';

console.log(`
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🦞  DeiviTech VoiceHub Server
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🌐  URL: http://localhost:${PORT}
📁  Diretório: ${PUBLIC_DIR}
⚡  Runtime: Bun ${Bun.version}
🎤  Método: Web Speech API (Chrome/Edge)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
`);

const server = Bun.serve({
    port: PORT,
    
    async fetch(req, server) {
        const url = new URL(req.url);
        
        // Health check endpoint
        if (url.pathname === '/health') {
            return new Response(JSON.stringify({
                status: 'ok',
                uptime: process.uptime(),
                timestamp: Date.now()
            }), {
                headers: { 
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
            });
        }
        
        // Servir arquivos estáticos
        return serveStatic(url.pathname);
    },
});

async function serveStatic(pathname: string) {
    // Map root to index.html
    const filePath = pathname === '/' ? '/index.html' : pathname;
    const fullPath = PUBLIC_DIR + filePath;
    
    try {
        const file = Bun.file(fullPath);
        const exists = await file.exists();
        
        if (!exists) {
            return new Response('404 Not Found', { 
                status: 404,
                headers: { 'Content-Type': 'text/plain' }
            });
        }
        
        return new Response(file, {
            headers: {
                'Content-Type': getContentType(filePath),
                'Access-Control-Allow-Origin': '*',
                'Cache-Control': 'no-cache' // Facilita desenvolvimento
            }
        });
    } catch (error) {
        console.error(`Error serving ${pathname}:`, error);
        return new Response('500 Internal Server Error', { 
            status: 500,
            headers: { 'Content-Type': 'text/plain' }
        });
    }
}

function getContentType(filePath: string): string {
    const ext = filePath.split('.').pop()?.toLowerCase();
    
    const mimeTypes: Record<string, string> = {
        'html': 'text/html; charset=utf-8',
        'css': 'text/css; charset=utf-8',
        'js': 'application/javascript; charset=utf-8',
        'json': 'application/json; charset=utf-8',
        'png': 'image/png',
        'jpg': 'image/jpeg',
        'jpeg': 'image/jpeg',
        'gif': 'image/gif',
        'svg': 'image/svg+xml',
        'ico': 'image/x-icon',
        'woff': 'font/woff',
        'woff2': 'font/woff2',
        'ttf': 'font/ttf',
        'webm': 'video/webm',
        'mp4': 'video/mp4'
    };
    
    return mimeTypes[ext || ''] || 'application/octet-stream';
}

console.log(`✅ Server rodando em http://localhost:${PORT}`);
console.log(`🎙️  Pronto para transcrição!\n`);
console.log(`💡 Dica: Use Chrome ou Edge para melhor suporte\n`);

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('\n👋 Desligando servidor...');
    process.exit(0);
});
