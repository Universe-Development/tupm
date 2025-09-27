@echo off
setlocal

:: --- Configuration ---
set "OUTPUT_DIR=builds"
set "PROGRAM_NAME=tupm"
set "TARGET_OS="
set "GOOS_ENV="
set "OUTPUT_FILENAME="

:: --- Handle Input Argument ---
if "%1" == "" (
    set "TARGET_OS=windows"
) else if /i "%1" == "windows" (
    set "TARGET_OS=windows"
) else if /i "%1" == "linux" (
    set "TARGET_OS=linux"
) else (
    echo Error: Invalid target '%1'.
    echo Usage: %~n0 [target]
    echo   target: windows (Default) or linux
    goto :error
)

:: --- Set Go Build Environment Variables ---
if /i "%TARGET_OS%" == "windows" (
    set "GOOS_ENV=windows"
    set "OUTPUT_FILENAME=%PROGRAM_NAME%.exe"
) else (
    set "GOOS_ENV=linux"
    set "OUTPUT_FILENAME=%PROGRAM_NAME%"
)

:: --- Compilation Steps ---

:: 1. Create the output directory
echo Checking and creating output directory: %OUTPUT_DIR%
if not exist "%OUTPUT_DIR%" (
    mkdir "%OUTPUT_DIR%"
    if errorlevel 1 (
        echo Error: Failed to create directory %OUTPUT_DIR%.
        goto :error
    )
)

:: 2. Compile the Go program
echo Compiling for %TARGET_OS% (GOOS=%GOOS_ENV%) to %OUTPUT_DIR%\%OUTPUT_FILENAME%

:: The core cross-compilation command using GOOS
set GOOS=%GOOS_ENV%
go build -o "%OUTPUT_DIR%\%OUTPUT_FILENAME%" .
set GOOS= :: Clear GOOS environment variable after compilation

:: 3. Check the compilation status
if errorlevel 1 (
    echo ❌ Error: Go compilation failed.
    goto :error
) else (
    echo ✅ Success! Program compiled for %TARGET_OS% and placed at: %OUTPUT_DIR%\%OUTPUT_FILENAME%
)

goto :eof

:error
echo Script finished with errors.
endlocal
exit /b 1