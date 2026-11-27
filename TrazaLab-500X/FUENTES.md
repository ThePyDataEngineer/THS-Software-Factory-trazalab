# Fuentes y alcance de la evidencia

## Fuentes documentales de TrazaLab

[D1] 01-TrazaLab-Producto-y-Funcionamiento.md, edición 0.1.0, 24-11-2026. Consolidación candidata, no auditoría del commit vigente.
[D2] 02-TrazaLab-Ecosistema-y-Arquitectura.md, misma edición. Rust/Wasm, divergencia de frontend y límites de soporte.
[D3] 03-TrazaLab-Evaluacion-y-Ruta-de-Diseno.md, misma edición. Corrección acotada, contrato de traza y casos TL no ejecutados.

Los originales no se incluyen ni se modifican. Se consultaron sus contenidos en el hilo. No se leyó el repositorio privado ni se compiló TrazaLab. Sus antecedentes y commits abreviados siguen siendo REFERIDOS, no auditados.

## Fuentes técnicas primarias consultadas el 24-11-2026

[S1] Tauri, Architecture: https://v2.tauri.app/concept/architecture/
[S2] Tauri, Windows Installer: https://v2.tauri.app/distribute/windows-installer/
[S3] Tauri, Capabilities: https://v2.tauri.app/security/capabilities/
[S4] wasm-bindgen, Wasm in Web Worker: https://wasm-bindgen.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
[S5] W3C, WCAG 2.2: https://www.w3.org/TR/WCAG22/
[S6] Microsoft, repositorio Code-OSS: https://github.com/microsoft/vscode
[S7] Microsoft, FAQ de VS Code: https://code.visualstudio.com/docs/supporting/faq
[S8] Eclipse Theia, plataforma y composición: https://theia-ide.org/docs/composing_applications/

S1–S5 se recuperaron/consultaron para esta entrega; S6–S8 proceden de la investigación primaria del mismo hilo. Ninguna fuente sustituye una prueba de compatibilidad en la revisión elegida. S4 conserva una nota histórica de compatibilidad de módulos workers: no se usa para afirmar soporte actual por navegador ni para fijar flags de producción. Las versiones concretas de crates, herramientas y anfitriones deben fijarse en G0/G5. No se trasladan las afirmaciones de marketing sobre tamaños o velocidad como resultados de TrazaLab.

## Método

Cada Mega Documento distingue base documental, propuesta, comprobación necesaria y puerta de aceptación. Las fórmulas y fixtures son diseño de referencia. La existencia de ejemplos JSON válidos no prueba que el motor vigente los acepte. No se emite aprobación de arquitectura, licencia o despliegue. Estados de pruebas de producto: NOT_RUN. Hashes del paquete: integridad, no firma ni prueba de procedencia de terceros.
