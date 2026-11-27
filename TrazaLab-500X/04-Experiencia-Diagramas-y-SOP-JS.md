---
document_id: TL500-M04
project: TrazaLab
version: 0.1.0
status: PROPUESTA_PARA_REVISION
date: 2026-11-24
---

# Mega Documento 04 · Experiencia de usuario, diagramas y presentación SOP-JS

Esta edición propone mejoras; no declara implementación, resultados de pruebas ni autorización para modificar repositorios. Las referencias [D1]–[D3] y [S1]–[S8] se resuelven en [FUENTES.md](FUENTES.md). Leer junto al [índice](00-LEEME.md).

## Resumen ejecutivo

La mejora visual debe hacer legible el estado verdadero, no tapar fallos con animación. Preservar la identidad visual aceptada de SOP-JS en exportaciones y presentaciones: centrado, paleta y controles inferiores, Control Desk y Presenter. Esto no impone copiar cada control de presentación en el editor de arreglos. TrazaLab autoriza y produce; el exportador traduce; SOP-JS reproduce. Son responsabilidades separadas.

## 1. Máquina de interacción

| Evento | Estado visible | Acción permitida |
|---|---|---|
| Abrir | Motor cargando; entrada pendiente | Editar, no ejecutar |
| Init OK | Motor listo | Validar |
| Validación OK | Revisión aceptada | Ordenar si no hay tarea activa |
| Editar | Revisión nueva | Invalidar autorización anterior |
| Ejecutar | run_id e input_revision fijos | Cancelar según modalidad; impedir doble inicio |
| Resultado | Traza aceptada | Navegar dentro de su rango |
| Error | Etapa y recuperación | Conservar entrada; no simular éxito |

Separar estado del motor, validación, tarea y playback. El predicado de ejecución propuesto es motor listo Y revisión validada actual Y ninguna ejecución activa [D3]. Si se conserva una traza previa tras editar, rotularla como anterior y mantener la revisión visible. Una respuesta obsoleta no la reemplaza.

## 2. Cuatro modelos visuales

Semántico: entidades, relaciones y operación. Geométrico: cajas, puertos, rutas, corredores, etiqueta y cámara. Visual: tema, contraste, activo/seleccionado/error. Temporal: cursor, fase y velocidad. Seleccionar un nodo no altera la traza; cambiar velocidad no altera la secuencia de estados. Un renderer alternativo recibe la misma proyección, no vuelve a ordenar.

Para arrays: índice fijo en el modelo de posiciones, valor legible, pareja comparada y operación en texto. Para grafos futuros: IDs estables, selección por entidad y orden de visitas explícito. Mantener retorno al mismo objeto tras recalcular layout cuando sea posible. Si la geometría cambia por contenido, conservar una política de anclaje y anunciar cambio, no seguir automáticamente el token.

## 3. Catálogo inicial de diagramas

| Patrón | Uso | Regla |
|---|---|---|
| Recorrido principal | Entrada → motor → traza → vista | Distinguir datos de control |
| Árbol de estados | Validación y ejecución | Rama de insuficiencia visible |
| Relevo con retorno | Productor y consumidor | Corredor de retorno sin atravesar cajas |
| Timeline | Cursor y eventos | No confundir tiempo visual con orden lógico |
| Antes/después | Swap o mutación | Índices y valores además de color |
| Frontera de confianza | Archivo, UI, Wasm, host | Señalar dónde se valida y qué privilegios existen |

Arrancar con geometría controlada. Si una escena es demasiado densa, agrupar o subdividir antes de reducir tipografía. Probar etiquetas largas, rutas de retorno y foco del inspector, no sólo el ejemplo bonito de tres nodos. SVG es candidato para este corpus; Canvas/WebGL se evalúan sólo con cuellos medidos y alternativa accesible.

## 4. Cámara, selección e inspector

Calcular bounds de nodos, etiquetas, puntas y trazo en un mismo espacio. Mantener margen y viewBox estable durante pasos ordinarios. El panel lateral reduce el área útil; centrar en la región que realmente queda disponible. Reencuadrar sólo por solicitud o cambio topológico significativo. En móvil, apilar inspector sin convertir toda la página en un lienzo horizontal.

Inspector mínimo: ejecución y revisión; cursor actual y total; operación; índices afectados; valores antes y después; motivo del paso; terminal y advertencia de parcialidad. No mostrar un total definitivo mientras el productor siga emitiendo sin longitud conocida. Diferenciar 'sin primer intercambio' de 'no ejecutado'.

## 5. Accesibilidad y comprensión

Objetivo propuesto WCAG 2.2 AA para el flujo completo [S5]. Verificar teclado, foco visible y no oculto, mensajes de estado, nombre/rol/valor, contraste y alternativa textual. Contraste mínimo de texto normal 4,5:1 y texto grande 3:1; gráficos/controles necesarios 3:1 donde aplique. Evaluar estados activos y error, no sólo tokens de tema. No usar color exclusivo. Ofrecer paso manual y movimiento reducido; el usuario puede entender el evento sin esperar una transición.

No anunciar todo el stream en aria-live: saturaría la lectura. Priorizar cambios solicitados, errores y término de tarea. Probar con lector real y recorrido íntegro. El reflow tiene excepciones para diagramas que requieren dos dimensiones, no para esconder todos los controles fuera de pantalla.

## 6. Exportación didáctica y privacidad

El exportador recibe una traza ya aceptada y una selección de hitos; no ejecuta código arbitrario ni lleva permisos de escritorio. Paquete autónomo con assets locales cuando se requiera offline. Conservar datos mínimos, advertir si hay rutas/nombres privados y permitir omitirlos. Validar relación entre cursor exportado y evento fuente. Una selección de hitos se etiqueta como resumen y no como traza completa.

Stage es la proyección publicada; Desk prepara; Presenter controla según permisos. PROPUESTA: Freeze mantiene snapshot; PROJECT publica deliberadamente candidata validada; UNFREEZE no publica por sí solo. Confirmar política contra la plantilla vigente antes de implementarla. Reconectar una ventana solicita snapshot publicado y no envía borrador. Mantener estilo y ubicación de controles; no rediseñar la plantilla para resolver un problema del motor.

## Puerta G4

Recorrer caso [4,1,3] y caso ya ordenado por teclado, vista estrecha, zoom y movimiento reducido. Comparar estado e inspector en avance/retroceso/salto. Exportar dos hitos y comprobar su correspondencia. Si se integra Presenter, probar tres ventanas y ausencia de publicación involuntaria. Los casos están NOT_RUN y no certifican conformidad accesible.
