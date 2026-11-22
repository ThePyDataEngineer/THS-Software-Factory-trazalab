#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Inicia servidor estático para web/ con artefactos Wasm (miniserve).
.NOTES
    WORKSPACE_ROOT: I:\THS Software Factory - Visu
    DEC-007: miniserve v0.25.0, puerto 8080, 127.0.0.1
#>

param(
    [int]$Port = 8080,
    [string]$Address = '127.0.0.1'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$Root = "I:\THS Software Factory - Visu"
$WebDir = "$Root/web"
$PkgDir = "$WebDir/pkg"

Write-Host "=== VISU · Serve Local (miniserve) ===" -ForegroundColor Cyan
Write-Host "Address: $Address:$Port" -ForegroundColor Gray
Write-Host "Serving: $WebDir" -ForegroundColor Gray

# Verificar artefactos
if (-not (Test-Path "$PkgDir/visu_wasm_bg.wasm") -and -not (Test-Path "$PkgDir/visu_wasm.wasm")) {
    throw "Artefacto Wasm no encontrado en $PkgDir. Ejecute Build Wasm primero."
}
if (-not (Test-Path "$PkgDir/visu_wasm.js")) {
    throw "Glue JS no encontrado en $PkgDir. Build Wasm incompleto."
}

# Verificar puerto libre
$PortInUse = Get-NetTCPConnection -LocalAddress $Address -LocalPort $Port -State Listen -ErrorAction SilentlyContinue
if ($PortInUse) {
    Write-Warning "Puerto $Port ya en uso (PID: $($PortInUse.OwningProcess)). Use -Port para cambiar."
}

# Verificar miniserve
if (-not (Get-Command miniserve -ErrorAction SilentlyContinue)) {
    throw "miniserve no instalado. Ejecute: cargo install miniserve@0.25.0 --locked"
}

cd $WebDir
miniserve --port $Port --address $Address .
