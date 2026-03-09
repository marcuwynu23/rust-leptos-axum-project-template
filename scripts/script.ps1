# Leptos + Axum project script
# Usage: pwsh scripts\script.ps1 <command>
#   install  - one-time setup (rustup target, cargo-leptos, check sass)
#   dev      - dev server with hot reload (cargo leptos watch)
#   build    - release build (server + site)
#   run      - run release server (after build)
#   check    - cargo check
#   clean      - remove target/
#   free-space - free disk space if install fails with "no space"

$ErrorActionPreference = "Stop"
$cmd = $args[0]

# OpenSSL install path when script downloads it (full build with .lib files for linking)
$OpenSSLInstallPath = "C:\OpenSSL-light"

function Show-Help {
    @"
Leptos + Axum script
Usage: pwsh scripts\script.ps1 <command>

Commands:
  install     One-time setup: wasm32 target, cargo-leptos, sass check
  dev         Start dev server with hot reload (cargo leptos watch)
  build       Release build (cargo leptos build --release)
  run         Run release server (cargo run -p server --release)
  check       Run cargo check
  clean       Remove target/

Examples:
  pwsh scripts\script.ps1 install
  pwsh scripts\script.ps1 dev
"@
}

# Returns the OpenSSL root path (with lib\libcrypto.lib) or $null if failed.
function Install-OpenSSLToPath {
    param([string]$TargetDir = $OpenSSLInstallPath)
    # Full Win64 OpenSSL installer (has lib\libcrypto.lib and lib\libssl.lib). Not the "Light" package.
    $urls = @(
        "https://slproweb.com/download/Win64OpenSSL-3_2_0.exe",
        "https://slproweb.com/download/Win64OpenSSL-3_3_0.exe"
    )
    $exe = Join-Path $env:TEMP "Win64OpenSSL-install.exe"
    foreach ($url in $urls) {
        try {
            Write-Host "Downloading OpenSSL from slproweb (full build)..." -ForegroundColor Yellow
            Invoke-WebRequest -Uri $url -OutFile $exe -UseBasicParsing -TimeoutSec 60
            if (Test-Path $exe) { break }
        } catch {
            Write-Host "  Failed: $url" -ForegroundColor Gray
        }
    }
    if (-not (Test-Path $exe)) {
        Write-Host "Could not download OpenSSL. Install manually from https://slproweb.com/products/Win32OpenSSL.html (full, not Light)." -ForegroundColor Red
        return $null
    }
    Write-Host "Installing OpenSSL to $TargetDir (silent)..." -ForegroundColor Yellow
    $psi = @{
        FilePath = $exe
        ArgumentList = "/VERYSILENT", "/SUPPRESSMSGBOXES", "/DIR=$TargetDir"
        Wait = $true
        PassThru = $true
    }
    $proc = Start-Process @psi
    Remove-Item $exe -Force -ErrorAction SilentlyContinue
    if ($proc.ExitCode -ne 0) {
        Write-Host "OpenSSL installer exited with code $($proc.ExitCode). Try running as Administrator or install manually." -ForegroundColor Yellow
        return $null
    }
    $libCrypto = Join-Path $TargetDir "lib\libcrypto.lib"
    $libSsl   = Join-Path $TargetDir "lib\libssl.lib"
    if ((Test-Path $libCrypto) -and (Test-Path $libSsl)) {
        Write-Host "OpenSSL installed at $TargetDir" -ForegroundColor Green
        return $TargetDir
    }
    # Installer may have created a versioned subfolder (e.g. OpenSSL-Win64)
    $sub = Get-ChildItem -Path $TargetDir -Directory -ErrorAction SilentlyContinue | Where-Object {
        (Test-Path (Join-Path $_.FullName "lib\libcrypto.lib")) -and (Test-Path (Join-Path $_.FullName "lib\libssl.lib"))
    } | Select-Object -First 1
    if ($sub) {
        Write-Host "OpenSSL installed at $($sub.FullName)" -ForegroundColor Green
        return $sub.FullName
    }
    Write-Host "OpenSSL may have installed to a different path. Check $TargetDir for lib\libcrypto.lib and lib\libssl.lib" -ForegroundColor Yellow
    return $null
}

