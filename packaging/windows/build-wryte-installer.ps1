<#
Builds the Wryte release: the app, its installer, and the zip published on GitHub.

  ./packaging/windows/build-wryte-installer.ps1 [-Iscc <path to ISCC.exe>]

Needs Rust and Inno Setup 6. Output: target/installer/Wryte-Setup-<version>.exe and
target/installer/Wryte-v<version>-Windows-x64.zip.

Local folder names (your user folder, the checkout's location) are mapped to neutral names in the
compiled program, so the executable carries no paths from the machine that built it.
#>
param([string]$Iscc)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$version = (Select-String -Path (Join-Path $root 'Cargo.toml') -Pattern '^version = "([^"]+)"').Matches[0].Groups[1].Value

$cargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $env:USERPROFILE '.cargo' }
$rustupHome = if ($env:RUSTUP_HOME) { $env:RUSTUP_HOME } else { Join-Path $env:USERPROFILE '.rustup' }
# rustc applies the last matching rule, so the most specific folders come last.
$env:RUSTFLAGS = "--remap-path-prefix=$env:USERPROFILE=home --remap-path-prefix=$rustupHome=rustup --remap-path-prefix=$cargoHome=cargo --remap-path-prefix=$root=wryte"
Push-Location $root
try {
    # Cargo reports progress and warnings on stderr; only its exit code decides.
    $ErrorActionPreference = 'Continue'
    cargo build --release -p wryte
    $ErrorActionPreference = 'Stop'
    if ($LASTEXITCODE -ne 0) { throw 'cargo build failed' }
} finally { Pop-Location }

if (-not $Iscc) {
    $Iscc = @("$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe", "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe", "$env:ProgramFiles\Inno Setup 6\ISCC.exe") |
        Where-Object { Test-Path $_ } | Select-Object -First 1
}
if (-not $Iscc) { throw 'Inno Setup 6 (ISCC.exe) not found; pass -Iscc' }
& $Iscc /Q "/DAppVersion=$version" "/DSourceDir=$root\target\release" (Join-Path $PSScriptRoot 'wryte.iss')
if ($LASTEXITCODE -ne 0) { throw 'Inno Setup failed' }

$out = Join-Path $root 'target\installer'
$setup = Join-Path $out "Wryte-Setup-$version.exe"
$zip = Join-Path $out "Wryte-v$version-Windows-x64.zip"
if (Test-Path $zip) { Remove-Item $zip }
Compress-Archive -Path $setup -DestinationPath $zip
Get-Item $setup, $zip | Select-Object Name, Length
