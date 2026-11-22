---
id: REPRO-TEST
title: "Guía Reproducción — Test"
version: "0.1.0"
status: "VIGENTE"
---

# Reproducción — Test

## Test Núcleo (visu-core)

```powershell
cd I:\THS Software Factory - Visu
cargo test -p visu-core --all-targets --locked
# Esperado: validation_tests, sorting_tests, trace_tests → PASS
```

## Test Wasm (visu-wasm) — Diferido a WEB-01

```powershell
# wasm-pack test --headless --firefox --chrome
# Requiere navegadores en CI; no en CI básica v0.1
```

## Validación Fixtures

```powershell
cd I:\THS Software Factory - Visu
# Validar que fixtures cumplen schema (script diferido)
```

## Criterios VISU-T01 a T04 (Ejecución Manual en Edge)

| Test | Acción | Éxito |
|:---|:---|:---|
| T01 | Cargar `http://127.0.0.1:8080` → consola sin error "Failed to load module" | Módulo Wasm carga |
| T02 | Input `[5,2,9,1,7,3]` → ejecutar → trace navega pasos → resultado `[1,2,3,5,7,9]` | Ordenamiento + trace |
| T03 | Navegar: Final → Primer intercambio → Final → mismo estado lógico | Navegación sin reinicio |
| T04 | Input `[3,1,"x",2]` → error `non_integer` → corregir a `[3,1,3,2]` → `[1,2,3,3]` | Validación + recuperación |

> **Nota**: Abrir página ≠ módulo cargado. Cargar módulo ≠ ordenar. Ordenar ≠ navegar. Navegar ≠ recuperar.
