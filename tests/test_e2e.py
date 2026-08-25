"""
E2E Test Suite for the Rust ray tracing engine.

NOTE ON PYTHON PARITY:
Historically, this suite tested for pixel-for-pixel parity between the Rust engine
and the pure-Python legacy engine. That is no longer the correctness bar.
Rust now has capabilities Python never will (Fresnel, textures, environment maps, etc.),
and uses a default maxdepth of 5 (vs Python's 2) to correctly render environment maps
through refractions.
Therefore, these tests now compare the current Rust output against a frozen baseline 
snapshot of the Rust engine's output. Python is retained as an unmaintained historical
reference only.
"""

import os
import glob
import json
import pytest

import hairy_tracer_core

SCENE_DIRS = [
    os.path.join(os.path.dirname(__file__), '..', 'scenes', 'whitted', 'legacy'),
    os.path.join(os.path.dirname(__file__), '..', 'example', 'materials')
]
SNAPSHOT_DIR = os.path.join(os.path.dirname(__file__), 'snapshots')

WIDTH = 50
HEIGHT = 50
MAXDEPTH = 5

def get_scene_files():
    scene_files = []
    for d in SCENE_DIRS:
        for f in glob.glob(os.path.join(d, '**', '*.json'), recursive=True):
            if "cornell" not in f:
                scene_files.append(f)
    return scene_files

@pytest.mark.parametrize("scene_file", get_scene_files())
def test_render_matches_baseline(scene_file):
    basename = os.path.basename(scene_file)
    name = os.path.splitext(basename)[0]
    snapshot_file = os.path.join(SNAPSHOT_DIR, f"{name}_{WIDTH}x{HEIGHT}.json")
    
    # Ensure snapshot exists
    assert os.path.exists(snapshot_file), f"Baseline snapshot missing for {name}. Please generate it."
    
    with open(scene_file, 'r') as f:
        scene_json = f.read()

    # Call Rust engine
    pixels_bytes = hairy_tracer_core.render_image(scene_json, WIDTH, HEIGHT, MAXDEPTH)
    
    data_as_lists = []
    for i in range(0, len(pixels_bytes), 3):
        data_as_lists.append([pixels_bytes[i], pixels_bytes[i+1], pixels_bytes[i+2]])
        
    with open(snapshot_file, 'r') as f:
        snapshot_data = json.load(f)
        
    assert data_as_lists == snapshot_data, f"Render output for {name} diverged from baseline snapshot."
