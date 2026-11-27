# Arquitectura Rust-first y contratos entre capas

```text
UI → sesión (revisión y run_id) → adaptador Wasm → núcleo Rust
 ↑                   ↓                                  ↓
 vista ← proyección de replay ← almacén validado ← eventos
                                      ↓
                               exportador SOP-JS
```

No reorganizar Cargo antes de leerlo. Definir fronteras por dependencias comprobables: dominio Rust no importa DOM, host, red ni almacenamiento; UI no interpreta eventos desconocidos ni repite el algoritmo; adaptador conserva exactitud, errores y tipos; coordinador invalida respuestas obsoletas. No duplicar un motor Rust en JS. Tratar JS de enlace como adaptador, no como migración.

Fase 1 mantiene serialización vigente mientras un adaptador de pruebas compara su semántica con TLTrace. Fase 2 permite contrato versionado sólo con compatibilidad o migración explícita. Fase 3 evalúa ejecución fragmentada o Web Worker únicamente si el hilo principal domina. Un `postMessage` no interrumpe cálculo síncrono en el mismo worker: usar tramos que cedan o terminación controlada. Clasificar resultado descartado frente a cancelación efectiva. Lotes numerados, límite en vuelo, Ack tras commit y terminal única. Al cancelar, descartar mensajes tardíos y etiquetar prefijo parcial. Un worker no es frontera para ejecutar código arbitrario ni reduce por sí solo eventos.

Medir copia antes de introducir buffers transferibles. Al transferir un ArrayBuffer deja de estar disponible para el remitente; no transferir memoria viva necesaria para el núcleo o replay. Referencia: https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Transferable_objects. Sin SharedArrayBuffer, hilos Wasm o nuevas dependencias obligatorias para el primer corte.
