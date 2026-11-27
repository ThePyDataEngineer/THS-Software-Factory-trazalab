---
document_id: TL500-M06
project: TrazaLab
version: 0.1.0
status: PROPUESTA_PARA_REVISION
date: 2026-11-24
---

# Mega Documento 06 · Calidad, seguridad y distribución instalable en Windows

Esta edición propone mejoras; no declara implementación, resultados de pruebas ni autorización para modificar repositorios. Las referencias [D1]–[D3] y [S1]–[S8] se resuelven en [FUENTES.md](FUENTES.md). Leer junto al [índice](00-LEEME.md).

## Resumen ejecutivo

La calidad se incorpora desde el primer corte. No esperar a una fase final para probar importación, errores, cancelación o lectura por teclado. El producto web es la referencia documentada; el instalador es un perfil candidato separado [D1–D3]. Tauri permite NSIS/EXE y MSI para Windows [S2], pero ese soporte no prueba que TrazaLab esté empaquetado ni que su frontend vigente sea compatible sin trabajo.

## 1. Fronteras de confianza

| Entrada/frontera | Amenaza | Control propuesto |
|---|---|---|
| Texto del editor | Tamaño, enteros fuera de rango, separadores ambiguos | Límites y diagnóstico sin coerción silenciosa |
| Traza importada | Índices maliciosos, versión desconocida, volumen | Parser acotado y validación semántica |
| Explicaciones/etiquetas | HTML o script embebido | Texto escapado, sin eval ni handlers |
| Worker | Mensaje tardío, secuencia falsa, cola sin límite | Esquema, identidad, cuotas, Ack y cancelación |
| Escritorio | Acceso a archivos o shell por la webview | Comandos estrechos y autorización por capacidad |
| Actualización | Sustitución de paquete o corrupción | Verificación criptográfica del canal/paquete y recuperación |
| Exportación | Filtración de rutas, notas o credenciales | Selección de datos y revisión antes de publicar |

No confiar en un archivo porque su extensión sea JSON o su hash coincida con un manifiesto aportado por el mismo remitente. No exponer comandos genéricos 'ejecutar shell', 'leer cualquier ruta' o 'descargar y ejecutar'. Rust no garantiza que la política de permisos de la aplicación sea correcta.

## 2. Pruebas y oráculos

- Unitarias: parsing, cotas, compare/swap, multiplicidad, casos borde, validación de trace y terminal.
- Propiedades: salida ordenada y permutación de entrada; replay repetible; round-trip de serialización sin pérdida semántica.
- Diferenciales: comparar el motor con un oráculo independiente y proyecciones pequeñas; no usar el mismo helper defectuoso como autor y juez.
- Integración: revisión de entrada, inicialización, tarea, descarte de respuesta vieja y error de motor.
- UI: foco, controles y estado visible; no sólo captura bonita.
- Seguridad: entradas sobredimensionadas, secuencias corruptas y comandos no autorizados.
- Distribución: instalar, abrir, actualizar, recuperar y desinstalar en máquina limpia.

Una prueba generativa o fuzzing es candidata, no garantía. Fijar semillas o preservar casos mínimos que fallen. Nunca borrar un fallo sólo porque el siguiente intento pasó. PASS de la revisión A no se hereda a B.

## 3. Accesibilidad de proceso completo

Usar WCAG 2.2 como objetivo de evaluación [S5]. Automatización más prueba manual con teclado y lector. Cubrir controles deshabilitados, errores, traza parcial y estado ya ordenado. No bloquear lectura con anuncios continuos. Incluir foco no oculto por la barra inferior y alternativa a arrastrar el cursor. Guardar observaciones por navegador y zoom; no declarar AA por un único escaneo.

## 4. Opciones de empaquetado Windows

Tauri usa una webview y Rust [S1]. Para el primer experimento, conservar motor Wasm y assets empaquetados evita cambiar al mismo tiempo host y semántica. Evaluar después adaptador nativo si una necesidad medible lo justifica, con pruebas diferenciales. No ejecutar simultáneamente dos motores que compitan por la misma sesión.

Opciones de WebView2 [S2]: bootstrapper descargado requiere red; incluir sólo bootstrapper sigue requiriendo red; instalador offline incluye recursos necesarios; runtime fijo permite control pero transfiere responsabilidad de actualización y aumenta distribución. Por tanto 'funciona offline' no es 'se instala offline'. No asumir presencia universal en la máquina final. Seleccionar y probar una política real.

Windows 11 x64 es el primer perfil candidato por continuidad del expediente. Windows 10, ARM64, Ubuntu y Android requieren decisiones/versiones y resultados propios. No deducir compatibilidad de un ejemplo de documentación antigua. El usuario final no debe instalar Rust ni herramientas de compilación; verificar dependencias runtime que sí requiera el artefacto.

## 5. Capacidades y límites de Tauri

La documentación permite restringir acceso por ventana/webview [S3]. Las capacidades de una ventana se combinan, por lo que dos archivos permisivos pueden ampliar privilegios. Además los comandos propios registrados necesitan configuración explícita si se pretende restringirlos: no basta con reducir permisos de plugins. Auditar permisos efectivos y verificar una llamada denegada desde una superficie sin autorización.

Nada de esto protege de código Rust inseguro por diseño, scopes laxos o una implementación que eluda controles [S3]. Limitar creación de ventanas privilegiadas; no conceder APIs a orígenes remotos por conveniencia. Validar ruta elegida por el usuario y formato; aplicar escrituras atómicas y no sobrescribir sin intención explícita.

## 6. Instalación y recuperación

Ensayar por usuario sin privilegios administrativos cuando sea posible y modo institucional por separado. Validar rutas con espacios y caracteres no ASCII, asociación de archivos si se aprueba, arranque desde acceso directo y perfil de usuario sin entorno de desarrollo. Desinstalar no borra proyectos sin consentimiento; explicitar política de configuración y caché. Registrar códigos de salida y un log sin secretos.

Separar firma de ejecutable/instalador de firma de actualización: no son intercambiables. No prometer desaparición de alertas SmartScreen. Conservar última versión válida y copia previa a migración; no abrir para escritura un proyecto de versión futura desconocida. Si actualización falla, no perder datos ni requerir red para abrir un proyecto local ya disponible.

## 7. Cadena de suministro y CI

Registrar toolchain, target, features, lockfile, versión del bundler, artefactos y licencias. Separar tests nativos de tests Wasm/UI: éxito nativo no acredita exports web. Jobs candidatos: formato/lints, pruebas de dominio, integración de traza, build web, UI navegador, análisis de dependencias y build desktop sólo si aprobado. Publicación requiere una puerta humana y credenciales mínimas; no ejecutar código de contribuciones no confiables con secretos de firma.

SBOM e inventario incluyen crates, glue, scripts, fuentes, iconos y recursos incorporados. Decidir licencia del propio proyecto después de cotejar LICENSE y manifiestos; D2 informa divergencias MIT frente a badge dual, no las resuelve. No retirar avisos ni deducir licencia del nombre del repositorio.

## Puerta G5

Instalación limpia online y offline según política elegida, apertura de proyecto, guardado, rechazo de corrupto, actualización, migración reversible, recuperación y desinstalación. Adjuntar hashes, firma si procede, matriz OS/runtime y logs. Este paquete no contiene un instalador ni ejecuta esas pruebas.
