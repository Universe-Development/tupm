@echo off
setlocal

:: Define the output directory and filename
set "OUTPUT_DIR=builds"
set "OUTPUT_FILENAME=tupm.exe"

:: 1. Create the output directory if it doesn't exist
echo Checking and creating output directory: %OUTPUT_DIR%
if not exist "%OUTPUT_DIR%" (
    mkdir "%OUTPUT_DIR%"
    if errorlevel 1 (
        echo Error: Failed to create directory %OUTPUT_DIR%.
        goto :error
    )
)

:: 2. Compile the Go program (assuming the main package is in the current directory)
echo Compiling Go program to %OUTPUT_DIR%\%OUTPUT_FILENAME%
go build -o "%OUTPUT_DIR%\%OUTPUT_FILENAME%" .

:: 3. Check the compilation status
if errorlevel 1 (
    echo ❌ Error: Go compilation failed.
    goto :error
) else (
    echo ✅ Success! Program compiled and placed at: %OUTPUT_DIR%\%OUTPUT_FILENAME%
)

goto :eof

:error
echo Script finished with errors.
endlocal
exit /b 1