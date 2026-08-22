import sys
import os
import json
import pytest

from btrace import BTracer
from scenes import readScene_3spheres

SNAPSHOT_FILE = os.path.join(os.path.dirname(__file__), 'snapshots', 'spheres3_10x10.json')

def test_render_spheres3_e2e():
    tracer = BTracer(size=(10, 10), testflag=False)
    tracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                        distance = 10.0,
                        vpwidth = 6.40 * 0.9,
                        vpheight = 4.80 * 0.9 )
    readScene_3spheres(world=tracer.world)
    data = tracer.get_data()
    
    # If snapshot doesn't exist, generate it
    if not os.path.exists(SNAPSHOT_FILE):
        os.makedirs(os.path.dirname(SNAPSHOT_FILE), exist_ok=True)
        with open(SNAPSHOT_FILE, 'w') as f:
            json.dump(data, f, indent=2)
        pytest.skip("Generated new snapshot. Run again to verify.")
        
    # Compare with snapshot
    with open(SNAPSHOT_FILE, 'r') as f:
        snapshot_data = json.load(f)
        
    # Convert tuples to lists for JSON comparison
    data_as_lists = [list(p) for p in data]
    assert data_as_lists == snapshot_data
