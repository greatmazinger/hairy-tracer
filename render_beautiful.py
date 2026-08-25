import json

with open("scenes/path_trace/pbr/sponza.json", "r") as f:
    scene = json.load(f)

# High-quality settings
scene["camera"]["samples_per_pixel"] = 100
scene["integrator"] = "pathtracer"

with open("scenes/path_trace/pbr/sponza.json", "w") as f:
    json.dump(scene, f, indent=2)
