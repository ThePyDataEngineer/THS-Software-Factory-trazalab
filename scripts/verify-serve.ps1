#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Verifica que el servidor local sirve correctamente los artefactos Wasm.
.NOTES
    Ejecutar MIENTRAS el servidor está corriendo (scripts/serve-local.ps1 en otra terminal).
#>

param(
    [int]$Port = 8080,
    [string]$Address = '127.0.0.1'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$BaseUrl = "http://$Address:$Port"
$Root = "I:\THS Software Factory - TrazaLab"
$PkgDir = "$Root/web/pkg"

Write-Host "=== Verificando servicio en $BaseUrl ===" -ForegroundColor Cyan

function Test-Endpoint($url, $expectedType) {
    try {
        $resp = Invoke-WebRequest -Uri $url -Method Head -UseBasicParsing -ErrorAction Stop
        $ok = $resp.StatusCode -eq 200 -and $resp.ContentType -like "$expectedType*"
        [pscustomobject]@{
            Url = $url
            Status = $resp.StatusCode
            ContentType = $resp.ContentType
            Pass = $ok
        }
    } catch {
        [pscustomobject]@{
            Url = $url
            Status = $_.Exception.Response.StatusCode.value__
            ContentType = $null
            Pass = $false
        }
    }
}

# 1. HTML
$r1 = Test-Endpoint "$BaseUrl/index.html" 'text/html'
# 2. JS bootstrap
$r2 = Test-Endpoint "$BaseUrl/index.js" 'application/javascript'
# 3. Wasm
$r3 = Test-Endpoint "$BaseUrl/pkg/trazalab_wasm_bg.wasm" 'application/wasm'
if (-not $r3.Pass) {
    $r3 = Test-Endpoint "$BaseUrl/pkg/trazalab_wasm.wasm" 'application/wasm'
}
# 4. Glue JS
$r4 = Test-Endpoint "$BaseUrl/pkg/trazalab_wasm.js" 'application/javascript'

$r1, $r2, $r3, $r4 | Format-Table Url, Status, ContentType, Pass

# Verificar hash Wasm
$WasmFile = Get-ChildItem $PkgDir -Filter '*.wasm' | Select-Object -First 1
if ($WasmFile) {
    $LocalHash = (Get-FileHash $WasmFile.FullName -Algorithm SHA256).Hash
    Write-Host "Hash local Wasm: $LocalHash" -ForegroundColor Green
    # Comparar con evidence/wasm-artifacts/ si existe
    $EvidenceHashFile = Get-ChildItem "$Root/evidence/wasm-artifacts" -Filter '*.sha256' | Select-Object -Last 1
    if ($EvidenceHashFile) {
        $EvidenceHash = (Get-Content $EvidenceHashFile.FullName).Split(' ')[0]
        if ($LocalHash -eq $EvidenceHash) {
            Write-Host "✓ Hash coincide con evidence/wasm-artifacts/" -ForegroundColor Green
        } else {
            Write-Warning "✗ Hash NO coincide con evidence: $EvidenceHash"
        }
    }
}

$AllPass = $r1.Pass -and $r2.Pass -and $r3.Pass -and $r4.Pass
if ($AllPass) {
    Write-Host "=== SERVICIO VERIFICADO — LISTO PARA EDGE CHECK ===" -ForegroundColor Green
    exit 0
} else {
    Write-Host "=== SERVICIO CON ERRORES — REVISAR ===" -ForegroundColor Red
    exit 1
}
