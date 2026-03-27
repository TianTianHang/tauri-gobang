@echo off
REM Sidecar download script for Rapfi engine (Windows)

setlocal enabledelayedexpansion

set SCRIPT_DIR=%~dp0
set BINARY_DIR=%SCRIPT_DIR%binaries
set RAPFI_VERSION=250615
set RAPFI_URL=https://github.com/dhbloo/rapfi/releases/download

if not exist "%BINARY_DIR%" mkdir "%BINARY_DIR%"

echo 📦 Downloading Rapfi engine (sidecar)...
cd /d "%BINARY_DIR%"

set BINARY_NAME=rapfi.exe
set DOWNLOAD_URL=%RAPFI_URL%/%RAPFI_VERSION%/rapfi-cli-%RAPFI_VERSION%-win-x64.zip

if not exist "%BINARY_NAME%" (
    echo ⬇️  Downloading from: !DOWNLOAD_URL!
    curl -L -o "rapfi.zip" "!DOWNLOAD_URL!"
    tar -xf "rapfi.zip"

    REM Get target triple for sidecar naming
    for /f "tokens=2" %%a in ('rustc -Vv ^| findstr host:') do set TARGET_TRIPLE=%%a
    echo 📝 Target triple: !TARGET_TRIPLE!

    ren rapfi-cli-*.exe rapfi.exe 2>nul
    copy "%BINARY_NAME%" "%BINARY_NAME%-!TARGET_TRIPLE!" 2>nul

    del /q "rapfi.zip"
    echo ✅ Downloaded: %BINARY_DIR%\%BINARY_NAME%
    echo ✅ Sidecar: %BINARY_DIR%\%BINARY_NAME%-!TARGET_TRIPLE!
) else (
    echo ✅ Already exists: %BINARY_DIR%\%BINARY_NAME%
)

if exist "%BINARY_NAME%" (
    echo.
    echo 🔍 Binary info:
    dir "%BINARY_NAME%"
    echo.
    echo ✨ Sidecar ready! The engine will be bundled with the application.
) else (
    echo ❌ Failed to download binary
    exit /b 1
)

endlocal
