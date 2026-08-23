#!/bin/bash

# Exit on any error
set -e

# Default size if not provided as an argument
SIZE=${1:-"640x480"}
PYTHON_BIN="python3"

echo "========================================="
echo " rendering scenes at size: $SIZE"
echo "========================================="

# Render Spheres3 scene
echo "Rendering scenes/spheres3.json -> output_spheres3.bmp"
$PYTHON_BIN src/btrace.py --scene scenes/spheres3.json --size $SIZE --outfile output_spheres3.bmp

# Render Checkers scene
echo "Rendering scenes/checkers.json -> output_checkers.bmp"
$PYTHON_BIN src/btrace.py --scene scenes/checkers.json --size $SIZE --outfile output_checkers.bmp

echo "========================================="
echo " All scenes rendered successfully!"
echo "========================================="
