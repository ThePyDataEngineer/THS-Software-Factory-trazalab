#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Réplica local de pasos CI para Windows (MSVC toolchain).
.DESCRIPTION
    Ejecuta: fmt, clippy (trazalab-core), test (trazalab-core), wasm-build (trazalab-wasm).
    Alinea con .github/workflows/ci.yml. No ejecuta pruebas en navegador.
.NOTES
    WORKSPACE_ROOT: I:\THS Software Factory - TrazaLab
    Requiere: Rust 1.98.1, wasm32-unknown-unknown, wasm-pack 0.15.1.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$Root = "I:\THS Software Factory - TrazaLab"
Write-Host "=== WORKSPACE_ROOT: $Root ===" -ForegroundColor Cyan

function Check-Command($cmd, $versionFlag = '--version') {
    try {
        & $cmd $versionFlag 2>$null | Out-Null
        return $true
    } catch {
        return $false
    }
}

# --- Verificaciones previas ---
Write-Host "=== [0/4] Verificando toolchain ==="
if (-not (Check-Command 'rustc')) { throw "rustc no encontrado en PATH" }
if (-not (Check-Command 'cargo')) { throw "cargo no encontrado en PATH" }
rustc --version
cargo --version

$TargetInstalled = rustup target list --installed | Select-String 'wasm32-unknown-unknown'
if (-not $TargetInstalled) {
    throw "Target wasm32-unknown-unknown no instalado. Ejecute: rustup target add wasm32-unknown-unknown --toolchain 1.98.1"
}

# --- 1. Format ---
Write-Host "=== [1/4] Format (rustfmt) ==="
cd $Root
cargo fmt --all -- --check

# --- 2. Clippy (trazalab-core) ---
Write-Host "=== [2/4] Clippy (trazalab-core) ==="
cargo clippy -p trazalab-core --all-targets --locked -- -D warnings

# --- 3. Test Core ---
Write-Host "=== [3/4] Test Core (trazalab-core) ==="
cargo test -p trazalab-core --locked

# --- 4. Wasm Build (wasm-pack) ---
Write-Host "=== [4/4] Wasm Build (trazalab-wasm) - wasm-pack ==="
if (-not (Check-Command 'wasm-pack')) { throw "wasm-pack no encontrado. Instale: cargo install wasm-pack@0.15.1 --locked" }
cd "$Root/crates/trazalab-wasm"
wasm-pack build --release --target web --out-dir ../../web/pkg
cd $Root

# --- Verificación artefacto ---
Write-Host "=== Verificación artefacto ==="
$PkgDir = "$Root/web/pkg"
if (-not (Test-Path $PkgDir)) { throw "Directorio $PkgDir no existe tras build" }
Get-ChildItem $PkgDir | Format-Table Name, Length, LastWriteTime
$WasmFile = Get-ChildItem $PkgDir -Filter '*.wasm' | Select-Object -First 1
if ($WasmFile) {
    $Hash = Get-FileHash $WasmFile.FullName -Algorithm SHA256
    Write-Host "WASM: $($WasmFile.Name) - SHA256: $($Hash.Hash)" -ForegroundColor Green
    $Hash.Hash | Out-File "$Root/wasm-hash.txt" -Encoding utf8
}
else {
    throw "No se encontró archivo .wasm en $PkgDir"
}

Write-Host "=== CI LOCAL COMPLETO ===" -ForegroundColor Green
