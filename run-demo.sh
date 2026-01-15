#!/bin/bash
# Script to run the browser demo locally

set -e

# Check if python3 is available
if ! command -v python3 &> /dev/null; then
    echo "Error: python3 is not installed or not in PATH"
    echo "Please install Python 3 to run the local server"
    exit 1
fi

echo "Starting local server for Chord2MML Parser Demo..."
echo "Open your browser and navigate to: http://localhost:8000/demo/"
echo "Press Ctrl+C to stop the server"
echo ""

cd "$(dirname "$0")"
python3 -m http.server 8000
