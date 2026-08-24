import os
import glob
import time
import hairy_tracer_core

SCENE_DIRS = [
    os.path.join(os.path.dirname(__file__), '..', 'scenes'),
    os.path.join(os.path.dirname(__file__), '..', 'example', 'materials')
]

def benchmark():
    scene_files = []
    for d in SCENE_DIRS:
        scene_files.extend(glob.glob(os.path.join(d, '*.json')))
        
    width = 400
    height = 300
    
    time_2 = 0.0
    time_5 = 0.0
    
    print(f"{'Scene':<20} | {'Depth 2 (s)':<15} | {'Depth 5 (s)':<15}")
    print("-" * 55)
    
    for scene_file in scene_files:
        basename = os.path.basename(scene_file)
        
        with open(scene_file, 'r') as f:
            scene_json = f.read()
            
        # Warmup
        hairy_tracer_core.render_image(scene_json, 10, 10, 2)
        
        # Depth 2
        t0 = time.time()
        hairy_tracer_core.render_image(scene_json, width, height, 2)
        t1 = time.time()
        dur_2 = t1 - t0
        time_2 += dur_2
        
        # Depth 5
        t0 = time.time()
        hairy_tracer_core.render_image(scene_json, width, height, 5)
        t1 = time.time()
        dur_5 = t1 - t0
        time_5 += dur_5
        
        print(f"{basename:<20} | {dur_2:<15.4f} | {dur_5:<15.4f}")
        
    print("-" * 55)
    print(f"{'TOTAL':<20} | {time_2:<15.4f} | {time_5:<15.4f}")
    
    inc = (time_5 - time_2) / time_2 * 100 if time_2 > 0 else 0
    print(f"\nCost of Depth 5 vs 2: {inc:.2f}% increase in render time")

if __name__ == '__main__':
    benchmark()
