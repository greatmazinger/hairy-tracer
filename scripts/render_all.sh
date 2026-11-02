#!/bin/bash

# Exit on any error
set -e

# Default size if not provided as an argument
SIZE=${1:-"640x480"}
PYTHON_BIN="python3"

echo "========================================="
echo " rendering scenes at size: $SIZE"
echo "========================================="

# --- Classic scenes (scenes/whitted/legacy/) ---

echo "Rendering scenes/whitted/legacy/spheres3.json -> output_spheres3.bmp"
$PYTHON_BIN src/btrace.py --scene scenes/whitted/legacy/spheres3.json --size $SIZE --outfile output_spheres3.bmp

echo "Rendering scenes/whitted/legacy/checkers.json -> output_checkers.bmp"
$PYTHON_BIN src/btrace.py --scene scenes/whitted/legacy/checkers.json --size $SIZE --outfile output_checkers.bmp

echo "Rendering scenes/whitted/legacy/camera_test.json -> output_camera.bmp"
$PYTHON_BIN src/btrace.py --scene scenes/whitted/legacy/camera_test.json --size $SIZE --outfile output_camera.bmp

echo "Rendering scenes/whitted/legacy/triangle.json -> output_triangle.bmp"
$PYTHON_BIN src/btrace.py --scene scenes/whitted/legacy/triangle.json --size $SIZE --outfile output_triangle.bmp

echo "Rendering scenes/whitted/legacy/mesh_test.json -> output_mesh.bmp"
$PYTHON_BIN src/btrace.py --scene scenes/whitted/legacy/mesh_test.json --size $SIZE --outfile output_mesh.bmp

echo "Rendering scenes/whitted/legacy/torus_test.json -> output_torus.bmp"
$PYTHON_BIN src/btrace.py --scene scenes/whitted/legacy/torus_test.json --size $SIZE --outfile output_torus.bmp

# --- Material quality examples (example/materials/) ---

echo ""
echo "Rendering material examples..."

echo "Rendering example/materials/fresnel_off.json -> example/materials/fresnel_off.png"
$PYTHON_BIN src/btrace.py --scene example/materials/fresnel_off.json --size $SIZE --outfile example/materials/fresnel_off.png

echo "Rendering example/materials/fresnel_on.json -> example/materials/fresnel_on.png"
$PYTHON_BIN src/btrace.py --scene example/materials/fresnel_on.json --size $SIZE --outfile example/materials/fresnel_on.png

echo "Rendering example/materials/absorption.json -> example/materials/absorption.png"
$PYTHON_BIN src/btrace.py --scene example/materials/absorption.json --size $SIZE --outfile example/materials/absorption.png

echo "Rendering example/materials/procedural.json -> example/materials/procedural.png"
$PYTHON_BIN src/btrace.py --scene example/materials/procedural.json --size $SIZE --outfile example/materials/procedural.png

echo "Rendering example/materials/image_texture.json -> example/materials/image_texture.png"
$PYTHON_BIN src/btrace.py --scene example/materials/image_texture.json --size $SIZE --outfile example/materials/image_texture.png

echo "Rendering example/materials/envmap.json -> example/materials/envmap.png"
$PYTHON_BIN src/btrace.py --scene example/materials/envmap.json --size $SIZE --outfile example/materials/envmap.png

echo ""
echo "========================================="
echo " All scenes rendered successfully!"
echo "========================================="
