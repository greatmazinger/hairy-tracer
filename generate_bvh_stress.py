import json
import random
import math

objs = []
# Create a sphere made of thousands of triangles
# UV sphere logic
segments = 60
rings = 60
radius = 10.0
center = [0.0, 0.0, -20.0]

vertices = []
for i in range(rings + 1):
    v = i / rings
    phi = v * math.pi
    for j in range(segments):
        u = j / segments
        theta = u * 2.0 * math.pi
        
        x = center[0] + radius * math.sin(phi) * math.cos(theta)
        y = center[1] + radius * math.cos(phi)
        z = center[2] + radius * math.sin(phi) * math.sin(theta)
        vertices.append([x, y, z])

triangles = []
obj_content = ""
for v in vertices:
    obj_content += f"v {v[0]} {v[1]} {v[2]}\n"

for i in range(rings):
    for j in range(segments):
        j_next = (j + 1) % segments
        
        v0 = i * segments + j + 1
        v1 = i * segments + j_next + 1
        v2 = (i + 1) * segments + j_next + 1
        v3 = (i + 1) * segments + j + 1
        
        obj_content += f"f {v0} {v1} {v2}\n"
        obj_content += f"f {v0} {v2} {v3}\n"

with open("models/bvh_stress.obj", "w") as f:
    f.write(obj_content)

scene = {
    "camera": {
        "origin": [0.0, 0.0, 10.0],
        "look_at": [0.0, 0.0, 0.0],
        "up": [0.0, 1.0, 0.0],
        "distance": 1.0,
        "vpwidth": 1.0,
        "vpheight": 1.0
    },
    "materials": {
        "stress_mat": {
            "kAmbient": 0.2,
            "kDiffuse": [0.9, 0.2, 0.2],
            "kSpecular": 0.5,
            "nS": 30.0
        }
    },
    "objects": [
        {
            "type": "mesh",
            "file": "models/bvh_stress.obj",
            "material": "stress_mat"
        }
    ],
    "lights": [
        {
            "origin": [10.0, 10.0, 10.0],
            "color": [255.0, 255.0, 255.0]
        }
    ]
}

with open("scenes/bvh_stress.json", "w") as f:
    json.dump(scene, f)

