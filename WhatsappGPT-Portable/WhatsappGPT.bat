@echo off
REM Change to the directory where this batch file is located
cd /d "%~dp0"

REM Check if whatsappgpt.exe exists
if not exist "whatsappgpt.exe" (
    echo [ERROR] whatsappgpt.exe not found!
    echo Please ensure all files are in the same directory.
    echo.
    pause
    exit /b 1
)

REM Start the application using VBScript to hide console
cscript //nologo "Start-WhatsappGPT.vbs"
