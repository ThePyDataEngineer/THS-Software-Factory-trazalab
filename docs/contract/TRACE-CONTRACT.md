---
id: TRACE-CONTRACT-VISU
title: "Contrato de Traza — Visu v0.1"
version: "0.1.0"
status: "VIGENTE"
date: "2026-11-22"
---

# Contrato de Traza — Visu v0.1

## 1. Versionado del Contrato

Formato: `MAJOR.MINOR.PATCH` — Cambio MAJOR = breaking en serialización o API Wasm.

## 2. Entrada (Configuración de Escenario)

```json
{
  "scenario_id": "VISU-T02",
  "input": [5, 2, 9, 1, 7, 3],
  "algorithm": "bubble_sort",
  "variant": "standard"
}
```

| Campo | Tipo | Requerido | Descripción |
|:---|:---|:---:|:---|
| `scenario_id` | string | ✅ | Identificador: `VISU-T02` \| `VISU-T03` \| `VISU-T04` |
| `input` | array\<integer> | ✅ | Arreglo de enteros (1–100 elementos) |
| `algorithm` | string | ✅ | `"bubble_sort"` (v0.1) |
| `variant` | string | ✅ | `"standard"` — burbuja clásico con bandera swapped |

## 3. Salida (Trace Completo)

```json
{
  "scenario_id": "VISU-T02",
  "input": [5, 2, 9, 1, 7, 3],
  "output": [1, 2, 3, 5, 7, 9],
  "trace": {
    "steps": [
      { "step": 0, "array": [5,2,9,1,7,3], "swap": null },
      { "step": 1, "array": [2,5,9,1,7,3], "swap": { "index": 0, "a": 5, "b": 2 } },
      ...
      { "step": 10, "array": [1,2,3,5,7,9], "swap": null }
    ],
    "comparisons": 15,
    "swaps": 9
  }
}
```

### 3.1 Esquema de Paso (Step)

| Campo | Tipo | Descripción |
|:---|:---|:---|
| `step` | integer ≥ 0 | Número de paso (0 = estado inicial) |
| `array` | array\<integer> | Estado completo del arreglo tras el paso |
| `swap` | object \| null | Si hubo intercambio: `{ "index": i, "a": val_i, "b": val_i+1 }` |

### 3.2 Navegación (VISU-T03)

El contrato **requiere** que el núcleo permita consultar cualquier `step` sin reiniciar la simulación. La UI navega entre checkpoints:
- `final` → step N (estado ordenado)
- `first_swap` → step 1 (primer intercambio)
- `final_return` → step N (mismo estado lógico)

## 4. Validación y Errores (VISU-T04)

### 4.1 Entrada Inválida

```json
{ "input": [3, 1, "x", 2] }
```

### 4.2 Respuesta de Error

```json
{
  "error": "validation_failed",
  "code": "non_integer",
  "index": 2,
  "received": "x",
  "message": "Elemento en índice 2 no es entero: 'x'"
}
```

### 4.3 Recuperación

- UI preserva texto original en editor
- No muestra traza anterior como resultado actual
- Usuario corrige → re-ejecuta validación + ordenamiento
- Resultado: trace VISU-T04 estándar con `[1,2,3,3]`

## 5. Invariantes

1. **Determinismo**: Mismo `input` + misma `variant` → mismo `trace` bit-a-bit.
2. **Monotonía de pasos**: `step` crece 0, 1, 2... N.
3. **Conservación de elementos**: `output` es permutación de `input` (multiconjunto idéntico).
4. **Orden final**: `output[i] ≤ output[i+1]` para todo `i`.
5. **Serialización canónica**: JSON sin espacios, claves ordenadas, UTF-8 → bytes para hash golden.

## 6. Golden Outputs (Pendientes de Definición Exacta de Bytes)

| Fixture | Hash SHA-256 (Placeholder) | Registrado En |
|:---|:---|:---|
| VISU-T02 | `PENDIENTE_HASH` | `fixtures/golden/VISU-T02.output.sha256` |
| VISU-T03 | `PENDIENTE_HASH` | `fixtures/golden/VISU-T03.output.sha256` |
| VISU-T04 | `PENDIENTE_HASH` | `fixtures/golden/VISU-T04.output.sha256` |

> **No se declaran hashes** hasta fijar: codificación, normalización JSON, campos meta, orden de claves.
