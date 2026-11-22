# TrazaLab

Visualizador web de algoritmos y estructuras de datos con motor Rust/WebAssembly.

[![CI](https://github.com/ThePyDataEngineer/THS-Software-Factory-trazalab/actions/workflows/ci.yml/badge.svg)](https://github.com/ThePyDataEngineer/THS-Software-Factory-trazalab/actions/workflows/ci.yml)

TrazaLab permite estudiar **cómo** un algoritmo transforma sus datos, no solo ver el resultado final. Su unidad de navegación es la **traza**: una secuencia ordenada de pasos que permite examinar qué ocurrió, en qué orden y cómo cambió el estado de los datos.

El proyecto está dirigido a estudiantes que quieren entender un algoritmo, docentes que necesitan explicar su ejecución y desarrolladores que quieren revisar un ejemplo paso a paso. El primer incremento se concentra en un caso verificable: el ordenamiento burbuja de un arreglo de enteros.

TrazaLab es un proyecto independiente inspirado en [Visu, de iIonel](https://github.com/iIonel/Visu). No es una versión oficial ni está afiliado al proyecto original.

## Índice

- [Qué problema resuelve](#qué-problema-resuelve)
- [Alcance de la v0.1](#alcance-de-la-v01)
- [Ejemplo de lectura de una traza](#ejemplo-de-lectura-de-una-traza)
- [Arquitectura y estructura del repositorio](#arquitectura-y-estructura-del-repositorio)
- [Instalación y ejecución local](#instalación-y-ejecución-local)
- [Primer recorrido por la aplicación](#primer-recorrido-por-la-aplicación)
- [Estado de verificación y limitaciones](#estado-de-verificación-y-limitaciones)
- [Desarrollo y revisión de cambios](#desarrollo-y-revisión-de-cambios)
- [Cómo reportar un problema](#cómo-reportar-un-problema)
- [Atribución y licencia](#atribución-y-licencia)

## Qué problema resuelve

Una salida ordenada indica que el arreglo terminó en el estado esperado, pero no explica cómo llegó allí. Para entender el algoritmo hay que poder observar el estado inicial, las operaciones intermedias y el estado final.

TrazaLab organiza ese recorrido alrededor de tres preguntas:

1. **¿Con qué datos comenzó la ejecución?** Identifica el arreglo de entrada antes de cualquier transformación.
2. **¿Qué cambió en este paso?** Distingue una comparación de un intercambio y examina el arreglo antes y después de una operación.
3. **¿Por qué el resultado terminó así?** Relaciona el estado final con los cambios que se observaron durante la traza.

En este contexto, *determinista* significa que una misma entrada válida, con la misma versión del motor y la misma configuración, debe producir el mismo resultado y la misma secuencia de pasos. Es un criterio del motor, no una afirmación de que se hayan superado las pruebas de una revisión concreta.

## Alcance de la v0.1

La presentación del proyecto habla de algoritmos y estructuras de datos en general. El primer incremento tiene un alcance más acotado:

| Área | Alcance del incremento |
| --- | --- |
| Entrada | Edición de un arreglo de enteros desde la interfaz web. |
| Validación | Aceptar entradas conformes y rechazar entradas que no se ajusten al formato requerido, con un diagnóstico que permita corregirlas. |
| Motor | Ordenamiento burbuja determinista implementado en Rust y compilado a WebAssembly para ejecutarse en el navegador. |
| Traza | Recorrido paso a paso, con navegación al primer intercambio cuando exista y al estado final. |
| Entorno de referencia | Windows y Microsoft Edge 158.0.4370.33, según el alcance documentado del incremento. |

La tabla describe qué cubre el incremento, no qué pasó una prueba en la revisión actual. No implica que otros algoritmos, lenguajes, navegadores o entornos ya estén implementados o verificados.

## Ejemplo de lectura de una traza

Supón que queremos ordenar de menor a mayor el arreglo `[4, 1, 3]`. El ordenamiento burbuja compara elementos adyacentes y los intercambia cuando están fuera de orden.

| Paso ilustrativo | Operación | Estado del arreglo |
| --- | --- | --- |
| Inicio | Se recibe el arreglo original. | `[4, 1, 3]` |
| 1 | Se comparan `4` y `1`; se intercambian. | `[1, 4, 3]` |
| 2 | Se comparan `4` y `3`; se intercambian. | `[1, 3, 4]` |
| 3 | Se comparan `1` y `3`; no se intercambian. | `[1, 3, 4]` |
| Fin | El arreglo queda ordenado. | `[1, 3, 4]` |

Este ejemplo explica el algoritmo; no es una captura ni una transcripción de la aplicación. El motor puede representar la comparación y el intercambio como eventos separados. Lo importante es distinguir una operación que cambia el arreglo de otra que lo deja igual.

## Arquitectura y estructura del repositorio

| Componente | Responsabilidad |
| --- | --- |
| Motor Rust | Implementar el algoritmo y generar la secuencia de pasos de la ejecución. |
| WebAssembly (Wasm) | Permitir que el motor compilado se ejecute en el navegador. |
| Interfaz web | Recibir la entrada y presentar el estado del arreglo y los pasos de la traza. |
| Integración continua (CI) | Exponer las ejecuciones del workflow enlazado por el badge. Su estado no sustituye una verificación manual de la interfaz. |

Puntos de entrada para orientarse en el repositorio:

- [`README.md`](README.md): presentación, alcance y estado documental.
- [`web/index.html`](web/index.html): punto de entrada de la interfaz web.
- [`LICENSE`](LICENSE): texto de licencia y avisos aplicables al repositorio.
- [Workflow de CI](https://github.com/ThePyDataEngineer/THS-Software-Factory-trazalab/actions/workflows/ci.yml): historial de ejecuciones y detalle de los resultados.

## Instalación y ejecución local

**Prerrequisitos:** Rust 1.98.1, target `wasm32-unknown-unknown`, wasm-pack 0.15.1, miniserve 0.25.0.

```powershell
cd I:\THS Software Factory - TrazaLab
cargo fmt --all -- --check
cargo clippy -p trazalab-core --all-targets --locked
cargo test -p trazalab-core --locked
cargo build -p trazalab-wasm --target wasm32-unknown-unknown --locked
cd crates\trazalab-wasm
wasm-pack build --release --target web --out-dir ../../web/pkg
cd ..\..\web
miniserve --port 8080 --address 127.0.0.1 .
# Abrir http://127.0.0.1:8080 en Edge → ejecutar TRAZA-T01 a T04
```

Ver [docs/reproduction/BUILD.md](docs/reproduction/BUILD.md), [TEST.md](docs/reproduction/TEST.md), [SERVE.md](docs/reproduction/SERVE.md).

## Primer recorrido por la aplicación

Una vez iniciada la aplicación con el procedimiento de la revisión que estés usando:

1. Introduce un arreglo pequeño de enteros, por ejemplo `4`, `1` y `3`, usando el formato que solicite el editor.
2. Selecciona **Validar** y revisa si la interfaz acepta la entrada o indica qué hay que corregir.
3. Si la interfaz habilita **Ordenar**, ejecútalo y recorre la traza. Identifica el arreglo inicial, el primer intercambio y el arreglo final.
4. Compara lo que observaste con el resultado esperado. Para el ejemplo anterior, el estado final esperado es `[1, 3, 4]`.

Si **Ordenar** permanece deshabilitado después de validar una entrada correcta, detén el recorrido en ese punto. Registra el comportamiento como observado; no lo confundas con un fallo demostrado del motor, porque todavía no se ha comprobado que el motor haya recibido la entrada.

## Estado de verificación y limitaciones

- **Validar → Ordenar.** En una comprobación anterior se reportó que el botón **Ordenar** no se habilitó después de validar. Este README no certifica que el problema se haya corregido ni atribuye su causa al navegador o al motor.
- **Compatibilidad.** Windows y Edge son el entorno de referencia documentado. La compatibilidad con Chrome, Firefox u otros entornos no se da por verificada en este documento.
- **Revisión evaluada.** Para afirmar que un comportamiento fue verificado, registra el commit, el entorno, la entrada usada y el resultado observado. Una captura de otra revisión o un estado de CI sin ese contexto no sustituye la comprobación.
- **Alcance funcional.** El ejemplo de ordenamiento burbuja no implica que exista un catálogo de algoritmos ni una interfaz genérica para ejecutar programas en otros lenguajes.

## Desarrollo y revisión de cambios

Al revisar un cambio, identifica primero qué parte afecta: documentación, interfaz, motor, traza o procedimiento de construcción. Un cambio de marca en el README no acredita un cambio en la interfaz ni en el comportamiento del motor.

| Tipo de cambio | Qué conviene cotejar |
| --- | --- |
| Identidad y documentación | Nombre `TrazaLab`, enlaces al repositorio correcto, atribución a Visu y correspondencia entre el alcance publicado y la revisión. |
| Interfaz web | Entrada aceptada y rechazada, mensajes de validación, habilitación de controles y navegación por la traza. |
| Motor o traza | Resultado final, secuencia de eventos y repetibilidad con la misma entrada y configuración. |
| Construcción o CI | Procedimiento aplicable a la revisión, workflow ejecutado y resultado asociado al commit. |

En GitHub, el repositorio es `ThePyDataEngineer/THS-Software-Factory-trazalab`. `TrazaLab` es el nombre del producto; no debe sustituirse la ruta del repositorio por `ThePyDataEngineer/trazalab`.

## Cómo reportar un problema

Un reporte es más útil si permite repetir lo que ocurrió. Incluye:

- **Revisión:** rama y hash del commit, si están disponibles.
- **Entorno:** sistema operativo, navegador y versión. Si ejecutaste la aplicación localmente, indica cómo la serviste.
- **Entrada exacta:** valores y formato que escribiste.
- **Pasos:** acciones efectuadas hasta llegar al problema.
- **Resultado esperado y observado:** por ejemplo, «Esperaba que se habilitara Ordenar; permaneció deshabilitado después de validar».
- **Evidencia:** mensaje de error, captura o registro pertinente, sin credenciales ni datos sensibles.

## Atribución y licencia

TrazaLab está inspirado en [Visu, de iIonel](https://github.com/iIonel/Visu), que sirve como referencia para la visualización de algoritmos. La atribución reconoce el proyecto de referencia; no implica afiliación ni convierte las licencias o avisos de terceros en licencias propias de TrazaLab.

Consulta [`LICENSE`](LICENSE) para conocer los términos incluidos en esta revisión. Los componentes y recursos de terceros conservan sus licencias y avisos aplicables.
