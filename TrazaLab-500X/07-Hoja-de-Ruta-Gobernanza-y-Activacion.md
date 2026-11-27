---
document_id: TL500-M07
project: TrazaLab
version: 0.1.0
status: PROPUESTA_PARA_REVISION
date: 2026-11-24
---

# Mega Documento 07 · Hoja de ruta, responsabilidades y activación controlada

Esta edición propone mejoras; no declara implementación, resultados de pruebas ni autorización para modificar repositorios. Las referencias [D1]–[D3] y [S1]–[S8] se resuelven en [FUENTES.md](FUENTES.md). Leer junto al [índice](00-LEEME.md).

## Resumen ejecutivo

Este documento convierte la visión en lotes pequeños con responsables, evidencias y puertas. Todos los gates TL500-G0..G6 son nuevos y candidatos. No son G02 histórico, P00 ni gates de otros productos [D2–D3]. Generar este paquete no los cierra ni activa a Hermes.

## 1. Ruta por incrementos

| Gate | Entrega | Criterio de salida | Dependencia |
|---|---|---|---|
| G0 | Línea base | Commit completo, estado local, inventario, build y divergencias identificados | Acceso de lectura a revisión efectiva |
| G1 | Recorrido actual | T01/T02/T06 pasan; error reproducido o ausencia documentada | G0 |
| G2 | Contrato y replay | T03/T04/T05/T07 pasan; semántica y compatibilidad revisadas | G1 |
| G3 | Escala | T08 con mediciones y exactitud; umbrales ratificados | G2 |
| G4 | Visual/exportación | T09/T10 pasan y fidelidad SOP-JS revisada | G2; presupuesto G3 |
| G5 | Windows | T11/T12 pasan; anfitrión y permisos aprobados | G2 y experimento de anfitrión |
| G6 | Segundo productor | T13 y equivalencia declarada; sin ejecutar código no confiable | G2 |

G4/G5/G6 pueden desarrollarse como experimentos separados cuando sus precondiciones existen; no convertir esta tabla en waterfall que retrase seguridad. El primer lote de implementación autorizado debería ser G0–G1. No se estiman fechas o presupuesto sin equipo, revisión y alcance ratificados.

## 2. Roles mínimos y límites

| Función | Puede hacer | No puede certificar sola |
|---|---|---|
| Dueño de producto | Priorizar y aprobar alcance | Correctitud técnica sin evidencia |
| Responsable Rust | Implementar núcleo y pruebas de autor | Revisión independiente de su cambio |
| UI/Wasm | Adaptación, estado, accesibilidad y lifecycle | Que build implica usabilidad |
| QA/revisor | Reproducir pruebas sobre revisión fija | Cambiar esperado para ocultar fallo |
| Release/seguridad | Empaquetado, secretos y procedencia | Publicar sin aprobación |

Una persona puede desempeñar varias funciones, pero debe declarar esa circunstancia; un segundo nombre de agente no demuestra independencia. Si se usa Hermes, distinguir coordinación, construcción y verificación. Ningún rol necesita acceso total por defecto. No heredar inventarios 24+1 sin localizar el contrato que realmente los define.

## 3. Procedimiento G0

Solicitar sólo las fuentes necesarias o consultar el repositorio autorizado en modo lectura. Registrar commit completo, ramas y cambios locales sin descartar nada. Inventariar Cargo.toml, lockfile, toolchain, fuentes UI, configuración de build, scripts, workflows, licencia y output. Contrastar Leptos/Trunk con lo observado. No asumir rutas históricas. Determinar qué comando sirve qué carpeta y qué artefacto carga el navegador.

Una copia local con cambios no confirmados no equivale al commit remoto. Registrar hashes de lo servido. Si falta material, gate BLOCKED con campo y efecto exactos; no 'PASS parcial'. Mantener divergencias como backlog y no corregirlas de forma silenciosa durante inventario.

## 4. Diagnóstico G1

