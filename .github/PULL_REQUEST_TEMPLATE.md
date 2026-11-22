# Pull Request

## Tipo de cambio
- [ ] Corrección (bug fix)
- [ ] Nueva funcionalidad (feature)
- [ ] Refactor / Limpieza
- [ ] Documentación
- [ ] Performance
- [ ] CI / Build

## Descripción
<!-- Qué y por qué. Referencia a issue: Fixes #XXX -->

## Alcance
- [ ] `visu-core`
- [ ] `visu-wasm`
- [ ] `web/`
- [ ] `fixtures/`
- [ ] `docs/`
- [ ] `scripts/`
- [ ] `.github/`

## Verificación (Ejecutar Localmente)
- [ ] `cargo fmt --all -- --check` pasa
- [ ] `cargo clippy -p visu-core --all-targets --locked -- -D warnings` pasa
- [ ] `cargo test -p visu-core --locked` pasa
- [ ] `cargo build -p visu-wasm --target wasm32-unknown-unknown --locked` pasa
- [ ] Enlace Wasm (wasm-pack) genera artefactos en `web/pkg/`
- [ ] Tests nuevos/actualizados para cambios funcionales
- [ ] Docs actualizadas (README, docs/, doc comments)

## Evidencia
- [ ] Fixtures actualizados si cambia contrato de traza
- [ ] Hash Wasm registrado en `evidence/wasm-artifacts/`
- [ ] Benchmarks actualizados si aplica

## Breaking Changes
- [ ] No hay breaking changes
- [ ] Hay breaking changes: documentados en `BREAKING-CHANGES.md` y ADR correspondiente

## Checklist de Revisión
- [ ] Commits siguen Conventional Commits (`feat(core): ...`)
- [ ] PR size razonable (<50 archivos, <1000 líneas)
- [ ] Etiquetas `type:` y `scope:` presentes
- [ ] No secretos, paths absolutos, ni logs crudos en diff
