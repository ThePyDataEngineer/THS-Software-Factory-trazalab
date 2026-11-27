---
document_id: TL500-M05
project: TrazaLab
version: 0.1.0
status: PROPUESTA_PARA_REVISION
date: 2026-11-24
---

# Mega Documento 05 · Escala, rendimiento y demostración cuantitativa de 500X

Esta edición propone mejoras; no declara implementación, resultados de pruebas ni autorización para modificar repositorios. Las referencias [D1]–[D3] y [S1]–[S8] se resuelven en [FUENTES.md](FUENTES.md). Leer junto al [índice](00-LEEME.md).

## Resumen ejecutivo

No existe benchmark actual que permita prometer 500×. La estrategia es localizar costos, definir capacidad útil y optimizar el componente dominante conservando exactitud. Rust/Wasm no elimina la complejidad del algoritmo, del volumen de eventos ni de la representación [D3]. El protocolo siguiente es propuesto; todos sus ensayos en TrazaLab están NOT_RUN.

## 1. Descomposición del costo

Medir por separado carga de recursos, inicialización Wasm, parsing/validación, algoritmo, emisión de eventos, serialización, transferencia, reconstrucción, layout y primer render. Medir también entrada→feedback y clic→estado visible. No atribuir todo al motor ni mezclar arranque frío con reproducción caliente. Para memoria registrar qué métrica ofrece cada entorno y sus limitaciones: heap del consumidor, memoria lineal Wasm y proceso no son intercambiables.

| Síntoma medido | Intervención candidata | Verificación obligatoria |
|---|---|---|
| Duplicación de arrays por paso | Deltas/checkpoints | Igualdad de todos los cursores pequeños |
| UI bloqueada por cálculo | Fragmentación o worker | Cancelación, latencia y costo de copia |
| Exceso de cruce de frontera | Lotes y buffers | Sin pérdida ni reordenamiento |
| Layout domina navegación | Reutilizar geometría | Misma semántica y cámara estable |
| DOM grande | Ventana visible/agregación | Consulta de valores fuera de vista |
| Log desborda memoria | Cuotas y almacenamiento por segmentos | Parcialidad explícita y recuperación |

No aplicar todas las optimizaciones juntas: impediría atribuir mejora y localizar regresión. Mantener un oráculo simple, aun si sólo sirve para trazas pequeñas.

## 2. Complejidad y límites antes de ejecutar

Una variante de burbuja con pasada decreciente y sin salida temprana realiza n(n−1)/2 comparaciones. Invertida, realiza ese mismo número de swaps. Si se registran ambos por separado, E=n(n−1). Con n=400 son 79.800 comparaciones y hasta 159.600 eventos compare/swap. Esto es una derivación del modelo, no un conteo observado del producto. Un control que admite n grande sin presupuesto de E puede bloquearse aun con un renderer eficiente.

Definir cuotas de bytes de entrada, elementos, eventos, bytes retenidos, tareas activas y trabajo por lote. Proponer advertencia antes del inicio si la cota conocida supera el presupuesto. Si se ofrece 'sólo resultado', es otra modalidad sin detalle completo, no una optimización transparente de la traza. No muestrear eventos mutantes silenciosamente; si se exporta resumen, conservar semántica y rotular hitos omitidos.

## 3. Corpus escalonado

| Familia | Entradas | Qué prueba |
|---|---|---|
| Correctitud | Vacío si se admite, singleton, repetidos, negativos, ordenado, inverso | Orden y multiplicidad; errores de frontera |
| Escala inicial | n=10, 40, 100, 400; aumentar sólo con presupuesto | Crecimiento y costo por evento |
| Navegación | Cerca, inicio, centro, final; saltos con semilla fija | Checkpoints y latencia |
| Robustez | Evento corrupto, archivo truncado, cuota agotada | Rechazo y conservación de estado |
| UI | Vista estrecha, zoom, entrada larga y cambio durante tarea | Usabilidad bajo carga |

No imponer esos tamaños a la versión real antes de G0. Si un límite vigente es menor, ensayar hasta él y registrar bloqueo. Para grafos futuros fijar V,E, densidad, etiquetas y operaciones visibles; 10.000 nodos sin más no es un workload.

## 4. Procedimiento de comparación

1. Fijar baseline B y candidata C con commit completo y configuración release.
2. Fijar datos, semilla, variante, modalidad de traza y unidad de paso.
3. Ejecutar una fase pequeña de verificación de resultados antes de medir.
4. Separar arranque frío/caliente y repetir series; registrar tamaño de muestra y orden de ejecución. Evitar carga externa conocida y documentar las excepciones.
5. Publicar muestras brutas, mediana, dispersión y percentiles sólo si la muestra permite interpretarlos. Un p95 sobre muy pocas repeticiones no demuestra estabilidad.
6. Repetir con límites de memoria y UI idénticos. Incluir cancelaciones, OOM y timeouts como fallos/censuras; no eliminarlos para mejorar el promedio.
7. Reproducir con un revisor diferente antes de publicar una afirmación de mejora.

El mismo motor nativo y Wasm puede compararse por núcleo; la experiencia Visu GTK frente a TrazaLab web requiere perfiles completos separados. No atribuir al lenguaje diferencias de navegador o renderer. Sin caso comparable, publicar capacidad absoluta.

## 5. Ratios y alcance de la afirmación

Para un mismo trabajo, speedup = T_B/T_C; ahorro de memoria se puede expresar M_B/M_C además de bytes y porcentaje. Capacidad útil = mayor carga que pasa correctitud, memoria, latencia y disponibilidad de detalle. Su ratio N_C/N_B se publica sólo con la misma familia y criterios. Si cambia la complejidad o modalidad, declarar cambio y comparar por separado.

Ejemplo hipotético, NO resultado: si un salto baja de 500 ms a 1 ms, ese salto mejora 500×; no significa que toda la aplicación lo haga. Nunca multiplicar velocidad por cobertura por número de lenguajes. Una mejora de 2× en memoria puede aportar más utilidad que 500× en una operación irrelevante.

## 6. Presupuestos candidatos y optimización

Como metas iniciales a ratificar tras baseline: respuesta de controles con p95≤100 ms en corpus acordado y capacidad de cancelar sin congelar UI. No se garantiza ese valor para toda carga o dispositivo. Memoria, inicio y salto largo se fijan después de medir la estación de referencia. Si no pasan, registrar antes de mover el umbral; cualquier cambio de presupuesto debe tener motivo y aprobación.

Evaluar K de checkpoints y tamaño de lote con una cuadrícula pequeña. Medir simultáneamente memoria y salto; escoger punto de compromiso, no sólo mínimo de una variable. Una mejora que borra la posibilidad de inspeccionar pasos no conserva el requisito.

## Puerta G3

Exigir datos brutos, entorno, revisión, corpus, modalidad, resultados correctos, fallos, comparación por etapa y recomendación. Si no hay mejora o hay regresión, no promover por entusiasmo. 'Objetivo 500X' queda separado del tablero de resultados alcanzados.
