#!/bin/bash

# --- Configuration ---
OUTPUT_DIR="builds"
PROGRAM_NAME="tupm"
TARGET_OS=""

# --- Function to display help ---
show_help() {
    echo "Usage: $0 [target]"
    echo ""
    echo "target: The operating system to compile for."
    echo "  - linux   (Default) Compiles for Linux."
    echo "  - windows Compiles for Windows (produces tupm.exe)."
    echo ""
    exit 0
}

# --- Handle Input Argument ---
if [ -z "$1" ]; then
    TARGET_OS="linux"
elif [ "$1" == "linux" ]; then
    TARGET_OS="linux"
elif [ "$1" == "windows" ]; then
    TARGET_OS="windows"
elif [ "$1" == "-h" ] || [ "$1" == "--help" ]; then
    show_help
else
    echo "Error: Invalid target '$1'." >&2
    show_help
    exit 1
fi

# --- Set Go Build Environment Variables ---
if [ "$TARGET_OS" == "windows" ]; then
    GOOS_ENV="windows"
    OUTPUT_FILENAME="${PROGRAM_NAME}.exe"
else
    GOOS_ENV="linux"
    OUTPUT_FILENAME="${PROGRAM_NAME}"
fi

# --- Compilation Steps ---

# 1. Create the output directory
echo "Checking and creating output directory: $OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR" || { echo "Error: Failed to create directory $OUTPUT_DIR."; exit 1; }

# 2. Compile the Go program
echo "Compiling for $TARGET_OS (GOOS=$GOOS_ENV) to $OUTPUT_DIR/$OUTPUT_FILENAME"

# The core cross-compilation command
GOOS="$GOOS_ENV" go build -o "$OUTPUT_DIR/$OUTPUT_FILENAME" .

# 3. Check the compilation status
if [ $? -eq 0 ]; then
    echo "✅ Success! Program compiled for $TARGET_OS and placed at: $OUTPUT_DIR/$OUTPUT_FILENAME"
else
    echo "❌ Error: Go compilation failed." >&2
    exit 1
fi