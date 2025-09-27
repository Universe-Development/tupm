#!/bin/bash

# Define the output directory and filename
OUTPUT_DIR="builds"
OUTPUT_FILENAME="tupm"

# 1. Create the output directory if it doesn't exist
echo "Checking and creating output directory: $OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Check if mkdir was successful
if [ $? -ne 0 ]; then
    echo "Error: Failed to create directory $OUTPUT_DIR." >&2
    exit 1
fi

# 2. Compile the Go program (assuming the main package is in the current directory)
echo "Compiling Go program to $OUTPUT_DIR/$OUTPUT_FILENAME"
go build -o "$OUTPUT_DIR/$OUTPUT_FILENAME" .

# 3. Check the compilation status
if [ $? -eq 0 ]; then
    echo "✅ Success! Program compiled and placed at: $OUTPUT_DIR/$OUTPUT_FILENAME"
else
    echo "❌ Error: Go compilation failed." >&2
    exit 1
fi