Reproducir carga → validación → ejecución → navegación. Separar conexión rechazada, fetch Wasm fallido, validación y estado del botón. Si Ordenar no se habilita, capturar revisión de entrada, init, validación y evento UI; no rediseñar renderer antes de localizar la transición. Proponer diff mínimo con regresión. Si no se reproduce, documentar entorno y no inventar causa histórica. Seguir con riesgo abierto si corresponde.

## 5. ADR candidatos

- ADR-TL500-01: preservar Rust/Wasm, comparando dependencias antes de extraer módulos.
- ADR-TL500-02: especificar TLTrace y migrar sólo con compatibilidad explícita.
- ADR-TL500-03: medir antes de añadir deltas, checkpoints o worker.
- ADR-TL500-04: mantener perfil web; evaluar Tauri, Code-OSS/Theia sólo por necesidad demostrada.
- ADR-TL500-05: exportar a SOP-JS sin modificar el canon ni mezclar ejecución privilegiada.
- ADR-TL500-06: primero importar trazas; instrumentación y ejecución externa son proyectos separados.

Cada ADR registra problema, opciones, evidencias, decisión humana, costos, prueba y condición de reapertura. Todos están PROPOSED. No se entiende 'conservar' como aprobación de una implementación no leída.

## 6. Riesgos y respuesta

| Riesgo | Señal | Respuesta | Responsable funcional |
|---|---|---|---|
| Reescritura prematura | Nuevo stack antes de reproducir | Detener migración; volver a G0/G1 | Arquitectura/producto |
| Explosión de eventos | Cuota cerca o memoria crece sin límite | Estimar, limitar y separar modalidad | Motor |
| Estado obsoleto | Respuesta A sustituye entrada B | run_id y revisiones | UI/Wasm |
| Replay falso | Mismo cursor cambia | Bloquear promoción G2 | QA/motor |
| Privilegios amplios | Host permite comandos genéricos | Reducir API y ensayar denegaciones | Seguridad |
| Deriva visual | Controles SOP-JS movidos o reemplazados | Rechazar exportación y comparar baseline | UI/producto |
| Multiplicador sin base | Se anuncia 500× por conteo de features | Retirar afirmación; publicar datos por métrica | Producto/QA |
| Pérdida de datos | Migración sobrescribe original | Copia, formato versionado y rollback | Persistencia/release |

## 7. Orden candidata para un ejecutor, NO ACTIVADA

> Trabaja sólo tras mandato explícito y acceso autorizado. Primera fase: lectura y diagnóstico G0/G1. No cambies nombre, carpetas, historial ni dependencias. No realices force push, despliegue, instalación o publicación. Identifica revisión y cambios locales; distingue observación de hipótesis. Presenta inventario, instrucciones reproducibles, resultado de cada caso con evidencia y propuesta de diff mínimo. Si falta precondición, detente con BLOCKED. Cualquier implementación posterior requiere alcance aprobado y conservar originales.

Esta orden no confiere permisos ni solicita efectuar acciones externas en este turno. Es una plantilla de trabajo para el próximo mandato. El solicitante debe revisar el destino y contenido antes de cualquier operación de escritura en servicios externos.

## 8. Registro de evidencia

Cada resultado registra test_id, requirement_id, commit completo, artifact_hash, entorno, input_fixture, modo, esperado, observado, localizador, ejecutor, revisor y estado. Separar PASS, FAIL, ERROR, NOT_RUN, BLOCKED y N/A con motivo. El gate sólo pasa cuando sus pruebas obligatorias están PASS y las excepciones aceptadas formalmente. Una captura aislada no prueba toda la secuencia.

El paquete incluye matriz de requisitos y plan de pruebas para comenzar. No contiene resultados inventados ni rutas al disco privado. Nueva edición si cambia el contrato, la plataforma o el alcance. Referenciar por versión/hash; no reemplazar evidencia antigua.

## Conclusión

La propuesta eleva TrazaLab por confianza, capacidad, explicación, distribución e interoperabilidad; no por una reescritura ni por multiplicar promesas. La secuencia recomendada es fijar y demostrar la versión actual, formalizar traza, medir, ampliar y empaquetar de manera reversible. La primera autorización a solicitar es G0–G1; las demás dependen de evidencia, no del nombre 500X.
