# 🎤 VoiceHub - Estado Atual

> **Data:** 14/02/2026
> **Status:** 🟡 EM DESENVOLVIMENTO

---

## 🏗️ Estrutura Atual

```
DeiviTech-VoiceHub/
├── legacy/              # ✅ Servidor web Bun (FUNCIONANDO)
│   ├── src/server.ts   # Servidor HTTP na porta 5001
│   └── src/public/     # Interface HTML/CSS/JS
│
├── src/                # ❌ Applet COSMIC (NÃO FUNCIONANDO)
│   ├── main.rs        # Entry point Rust
│   └── app.rs         # Applet de desktop
│
└── voicehub.sh         # ✅ Script principal (servidor web)
```

---

## ✅ Funcionando

### Servidor Web (legacy)
- **Comando:** `voicehub start`
- **URL:** http://localhost:5001
- **Stack:** Bun + Web Speech API
- **Navegador:** Chrome/Edge (suporte completo)

```bash
voicehub start   # Iniciar servidor
voicehub stop    # Parar servidor
voicehub status  # Ver status
voicehub log     # Ver logs
```

---

## ❌ Não Funcionando

### Applet COSMIC Desktop
- **Problema:** Compila mas não inicializa corretamente
- **Stack:** Rust + libcosmic
- **Status:** Precisa de debug

**Para testar manualmente:**
```bash
cd ~/Projetos/DeiviTech-VoiceHub
cargo run --release
```

---

## 📋 Próximos Passos

1. [ ] Resolver applet COSMIC (debugar inicialização)
2. [ ] Adicionar ydotool para injeção de texto
3. [ ] Melhorar interface web
4. [ ] Adicionar mais idiomas

---

## 📌 Notas

- O servidor web é a forma principal de uso atualmente
- O applet COSMIC é um projeto paralelo que precisa de trabalho
- Web Speech API requer HTTPS ou localhost (não funciona em http://IP)
