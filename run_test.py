import sys
sys.path.insert(0, '.')
import hairy_tracer_core

json_str = """{
    "camera": { "origin": [0,0,5], "distance": 5, "vpwidth": 1, "vpheight": 1 },
    "materials": {
        "white": { "kAmbient": 1.0, "kDiffuse": [0,0,0], "kSpecular": 0.0, "nS": 0.0, "ambientColor": [255,255,255] }
    },
    "objects": [
        { "type": "sphere", "center": [0.5, 0, 0], "radius": 0.5, "material": "white" }
    ],
    "lights": [
        { "origin": [0, 10, 0], "color": [0, 0, 0], "radius": 0.0 }
    ]
}"""

res = hairy_tracer_core.render_image(json_str, 3, 1, 1)
print(list(res))
