# Captures the four README proof screenshots against the running exporter using
# headless Microsoft Edge. Replaces the older Python SVG-mock generator so the
# README screenshots reflect what the real running page looks like.

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$screenshots = Join-Path $repoRoot "screenshots"
$stdout = Join-Path $screenshots "app.stdout.log"
$stderr = Join-Path $screenshots "app.stderr.log"
$port = "4978"
$process = $null
$edgeCandidates = @(
    "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    "C:\Program Files\Microsoft\Edge\Application\msedge.exe"
)

New-Item -ItemType Directory -Force -Path $screenshots | Out-Null

function Get-EdgePath {
    foreach ($candidate in $edgeCandidates) {
        if (Test-Path $candidate) {
            return $candidate
        }
    }
    throw "Microsoft Edge was not found."
}

function Wait-ForUrl {
    param([string]$Url)
    for ($i = 0; $i -lt 90; $i++) {
        try {
            Invoke-WebRequest -Uri $Url -UseBasicParsing | Out-Null
            return
        } catch {
            Start-Sleep -Seconds 1
        }
    }
    throw "Timed out waiting for $Url"
}

try {
    $env:PORT = $port
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

    # Make sure the release binary is ready; building inline keeps the script
    # idempotent regardless of how the repo was last touched.
    Push-Location $repoRoot
    cargo build --release | Out-Null
    Pop-Location

    $exe = Join-Path $repoRoot "target\release\cyberark-connector-observability-exporter.exe"
    if (-not (Test-Path $exe)) {
        throw "Release binary not found at $exe"
    }

    $process = Start-Process -FilePath $exe `
        -WorkingDirectory $repoRoot `
        -RedirectStandardOutput $stdout `
        -RedirectStandardError $stderr `
        -PassThru

    Wait-ForUrl "http://127.0.0.1:$port/"

    $edge = Get-EdgePath
    $targets = @(
        @{ Url = "http://127.0.0.1:$port/";                 File = "01-overview-proof.png";            Size = "1600,1180" },
        @{ Url = "http://127.0.0.1:$port/connectors";       File = "02-connector-board-proof.png";     Size = "1600,1180" },
        @{ Url = "http://127.0.0.1:$port/audit";            File = "03-audit-log-proof.png";           Size = "1600,1120" },
        @{ Url = "http://127.0.0.1:$port/metrics-preview";  File = "04-metrics-proof.png";             Size = "1600,1180" }
    )

    foreach ($target in $targets) {
        & $edge `
            --headless `
            --disable-gpu `
            --hide-scrollbars `
            "--window-size=$($target.Size)" `
            "--screenshot=$(Join-Path $screenshots $target.File)" `
            $target.Url | Out-Null
    }
} finally {
    if ($process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force
    }
}