function Script-Install {
    Write-Host "=== Install (one-time setup) ===" -ForegroundColor Cyan

    Write-Host "`n[1/3] Adding wasm32 target..." -ForegroundColor Yellow
    rustup target add wasm32-unknown-unknown

    Write-Host "`n[2/3] Installing cargo-leptos..." -ForegroundColor Yellow
    # On Windows, openssl-sys needs Perl to build from source. Use system OpenSSL if no Perl.
    if ($env:OS -eq "Windows_NT" -and -not (Get-Command perl -ErrorAction SilentlyContinue)) {
        $opensslPaths = @(
            $OpenSSLInstallPath,
            "C:\Program Files\OpenSSL-Win64",
            "C:\Program Files\OpenSSL",
            "C:\OpenSSL-Win64"
        )
        $found = $null
        $foundLibDir = $null
        foreach ($p in $opensslPaths) {
            $libCrypto = Join-Path $p "lib\libcrypto.lib"
            $libSsl   = Join-Path $p "lib\libssl.lib"
            if ((Test-Path $libCrypto) -and (Test-Path $libSsl)) {
                $found = $p
                break
            }
            # Some installers put .lib in lib\VC\x64\MD or lib\VC\x64\MT
            $vcSubdirs = @("lib\VC\x64\MD", "lib\VC\x64\MT", "lib\VC\x64\MDd", "lib\VC\x64\MTd")
            foreach ($sub in $vcSubdirs) {
                $libDir = Join-Path $p $sub
                $libCryptoVc = Join-Path $libDir "libcrypto.lib"
                $libSslVc   = Join-Path $libDir "libssl.lib"
                if ((Test-Path $libCryptoVc) -and (Test-Path $libSslVc)) {
                    $found = $p
                    $foundLibDir = $libDir
                    break
                }
            }
            if ($found) { break }
        }
        if ($found) {
            $env:OPENSSL_NO_VENDOR = "1"
            $env:OPENSSL_DIR = $found
            if ($foundLibDir) {
                $env:OPENSSL_LIB_DIR = $foundLibDir
                Write-Host "Using system OpenSSL at: $found (lib: $foundLibDir)" -ForegroundColor Gray
            } else {
                Write-Host "Using system OpenSSL at: $found" -ForegroundColor Gray
            }
        } else {
            Write-Host "Perl not found. Downloading OpenSSL (full build) to $OpenSSLInstallPath ..." -ForegroundColor Yellow
            $installedPath = Install-OpenSSLToPath -TargetDir $OpenSSLInstallPath
            if ($installedPath) {
                $env:OPENSSL_NO_VENDOR = "1"
                $env:OPENSSL_DIR = $installedPath
            } else {
                Write-Host "  Fallback: Install Strawberry Perl (https://strawberryperl.com/) or full OpenSSL from https://slproweb.com/products/Win32OpenSSL.html" -ForegroundColor Yellow
                Write-Host "  Then: `$env:OPENSSL_NO_VENDOR='1'; `$env:OPENSSL_DIR='C:\Program Files\OpenSSL-Win64'; `$env:OPENSSL_LIB_DIR='C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD'; pwsh scripts\script.ps1 install" -ForegroundColor Yellow
                Write-Host "`nTrying cargo install anyway (may fail)..." -ForegroundColor Gray
            }
        }
    }
    cargo install cargo-leptos --locked
    $installOk = ($LASTEXITCODE -eq 0)
    if (-not $installOk -and -not (Test-Path (Join-Path $env:USERPROFILE ".cargo\bin\cargo-leptos.exe"))) {
        Write-Host "`ncargo-leptos install failed." -ForegroundColor Red
        Write-Host "  If error was 'perl not found': install Strawberry Perl (https://strawberryperl.com/)" -ForegroundColor Yellow
        Write-Host "  If error was 'does not contain the required files': you need the FULL OpenSSL (not Light) from" -ForegroundColor Yellow
        Write-Host "  https://slproweb.com/products/Win32OpenSSL.html - the full build has lib\libcrypto.lib and lib\libssl.lib." -ForegroundColor Yellow
        Write-Host "  Then run: `$env:OPENSSL_NO_VENDOR='1'; `$env:OPENSSL_DIR='C:\Program Files\OpenSSL-Win64'; `$env:OPENSSL_LIB_DIR='C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD'; pwsh scripts\script.ps1 install" -ForegroundColor Yellow
        exit 1
    }
    if (-not (Get-Command cargo-leptos -ErrorAction SilentlyContinue)) {
        $bin = Join-Path $env:USERPROFILE ".cargo\bin"
        if (Test-Path (Join-Path $bin "cargo-leptos.exe")) {
            Write-Host "cargo-leptos installed at $bin - ensure that path is in your PATH." -ForegroundColor Yellow
        }
    }

    Write-Host "`n[3/3] Checking sass..." -ForegroundColor Yellow
    if (Get-Command sass -ErrorAction SilentlyContinue) {
        Write-Host "sass found: $(Get-Command sass | Select-Object -ExpandProperty Source)" -ForegroundColor Green
    } else {
        Write-Host "sass not found. Install for SCSS: choco install sass, or npm install -g sass, or https://sass-lang.com/install" -ForegroundColor Yellow
    }

    Write-Host "`nDone. Run: pwsh scripts\script.ps1 dev" -ForegroundColor Green
}

function Script-Dev {
    Write-Host "=== Dev server (cargo leptos watch) ===" -ForegroundColor Cyan
    Write-Host "Open http://127.0.0.1:3000" -ForegroundColor Gray
    cargo leptos watch
}

function Script-Build {
    Write-Host "=== Release build ===" -ForegroundColor Cyan
    cargo leptos build --release
    Write-Host "Server: target\server\release\server.exe" -ForegroundColor Green
    Write-Host "Site:   target\site\" -ForegroundColor Green
}

function Script-Run {
    Write-Host "=== Run release server ===" -ForegroundColor Cyan
    if (-not (Test-Path "target\server\release\server.exe")) {
        Write-Host "Run 'pwsh scripts\script.ps1 build' first." -ForegroundColor Yellow
        exit 1
    }
    Write-Host "Open http://127.0.0.1:3000" -ForegroundColor Gray
    cargo run -p server --release
}

function Script-Check {
    Write-Host "=== Cargo check ===" -ForegroundColor Cyan
    cargo check --workspace --exclude server --exclude frontend
    cargo check -p server
}

function Script-Clean {
    Write-Host "=== Clean (remove target/) ===" -ForegroundColor Cyan
    if (Test-Path "target") {
        Remove-Item -Recurse -Force target
        Write-Host "Removed target\" -ForegroundColor Green
    } else {
        Write-Host "No target\ folder." -ForegroundColor Gray
    }
}

switch ($cmd) {
    "install" { Script-Install }
    "dev"     { Script-Dev }
    "build"   { Script-Build }
    "run"     { Script-Run }
    "check"      { Script-Check }
    "clean"      { Script-Clean }
    default      {
        if ($cmd -eq "" -or $cmd -eq "-h" -or $cmd -eq "--help") {
            Show-Help
        } else {
            Write-Host "Unknown command: $cmd" -ForegroundColor Red
            Show-Help
            exit 1
        }
    }
}
