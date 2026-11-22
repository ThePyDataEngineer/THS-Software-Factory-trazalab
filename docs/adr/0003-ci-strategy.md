---
id: ADR-0003
title: "Estrategia CI: Compilación ≠ Prueba en Navegador"
status: "ACEPTADO"
date: "2026-11-22"
deciders: ["ThePyDataEngineer"]
---

# ADR-0003: CI Básica Sin Secretos, Build Wasm ≠ Test Navegador

## Principios

1. **Permisos mínimos**: `contents: read`, `actions: read`. Sin secretos.
2. **Compilación Wasm en CI** verifica que el código compila y genera artefactos.
3. **Prueba funcional en navegador (VISU-T01–T04) es fase separada (WEB-01)** — no sustituida por build.
4. **Runner**: `ubuntu-latest` para CI base.
5. **No despliegue, no publicación, no etiquetas, no jobs privilegiados** desde PR no confiable.

## Jobs CI

| Job | Comando | Qué Verifica | Qué NO Verifica |
|:---|:---|:---|:---|
| `fmt` | `cargo fmt --all -- --check` | Formato uniforme | Corrección lógica |
| `clippy` | `cargo clippy -p visu-core --all-targets --locked` | Lints en núcleo | Lints en wasm (target aparte) |
| `test-core` | `cargo test -p visu-core --locked` | Lógica determinista, validación, traza | Ejecución Wasm, UI, navegador |
| `wasm-build` | `cargo build -p visu-wasm --target wasm32-unknown-unknown --locked` + `wasm-pack` | Compilación Wasm + glue JS | Carga en navegador, VISU-T01–T04 |

## Exclusiones Explícitas

- `wasm-pack test --headless` — no en CI básica (WEB-01).
- Despliegue a GitHub Pages, Netlify, Vercel, etc.
- `cargo publish`, `gh release create`.
- Secrets (`CARGO_REGISTRY_TOKEN`, `NPM_TOKEN`, etc.).
