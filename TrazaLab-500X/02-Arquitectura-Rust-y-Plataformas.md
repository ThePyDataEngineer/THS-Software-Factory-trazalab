---
document_id: TL500-M02
project: TrazaLab
version: 0.1.0
status: PROPUESTA_PARA_REVISION
date: 2026-11-24
---

# Mega Documento 02 · Arquitectura Rust, Wasm y alternativas de plataforma

Esta edición propone mejoras; no declara implementación, resultados de pruebas ni autorización para modificar repositorios. Las referencias [D1]–[D3] y [S1]–[S8] se resuelven en [FUENTES.md](FUENTES.md). Leer junto al [índice](00-LEEME.md).

## Resumen ejecutivo

Preservar el motor Rust y el perfil web local. El paquete propone una arquitectura de puertos y adaptadores, no una reorganización inmediata del repositorio. La implementación efectiva de Leptos/Trunk debe identificarse antes de introducir TypeScript, otro frontend o empaquetado desktop [D2].

## 1. Dependencias y responsabilidades

```text
Entrada y comandos → coordinador de sesión → puerto de productor
                          ↓                        ↓
                    estado de UI           motor Rust / Wasm
                          ↑                        ↓
Inspector ← proyección de estado ← validador / almacén de traza
                          ↓
                 escena visual → renderer
                          ↓
                exportación SOP-JS (opcional)
```

El núcleo contiene tipos, algoritmo y emisión de eventos. No importa DOM, APIs del workbench, archivos de usuario ni red. El adaptador traduce tipos y errores. El coordinador gobierna revisiones, tareas y cancelación. La UI no ordena por su cuenta para suplir un error del motor. Los nombres de módulos son conceptuales hasta G0; no se crean crates vacíos por anticipación.

## 2. Tres perfiles sin promesas cruzadas

| Perfil | Motor y ejecución | Distribución | Condición |
|---|---|---|---|
| Web vigente | Rust a Wasm en cliente | Archivos servidos estáticamente | Referencia que debe verificarse |
| Desktop candidato | La misma lógica Rust por Wasm o adaptador nativo | Tauri como experimento de empaquetado | ADR y prueba Windows |
| Workbench candidato | Rust detrás de un protocolo restringido | Code-OSS/Theia | Requisito de edición/depuración no cubierto con menos costo |

El navegador no requiere un backend para ordenar localmente [D2]. Tauri es un toolkit Rust con interfaz web [S1], no un motor de algoritmos. Code-OSS ofrece código MIT, pero implica distribución derivada; el Marketplace no se hereda [S6–S7]. Theia ofrece composición de herramientas [S8]. Ninguno debe reemplazar el núcleo.

## 3. Matriz de decisión de anfitrión

Evaluar con el mismo recorrido: abrir, validar, ordenar, pausar navegación, saltar, guardar y reabrir. Registrar costo de integración, parches al anfitrión, tamaño completo de distribución, memoria por proceso y recuperación. No fijar un ganador por familiaridad. Recomendación provisional actualizada: web primero; Tauri merece el primer experimento para Windows por continuidad con Rust; Code-OSS y Theia siguen como alternativas si aparecen requisitos claros de workbench. Esto no ratifica ni revoca ADRs privados no leídos.

Causas de descarte: rompe el perfil web, obliga a duplicar semántica, no conserva controles aceptados, exige permisos excesivos o su costo de mantenimiento no se justifica. Una prueba de build exitosa no prueba experiencia, seguridad ni offline.

## 4. Interoperabilidad y errores

Definir DTO separados de las estructuras internas. Usar enteros de rango explícito; para el primer contrato de intercambio se propone i32. Esto no declara que el motor vigente use i32. Si se amplía a i64, decidir codificación para preservar exactitud en consumidores JavaScript. Índices y tamaños se validan antes de acceso o asignación. No exponer punteros ni handles de memoria Wasm como IDs persistentes.

Errores propuestos: INIT_FAILED, INVALID_INPUT, UNSUPPORTED_VERSION, RESOURCE_LIMIT, CANCELLED, TRACE_INVALID e INTERNAL. El adaptador añade etapa, run_id y detalle seguro. Un panic no se convierte en resultado vacío. No mostrar stack interno con rutas privadas en exportaciones.

## 5. Concurrencia y ciclo de vida

Separar input_revision y run_id. Una respuesta tardía de la ejecución A no actualiza la vista si el usuario ya inició B. Si el cálculo es síncrono largo, un mensaje de cancelación al mismo hilo no lo interrumpe: se necesita trabajo fragmentado con cesión o un worker terminable. Worker es candidato sólo tras medir bloqueo; puede duplicar memoria o copiar datos. La guía wasm-bindgen muestra la posibilidad de cargar Wasm en un worker, no la compatibilidad de nuestra aplicación [S4].

El protocolo de lote propone Start, Batch, Ack, Cancel y Terminal. Un límite de mensajes en vuelo aplica contrapresión; no producir indefinidamente mientras el usuario está pausado. Si termina el worker, invalidar sus callbacks y presentar la traza prefijo como parcial. No iniciar dos motores silenciosamente por doble clic.

## 6. Construcción y disposición del código

El inventario G0 recoge workspace Cargo, features, toolchain, lockfile, config Trunk si existe, fuentes UI, glue generado, workflow y artefacto servido. Comparar rutas de compilación y servidor: servir output no equivale a servir raíz Git. Conservar artefacto anterior con hash para rollback. Mantener source maps y logs sensibles fuera de la publicación pública.

Posibles módulos: domain, algorithms, trace, replay, wasm_adapter y UI. Sólo extraer cuando exista una frontera probada o una necesidad de pruebas. Para desktop añadir un adaptador aislado. No importar servicios de archivos en el crate compartido sólo para facilitar el instalador.

## 7. Persistencia y red

Comenzar con sesión en memoria. Exportar un archivo versionado es preferible a introducir BD sin consultas definidas. Si se ratifica almacenamiento del navegador, manejar cuota y fallo; no prometer durabilidad por haber guardado en memoria. SQLite local sólo con entidad y migración justificadas. Red remota, autenticación y colaboración requieren ADR específico, clasificación de datos y operación.

## Puerta G2 y reapertura

Demostrar que la misma semántica se prueba sin UI; que el adaptador preserva errores; que una respuesta obsoleta se descarta; y que cargar/servir/ejecutar son estados diferenciados. Si la implementación real contradice el diseño, registrar discrepancia y proponer cambio acotado. No renombrar repositorios ni publicar builds en esta fase documental.
