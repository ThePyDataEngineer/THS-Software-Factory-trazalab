---
id: ADR-0002
title: "Enlace Wasm: wasm-pack vs cargo build + wasm-bindgen-cli"
status: "ACEPTADO"
date: "2026-11-22"
deciders: ["ThePyDataEngineer"]
---

# ADR-0002: Herramienta de Enlace Wasm (DEC-006)

## Contexto

`visu-wasm` produce `cdylib` → necesita generar glue JS (`.js`, `.d.ts`, `package.json`) para `web/`.

## Decisión

**Opción A: `wasm-pack` v0.15.1**

Comando:
```bash
cd crates/visu-wasm
wasm-pack build --release --target web --out-dir ../../web/pkg
```

## Criterios de Decisión

1. Paridad de artefactos: produce `.wasm`, `.js`, `.d.ts`, `package.json` que consume `web/index.js`.
2. Reproducibilidad en CI (ubuntu-latest) y local (Windows MSVC).
3. Versión fijada y verificable (`cargo install wasm-pack@0.15.1 --locked`).
4. Simplicidad de `scripts/ci-local.ps1` réplica.

## Alternativa Evaluada (Opción B)

`cargo build --target wasm32-unknown-unknown --release` + `wasm-bindgen-cli` v0.2.98.
Mantenida como fallback documentada en `scripts/ci-local.ps1` param `-WasmTool cargo-bindgen`.

## Estado

**DEC-006 RESUELTA** — `wasm-pack` v0.15.1 seleccionado. CI y scripts actualizados en consecuencia.
