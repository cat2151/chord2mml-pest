#!/bin/bash
# Script to run the browser demo locally

echo "Starting local server for Chord2MML Parser Demo..."
echo "Open your browser and navigate to: http://localhost:8000/demo/"
echo "Press Ctrl+C to stop the server"
echo ""

cd "$(dirname "$0")"
python3 -m http.server 8000
