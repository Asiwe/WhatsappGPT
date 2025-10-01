@echo off
title Remove Desktop Shortcut
color 0C

REM Change to the directory where this batch file is located
cd /d "%~dp0"

echo.
echo ========================================
echo    Remove Desktop Shortcut
echo ========================================
echo.

set "DESKTOP=%USERPROFILE%\Desktop"

REM Check if shortcut exists
if exist "%DESKTOP%\WhatsappGPT.url" (
    echo [INFO] Found desktop shortcut.
    echo.
    del "%DESKTOP%\WhatsappGPT.url" 2>nul
    echo [SUCCESS] Desktop shortcut removed!
) else (
    echo [INFO] No desktop shortcut found.
)

echo.
echo ========================================
echo SHORTCUT REMOVED - Press any key to exit
echo ========================================
pause
