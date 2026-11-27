---
document_id: TL500-M01
project: TrazaLab
version: 0.1.0
status: PROPUESTA_PARA_REVISION
date: 2026-11-24
---

# Mega Documento 01 · Visión de producto, alcance y significado de 500X

Esta edición propone mejoras; no declara implementación, resultados de pruebas ni autorización para modificar repositorios. Las referencias [D1]–[D3] y [S1]–[S8] se resuelven en [FUENTES.md](FUENTES.md). Leer junto al [índice](00-LEEME.md).

## Resumen ejecutivo

Elevar TrazaLab significa convertir la demostración actual en una plataforma de trazabilidad computacional confiable, sin perder el núcleo Rust/Wasm ni la claridad del recorrido de aula. La primera decisión propuesta es consolidar el incremento documentado antes de introducir más algoritmos, soporte multilenguaje, almacenamiento remoto o un workbench distinto. La evidencia disponible no demuestra necesidad de reescritura [D1–D3].

500X designa la ambición de producto. No constituye un multiplicador de velocidad, número de lenguajes o capacidad ya obtenido. Una ampliación será aceptable si conserva exactitud y utilidad bajo presupuestos declarados. No se multiplican entre sí ratios de memoria, velocidad y cobertura para fabricar un resultado global.

## 1. Punto de partida y vacíos

La base documental describe arreglo de enteros, validación, burbuja en Rust/Wasm y navegación de traza. El frontend efectivo está pendiente de cotejo entre antecedentes Leptos/Trunk y una frontera core/adaptador/web. La corrección del control Ordenar y la disponibilidad del servidor son antecedentes distintos, no fallos actuales reproducidos. No se acredita backend, BD o siete runtimes [D1–D3].

El primer entregable de ingeniería no será una pantalla nueva: será una revisión de referencia, sus instrucciones reproducibles y un recorrido capturado. Si difiere del expediente, registrar la diferencia sin reescribir el pasado.

## 2. Usuarios y trabajos completos

| Persona | Trabajo | Resultado útil | Fallo inaceptable |
|---|---|---|---|
| Estudiante | Explicar un intercambio | Nombra índices, valores y causa | Adivina sólo por color |
| Docente | Comparar caso inverso y ordenado | Regresa al mismo estado y lo presenta | Confunde ausencia de swaps con fallo |
| Desarrollador | Comprobar un productor | Compara estados con un oráculo | Renderer corrige una traza inválida |
| Revisor | Autorizar una candidata | Tiene revisión, entorno y evidencias | Se hereda PASS de otra revisión |
| Autor de material | Exportar una explicación | Abre el artefacto sin herramientas de desarrollo | Se filtran rutas privadas o credenciales |

## 3. Niveles de evolución

| Nivel | Capacidad propuesta | Puerta |
|---|---|---|
| L0 · Fidelidad | Recorrido actual correcto y reproducible | G1 del documento 07 |
| L1 · Traza | Contrato, navegación y errores estables | G2 |
| L2 · Capacidad | Menor costo donde una medición lo justifique | G3 |
| L3 · Autoría | Escenas, inspector y exportación | G4 |
| L4 · Escritorio | Instalador Windows si se aprueba ADR | G5 |
| L5 · Interoperabilidad | Segundo productor y formatos versionados | G6 |

Los niveles no equivalen a 25×, 100× o 500×. Esos números sólo tendrán significado por métrica y workload. Una candidata puede mejorar L3 sin ser más rápida que L1.

## 4. Requisitos identificables

- TL500-R01: mantener el motor Rust y aislarlo de la tecnología de UI; verificar dependencias, no sólo carpetas.
- TL500-R02: habilitar ejecución únicamente con motor listo y revisión de entrada validada; editar invalida la autorización anterior.
- TL500-R03: todo estado reconstruido debe coincidir con el estado de referencia para ese cursor.
- TL500-R04: cuotas y finalización completa/parcial/cancelada explícitas; nunca ocultar descarte de eventos.
- TL500-R05: información textual equivalente, teclado, foco y controles coherentes.
- TL500-R06: medir costo por etapa y capacidad útil; no publicar multiplicadores sin baseline.
- TL500-R07: importaciones validadas y sin ejecución implícita.
- TL500-R08: producto de escritorio opcional, instalable y con permisos mínimos si se ratifica.
- TL500-R09: exportación SOP-JS conserva controles, estilo y correspondencia con la traza.
- TL500-R10: segundo productor prueba equivalencia lógica, no identidad accidental de instrucciones.
- TL500-R11: manifiesto de construcción y evidencias por revisión.
- TL500-R12: ampliaciones mediante decisiones reversibles, sin force push ni despliegue implícito.

La matriz CSV del paquete conecta estos requisitos con pruebas y puertas. IDs nuevos: no sustituyen los TL del expediente ni reutilizan gates oficiales de otros proyectos.

## 5. No objetivos del primer corte

No se incluye ejecución arbitraria de Rust/Python/JavaScript aportado por un usuario, marketplace, colaboración en tiempo real, base remota, IA necesaria para operar ni paridad con las ocho estructuras de Visu. Tampoco se instala Code-OSS por el hecho de usar RustRover para desarrollar. IDE de fabricación, anfitrión del producto y motor son decisiones distintas.

## 6. Catálogo de ampliaciones condicionado

Primero: comparación de burbuja y un segundo algoritmo sobre la misma estructura. Después: búsqueda y estructuras con identidad; grafos sólo tras fijar semántica de visita y orden de vecinos. Cada módulo añade fixture, productor, schema si aplica, renderer, explicación textual, límites, pruebas y documentación. No se da por incorporado un algoritmo por añadir un botón.

Para multilenguaje publicar una ficha por modo: IMPORTAR, INSTRUMENTAR o EJECUTAR, con versión, tipos admitidos, restricciones y evidencia. Primera ampliación recomendada: importar una traza equivalente de un segundo productor, sin compilar ni ejecutar código externo.

## 7. Valor y prioridades

P0 es confianza: carga, entrada, ordenamiento y navegación. P1 es capacidad: memoria y respuesta sin alterar semántica. P2 es explicación y distribución. P3 es nuevos dominios. Cada entrega debe resolver un trabajo de usuario completo; no contar paneles vacíos como valor. Métricas didácticas propuestas: acierto al localizar el primer intercambio, tiempo hasta explicar un paso y tasa de recuperación de error. Requieren un protocolo con participantes y consentimiento; no se inferirán del FPS.

## Conclusión y aceptación

Aceptar esta visión significa permitir especificación y pilotos, no autorizar migración. El dueño del producto ratifica alcance y un revisor independiente verifica el recorrido. Hasta entonces L0–L5 son objetivos. El primer éxito es una demostración íntegra sobre una revisión fija, con errores comprensibles y estados fieles.
