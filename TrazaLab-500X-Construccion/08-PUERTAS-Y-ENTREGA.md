# Puertas, expediente y promoción

| Gate | Evidencia mínima | Veto |
|---|---|---|
| G0 | SHA, inventario, build/serve reproducibles y perfil | No se puede fijar lo que se prueba |
| G1 | Recorrido y casos de fallo; corrección acotada si corresponde | Motor/entrada/ordenamiento no distinguidos |
| G2 | Oráculo, equivalencia de replay, rechazo de trazas inválidas | Estado no reproducible o parcial vendido como completo |
| G3 | Datos crudos, presupuesto comparable y dictamen por KPI | Degradación de modalidad o promesa sin datos |
| G4 | Comparación visual y accesible; exportación fiel | Fuga o modificación no aprobada del canon |
| G5 | Artefacto y matriz Windows, permisos y recuperación | Instalación/actualización insegura o no reproducida |
| G6 | Segundo productor y equivalencia declarada | Soporte o ejecución no demostrados |

Registrar PASS/FAIL/BLOCKED/NOT_RUN/N/A con motivo. Ninguna puerta hereda PASS de otro SHA. Evidencia por caso: SHA, hash, entorno, comando, código de salida, esperado, observado, archivo, responsable, revisor. Preservar fallos y divergencias. Para entrega de candidata: diff pequeño por frontera, notas de compatibilidad, prueba de reversión local y riesgo residual. No autorizar promoción humana con casillas vacías. Mantener sólo evidencias sanitizadas dentro del paquete de entrega.

Fuentes de diseño: informes TrazaLab 01–03 del 24-11-2026; propuesta TrazaLab 500X anterior; Cargo Book; MDN Workers; documentación Tauri. No se auditó el repositorio privado, no se compiló ni se ejecutó el producto, y no se cambió ningún estado remoto. Se verificaron únicamente artefactos documentales y el fixture simulado.
