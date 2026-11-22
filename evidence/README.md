---
id: EVIDENCE-POLICY
title: "Política de Evidencia Versionable — Visu"
version: "0.1.0"
status: "VIGENTE"
date: "2026-11-22"
---

# Política de Evidencia — Visu

Ver [docs/adr/0004-evidence-policy.md](../adr/0004-evidence-policy.md) para detalle completo.

## Resumen

| Directorio | Versionar | No Versionar |
|:---|:---|:---|
| `evidence/builds/` | Resúmenes sanitizados (commit, hash Wasm, tamaño, duración) | Logs crudos |
| `evidence/test-runs/` | Resúmenes (crate, target, passed/failed, duración) | Outputs completos |
| `evidence/benchmarks/` | Resúmenes sanitizados (media, p95, hardware, commit) | Datos crudos, perf.data |
| `evidence/wasm-artifacts/` | Hashes SHA-256 + metadata | Binarios `.wasm` |
| `fixtures/scenarios/` | VISU-T02, T03, T04 (entradas + trace esperado) | — |
| `fixtures/golden/*.sha256` | Hashes salidas canónicas | Salidas completas |
| `fixtures/schemas/` | JSON Schemas | — |

## Sanitización

Script `scripts/sanitize-evidence.ps1` (diferido) aplica:
- Paths absolutos → relativos
- Timestamps → `REDACTED_TIMESTAMP`
- Secretos (patrones conocidos) → `REDACTED_SECRET`

## Regla de Oro

**Antes de compartir o publicar: revisar contenido.** Un script no sustituye la revisión humana.
