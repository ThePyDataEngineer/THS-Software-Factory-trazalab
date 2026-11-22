---
id: ADR-0004
title: "Política de Evidencia Versionable"
status: "ACEPTADO"
date: "2026-11-22"
deciders: ["ThePyDataEngineer"]
---

# ADR-0004: Qué Versionar en evidence/ y Qué No

## Principio

Versionar solo lo necesario para reproducibilidad y auditoría. No versionar ruido ni datos sensibles.

## Incluido (Versionar)

| Directorio | Contenido | Formato |
|:---|:---|:---|
| `evidence/builds/` | Resumen: commit, timestamp, toolchain, hash Wasm, tamaño, duración | `.json` (uno por build) |
| `evidence/test-runs/` | Resumen: crate, target, passed/failed, duración, hash binario test | `.json` |
| `evidence/benchmarks/` | Resumen sanitizado: media, p95, hardware, commit (si aplica) | `.json` |
| `evidence/wasm-artifacts/` | Hash SHA-256 de `.wasm` + metadata (target, profile, toolchain) | `.sha256` + `.json` |
| `fixtures/scenarios/` | VISU-T02, T03, T04 (entradas + trace esperado) | `.json` |
| `fixtures/golden/*.sha256` | Hashes de salidas canónicas | `.sha256` |
| `fixtures/schemas/` | JSON Schemas validación | `.json` |

## Excluido (NO Versionar)

| Contenido | Razón | Alternativa |
|:---|:---|:---|
| Logs crudos cargo/wasm-pack | Ruido, paths absolutos, timestamps variables | Resumen sanitizado en `evidence/builds/` |
| Binarios `.wasm`, `.exe`, `.dll` | Tamaño, reproducibles desde hash + source | Hash en `evidence/wasm-artifacts/` |
| Salidas completas simulación (golden outputs) | Pueden ser MB/GB | Hash + metadata; regenerar en CI |
| Diagnósticos runtime (traces, perf.data) | Específicos de máquina, sensibles | Resumen agregado en benchmarks |
| Secretos, tokens, credenciales | Seguridad | **NUNCA** — `.gitignore` bloquea |
| Configuración local IDE/Hermes | Fábrica, no producto | `.gitignore` excluye `agents/`, `.hermes/` |

## Sanitización

Script `scripts/sanitize-evidence.ps1` (diferido hasta necesidad concreta):
- Paths absolutos → relativos
- Timestamps variables → `REDACTED_TIMESTAMP`
- Secretos conocidos (patrones `ghp_`, `sk-`, `Bearer `, etc.) → `REDACTED_SECRET`
- Validar salida contra schema `evidence/schema.json` (por definir)

## Retención

- `evidence/builds/`, `test-runs/`, `benchmarks/`: 90 días (historial Git).
- `evidence/wasm-artifacts/`: Permanente (trazabilidad releases).
- `fixtures/`: Permanente (contrato de traza).
