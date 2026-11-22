---
id: ADR-0001
title: "Estructura Workspace: Dos Crates (visu-core, visu-wasm)"
status: "ACEPTADO"
date: "2026-11-22"
deciders: ["ThePyDataEngineer"]
---

# ADR-0001: Estructura Workspace — Dos Crates

## Contexto

Separar **núcleo determinista** (Rust puro, testeable nativo, sin dependencias WASM) del **enlace WebAssembly** (cdylib, wasm-bindgen, web-sys).

## Decisión

Workspace Cargo con dos members:
- `crates/visu-core` — `lib`, edición 2021, target nativo only.
- `crates/visu-wasm` — `cdylib`, target `wasm32-unknown-unknown`, depende de `visu-core`.

## Consecuencias

**Positivas**:
- Tests unitarios rápidos en CI nativo (`cargo test -p visu-core`).
- Separación de preocupaciones: dominio vs glue.
- `visu-core` reutilizable en contextos no-WASM (CLI, server, property tests).

**Negativas**:
- Dos `Cargo.toml` que mantener.
- Sincronización de versiones via `workspace.package`.
- Frontera de tipos: `visu-wasm` serializa/deserializa tipos de `visu-core`.

## Alternativas Consideradas

1. **Un solo crate con `cfg(target_arch = "wasm32")`** — Rechazado: mezcla concerns, tests WASM lentos.
2. **Tres crates (core, wasm, web-host)** — Rechazado: `web/` es estático, no crate Rust.
3. **Dependencia externa (git/npm)** — Rechazado: acopla versiones, complica CI.
