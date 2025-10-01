@echo off
title Create Desktop Shortcut
color 0B

REM Change to the directory where this batch file is located
cd /d "%~dp0"

echo.
echo ========================================
echo    Create Desktop Shortcut
echo ========================================
echo.

REM Check if whatsappgpt.exe exists
if not exist "whatsappgpt.exe" (
    echo [ERROR] whatsappgpt.exe not found!
    echo Please ensure all files are in the same directory.
    echo.
    pause
    exit /b 1
)

echo [INFO] Creating desktop shortcut...
echo.

REM Get current directory and desktop path
set "CURRENT_DIR=%~dp0"
set "DESKTOP=%USERPROFILE%\Desktop"

REM Create desktop shortcut
echo [InternetShortcut] > "%DESKTOP%\WhatsappGPT.url"
echo URL=file:///%CURRENT_DIR%Start-WhatsappGPT.vbs >> "%DESKTOP%\WhatsappGPT.url"
echo IconFile=%CURRENT_DIR%whatsappgpt.exe >> "%DESKTOP%\WhatsappGPT.url"
echo IconIndex=0 >> "%DESKTOP%\WhatsappGPT.url"

echo [SUCCESS] Desktop shortcut created!
echo.
echo You can now find "WhatsappGPT" on your desktop.
echo Double-click it to start the application.
echo.
echo ========================================
echo SHORTCUT CREATED - Press any key to exit
echo ========================================
pause
