#!/bin/bash
set -e

echo "Building Rust core..."
maturin develop --release

SCENES=(
    "scenes/triangle_pt.json"
    "scenes/camera_test_pt.json"
    "scenes/torus_test_pt.json"
    "scenes/checkers_pt.json"
    "scenes/spheres3_pt.json"
    "scenes/mesh_test_pt.json"
    "scenes/stress/mirrors_pt.json"
    "scenes/stress/bvh_stress_pt.json"
    "scenes/cornell_pbr.json"
)

mkdir -p output_pt

for SCENE in "${SCENES[@]}"; do
    echo "======================================"
    echo "Rendering $SCENE..."
    echo "======================================"
    
    # Extract filename without extension for the output image
    BASENAME=$(basename "$SCENE" .json)
    
    time python src/btrace.py --scene "$SCENE" --size 400x400 --depth 5
    
    # Move the output so it doesn't get overwritten
    mv output.bmp "output_pt/${BASENAME}.bmp"
    echo "Saved to output_pt/${BASENAME}.bmp"
    echo ""
done

echo "All path traced renders completed!"
