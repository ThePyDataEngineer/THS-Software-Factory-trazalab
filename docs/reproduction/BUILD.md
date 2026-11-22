---
id: REPRO-BUILD
title: "Guía Reproducción — Build"
version: "0.1.0"
status: "VIGENTE"
---

# Reproducción Completa — Build

## Toolchain Requerida

```powershell
rustup toolchain install 1.98.1
rustup default 1.98.1
rustup target add wasm32-unknown-unknown --toolchain 1.98.1
cargo install wasm-pack@0.15.1 --locked
```

## Verificación Entorno

```powershell
rustc --version        # rustc 1.98.1
cargo --version        # cargo 1.98.1
rustup target list --installed | Select-String wasm32
wasm-pack --version    # wasm-pack 0.15.1
```

## Build Núcleo (trazalab-core)

```powershell
cd I:\THS Software Factory - TrazaLab
cargo build -p trazalab-core --release
cargo test -p trazalab-core --all-targets
```

## Build Wasm (trazalab-wasm) — wasm-pack (DEC-006)

```powershell
cd I:\THS Software Factory - TrazaLab\crates\trazalab-wasm
wasm-pack build --release --target web --out-dir ../../web/pkg
```

## Verificación Artefacto

```powershell
ls -la I:\THS Software Factory - TrazaLab\web\pkg\
# Debe contener: trazalab_wasm_bg.wasm, trazalab_wasm.js, trazalab_wasm.d.ts, package.json
Get-FileHash I:\THS Software Factory - TrazaLab\web\pkg\trazalab_wasm_bg.wasm -Algorithm SHA256
# Registrar hash en evidence/wasm-artifacts/
```

## Limpieza

```powershell
cargo clean
Remove-Item -Recurse -Force I:\THS Software Factory - TrazaLab\web\pkg -ErrorAction SilentlyContinue
```
