# TrazaLab

[![CI](https://github.com/ThePyDataEngineer/trazalab/actions/workflows/ci.yml/badge.svg)](https://github.com/ThePyDataEngineer/trazalab/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.98.1-orange.svg)](https://blog.rust-lang.org/2026/09/01/Rust-1.98.1.html)
[![Wasm Target](https://img.shields.io/badge/target-wasm32--unknown--unknown-lightgrey.svg)](https://rustwasm.github.io/)

> **TrazaLab** — TrazaLab es un visualizador web de algoritmos y estructuras de datos con motor Rust/WebAssembly. Es un proyecto propio inspirado en Visu, de iIonel, y no es una versión oficial ni está afiliado al original.

## Alcance v0.1 (Incremento 1)

- Arreglo editable de enteros en editor web
- Ordenamiento burbuja determinista ejecutado en Wasm
- Traza navegable: paso a paso, saltos a primer intercambio y estado final
- Validación de entrada: rechazo de no enteros con diagnóstico y recuperación
- Windows + Edge 158.0.4370.33 (Chrome/Firefox en WEB-01 posterior)

## Estructura

```
trazalab/
├── crates/
│   ├── trazalab-core/     # Núcleo: validación, ejecución, traza común (Rust puro)
│   └── trazalab-wasm/     # Adaptador Wasm: glue JS, runtime navegador
├── web/                   # Host estático: editor, visualización, controles
├── fixtures/              # Escenarios TRAZA-T02, T03, T04 + schemas
├── docs/                  # Contrato, ADRs, guías reproducción
├── evidence/              # Política + resúmenes sanitizados
├── scripts/               # ci-local.ps1 (réplica PowerShell de CI)
└── .github/               # CI, plantillas, gobernanza
```

## Reproducción Rápida

```powershell
# Prerrequisitos: Rust 1.98.1, wasm32-unknown-unknown, wasm-pack 0.15.1, miniserve 0.25.0
cd I:\THS Software Factory - TrazaLab
cargo fmt --all -- --check
cargo clippy -p trazalab-core --all-targets --locked
cargo test -p trazalab-core --locked
cargo build -p trazalab-wasm --target wasm32-unknown-unknown --locked
cd crates/trazalab-wasm
wasm-pack build --release --target web --out-dir ../../web/pkg
cd ..\..\web
miniserve --port 8080 --address 127.0.0.1 .
# Abrir http://127.0.0.1:8080 en Edge → ejecutar TRAZA-T01 a T04
```

Ver [docs/reproduction/BUILD.md](docs/reproduction/BUILD.md), [TEST.md](docs/reproduction/TEST.md), [SERVE.md](docs/reproduction/SERVE.md).

## Documentación

- [Contrato de Traza](docs/contract/TRACE-CONTRACT.md)
- [ADR-0001: Estructura Workspace](docs/adr/0001-workspace-structure.md)
- [ADR-0002: Enlace Wasm](docs/adr/0002-wasm-linkage.md)
- [ADR-0003: Estrategia CI](docs/adr/0003-ci-strategy.md)
- [ADR-0004: Política Evidencia](docs/adr/0004-evidence-policy.md)

## Estado

**v0.1.0** — Código de producto implementado, repositorio inicializado, CI configurada.
**Identidad**: TrazaLab — proyecto propio inspirado en Visu, de iIonel; no versión oficial.
Decisiones DEC-001 a DEC-008 resueltas (ver matriz en docs).
