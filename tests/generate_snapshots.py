import os
import glob
import json
import hairy_tracer_core

SCENE_DIRS = [
    os.path.join(os.path.dirname(__file__), '..', 'scenes'),
    os.path.join(os.path.dirname(__file__), '..', 'example', 'materials')
]
SNAPSHOT_DIR = os.path.join(os.path.dirname(__file__), 'snapshots')

WIDTH = 50
HEIGHT = 50
MAXDEPTH = 5

def generate():
    os.makedirs(SNAPSHOT_DIR, exist_ok=True)
    
    scene_files = []
    for d in SCENE_DIRS:
        scene_files.extend(glob.glob(os.path.join(d, '*.json')))
        
    for scene_file in scene_files:
        basename = os.path.basename(scene_file)
        name = os.path.splitext(basename)[0]
        
        with open(scene_file, 'r') as f:
            scene_json = f.read()
            
        print(f"Generating snapshot for {name} ({WIDTH}x{HEIGHT})...")
        pixels_bytes = hairy_tracer_core.render_image(scene_json, WIDTH, HEIGHT, MAXDEPTH)
        
        data_as_lists = []
        for i in range(0, len(pixels_bytes), 3):
            data_as_lists.append([pixels_bytes[i], pixels_bytes[i+1], pixels_bytes[i+2]])
            
        snapshot_file = os.path.join(SNAPSHOT_DIR, f"{name}_{WIDTH}x{HEIGHT}.json")
        with open(snapshot_file, 'w') as f:
            json.dump(data_as_lists, f)
            
    print("Done generating snapshots.")

if __name__ == '__main__':
    generate()
