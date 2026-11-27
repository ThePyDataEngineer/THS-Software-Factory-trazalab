---
document_id: TL500-M03
project: TrazaLab
version: 0.1.0
status: PROPUESTA_PARA_REVISION
date: 2026-11-24
---

# Mega Documento 03 · Contrato de trazas, identidad y reproducción exacta

Esta edición propone mejoras; no declara implementación, resultados de pruebas ni autorización para modificar repositorios. Las referencias [D1]–[D3] y [S1]–[S8] se resuelven en [FUENTES.md](FUENTES.md). Leer junto al [índice](00-LEEME.md).

## Resumen ejecutivo

La traza es el punto de interoperabilidad de TrazaLab, no una colección de capturas de pantalla. Este documento propone un contrato TLTrace 0.1 de alcance deliberadamente pequeño para arrays i32, comparaciones y swaps. Es candidato: no se afirma que los exports actuales lo acepten [D2–D3]. El anexo JSON es un fixture lógico completo conforme a la propuesta, no salida observada del motor.

## 1. Estado y numeración

El cursor 0 representa initial. Cada evento tiene seq desde 1, contiguo; el cursor k representa el estado después del evento k. Los eventos compare no mutan. Los swap intercambian los elementos en i,j. initial y eventos son inmutables después de aceptar la traza. El cursor y el zoom pertenecen a la sesión, no a la verdad de ejecución.

Para arrays futuros de registros, separar element_id de posición y valor; dos valores iguales no identifican el mismo elemento. Para la propuesta numérica actual no se afirman garantías de identidad de objetos. Si se introducen referencias, el contrato tendrá entidades, IDs y aristas de referencia; no se derivará identidad del índice que cambia al intercambiar.

## 2. Sobre propuesto

| Campo | Semántica |
|---|---|
| schema | TLTrace/0.1; versión mayor desconocida rechazada |
| run_id | Identificador opaco de ejecución, no autorización |
| input_revision | Revisión de entrada que originó la ejecución |
| producer | Nombre, versión e implementación declarada |
| algorithm | Nombre y variante; no basta con 'burbuja' |
| initial | Array i32; copia lógica inicial |
| events | Secuencia contigua de compare y swap |
| terminal | complete, cancelled, resource_limit o failed |
| final | Estado después del último evento aceptado |
| evidence_kind | simulated o recorded; no se infiere de apariencia |

Para complete debe demostrarse que el algoritmo finalizó, no sólo que los valores quedaron ordenados antes de agotar eventos. En cancelación final es el estado del prefijo, no necesariamente una solución. Una traza truncada no se transforma en complete.

## 3. Semántica de eventos

compare lleva i,j y result = -1, 0 o 1 según comparación entre los valores en el estado previo. El resultado se recalcula al validar. swap lleva i,j; el validador comprueba rango y aplica una única permutación. No se impone que todo swap de cualquier algoritmo esté precedido por compare; en burbuja la validación del productor sí puede exigir la secuencia de la variante elegida. Una anotación pedagógica no cambia estado.

No incluir milisegundos como criterio de igualdad lógica. Timestamps, IDs aleatorios y duración se comparan aparte. Si se requiere aleatoriedad, registrar semilla y versión del generador. El orden de vecinos de grafos y de iteración se hará explícito antes de admitirlos. No aceptar 'evento desconocido' y continuar cuando pueda mutar estado.

## 4. Reconstrucción y checkpoints

Con snapshots completos se conserva un oráculo sencillo. Con deltas, restaurar el checkpoint de cursor c≤k y aplicar eventos c+1..k. No sumar ni omitir el evento del checkpoint. Comparar estado reconstruido y referencia en todos los cursores pequeños y muestras reproducibles de trazas grandes. Un checkpoint importado también es dato no confiable: validar procedencia o contrastarlo; el hash de un paquete hostil no demuestra correctitud.

Con n elementos, E eventos, tamaño medio d por delta e intervalo K, una aproximación de payload es n·s + E·d + floor(E/K)·n·s, con s bytes por elemento. No incluye metadatos, asignador, UI, copias ni índices. Un salto rehace como máximo K−1 eventos desde un checkpoint cercano si existen todos los checkpoints previstos. Medir para elegir K; no congelar K=1000 sin datos.

## 5. Validación de entrada

Orden: limitar bytes → parsear de forma acotada → comprobar versión y campos → contar/validar arrays → aplicar eventos con límites → contrastar final/terminal → aceptar de forma atómica. Si falla, no reemplazar la ejecución válida que ya se muestra. El rango i32 es cerrado; índices no negativos y menores que n. seq y cantidades tienen máximo configurado para evitar desbordes o asignaciones enormes. Rechazar NaN/Infinity si el parser los admite; no normalizarlos silenciosamente.

El esquema formal deberá fijar additionalProperties y política de extensiones. Una versión menor sólo añade información que no cambia semántica; tipos de evento mutantes nuevos requieren negociación y soporte. Conservar bytes originales si el usuario requiere diagnóstico, dentro de cuotas y sin ejecutarlos.

## 6. Ejemplo verificable de diseño

Para [4,1,3], compare(0,1)=1; swap(0,1) → [1,4,3]; compare(1,2)=1; swap(1,2) → [1,3,4]; compare(0,1)=-1. Cursor 3 conserva [1,4,3] porque compara pero no muta. La variante es burbuja con pasada decreciente sin salida temprana. El anexo incorpora los cinco eventos y terminal complete. Es ejemplo simulado, no una traza capturada.

La prueba de cancelación en cursor 2 deja [1,4,3] y terminal cancelled. No conserva el final [1,3,4]. La de corrupción modifica compare result o introduce índice 99: debe rechazarse con localizador, sin corregirlo en el renderer.

## 7. Productores y equivalencia

Dos programas no necesitan ejecutar las mismas instrucciones para representar un algoritmo. Para comparar productores distinguir equivalencia de resultado, equivalencia de estados observables en hitos y equivalencia exacta de eventos. El modo elegido se publica en el fixture. Para el primer segundo productor, exigir misma variante y convención de paso; después permitir proyecciones de hitos. No usar igualdad de JSON bruto como sinónimo de determinismo.

## Puerta G2 y contrato de aceptación

Pruebas T03, T04 y T05: todos los cursores del fixture; entrada ordenada; repetidos y negativos; vacío y singleton según contrato ratificado; evento desconocido; cuota; corrupción; cancelación. Definir primero si vacío es válido. Estado actual NOT_RUN en producto. La semántica propuesta no se promoverá hasta comparar el formato real y preparar adaptador o migración reversible.
