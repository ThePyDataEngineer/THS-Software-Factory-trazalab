---
id: REPRO-SERVE
title: "Guía Reproducción — Servidor Local"
version: "0.1.0"
status: "VIGENTE"
---

# Reproducción — Servidor Local (DEC-007)

## Decisión: miniserve v0.25.0

```powershell
cargo install miniserve@0.25.0 --locked
```

## Ejecución

```powershell
cd I:\THS Software Factory - Visu\web
miniserve --port 8080 --address 127.0.0.1 .
```

## Verificación de Servicio

```powershell
# 1. HTML
Invoke-WebRequest "http://127.0.0.1:8080/index.html" | Select-Object StatusCode, ContentType
# 2. JS bootstrap
Invoke-WebRequest "http://127.0.0.1:8080/index.js" | Select-Object StatusCode, ContentType
# 3. Wasm
Invoke-WebRequest "http://127.0.0.1:8080/pkg/visu_wasm_bg.wasm" | Select-Object StatusCode, ContentType
# 4. Glue JS
Invoke-WebRequest "http://127.0.0.1:8080/pkg/visu_wasm.js" | Select-Object StatusCode, ContentType
# 5. Hash Wasm servido coincide con evidence
Get-FileHash I:\THS Software Factory - Visu\web\pkg\visu_wasm_bg.wasm -Algorithm SHA256
```

## Exclusiones

**NO servir**: `agents/`, `THS-AGENTES-PILOTO-PAQUETE-v1.0/`, `evidence/` (salvo `.gitkeep`), `.hermes/`, `.git/`, `target/`, `crates/`.
