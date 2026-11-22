---
id: CONTRIBUTING
title: "Guía de Contribución — Visu"
version: "0.1.0"
status: "VIGENTE"
---

# Guía de Contribución

## Convención de Commits

Usamos **Conventional Commits 1.0**:

```
<type>[scope]: <description>

[body]

[footer]
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`, `build`, `revert`.

**Scopes**: `core`, `wasm`, `web`, `fixtures`, `docs`, `ci`, `scripts`, `deps`.

Ejemplo: `feat(core): add adaptive early-exit to bubble sort`

## Flujo de Pull Request

1. Fork → rama `type/scope-short-desc` (ej: `feat/core/adaptive-exit`).
2. Commits atómicos, mensajes convencionales.
3. PR contra `main` con plantilla completada.
4. CI pasa (fmt, clippy, test-core, wasm-build).
5. Review: al menos 1 approval, 0 changes requested.
6. Squash merge (historia limpia).

## Requisitos de Calidad

- `cargo fmt --all -- --check` ✅
- `cargo clippy -p visu-core --all-targets --locked -- -D warnings` ✅
- `cargo test -p visu-core --locked` ✅
- `cargo build -p visu-wasm --target wasm32-unknown-unknown --locked` ✅
- Enlace Wasm (wasm-pack) genera `web/pkg/` completo
- Tests nuevos para funcionalidad nueva
- Docs actualizadas: README, doc comments, `docs/` si cambia contrato/ADR

## Seguridad

Ver [SECURITY.md](SECURITY.md) para reporte de vulnerabilidades.
