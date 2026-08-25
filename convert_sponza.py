import os
import json
import math

sponza_dir = "models/sponza"
mtl_path = os.path.join(sponza_dir, "sponza.mtl")
obj_path = os.path.join(sponza_dir, "sponza.obj")

# 1. Parse MTL
materials = {}
current_mat = None
with open(mtl_path, "r") as f:
    for line in f:
        line = line.strip()
        if not line or line.startswith("#"): continue
        parts = line.split()
        cmd = parts[0].lower()
        if cmd == "newmtl":
            current_mat = parts[1]
            materials[current_mat] = {
                "kAmbient": 0.0,
                "kDiffuse": [1.0, 1.0, 1.0],
                "kSpecular": 0.0,
                "nS": 0.0,
                "roughness": 0.8,
                "metallic": 0.0,
            }
        elif current_mat:
            if cmd == "kd":
                materials[current_mat]["kDiffuse"] = [float(parts[1]), float(parts[2]), float(parts[3])]
            elif cmd == "ks":
                # Average specular
                ks = (float(parts[1]) + float(parts[2]) + float(parts[3])) / 3.0
                materials[current_mat]["kSpecular"] = ks
            elif cmd == "ns":
                ns = float(parts[1])
                materials[current_mat]["nS"] = ns
                # Very rough heuristic
                roughness = math.sqrt(2.0 / (max(ns, 1.0) + 2.0))
                materials[current_mat]["roughness"] = roughness
            elif cmd == "map_kd":
                tex_path = line.split(maxsplit=1)[1].strip()
                # fix windows paths
                tex_path = tex_path.replace("\\", "/")
                materials[current_mat]["texture"] = {
                    "type": "image",
                    "path": f"models/sponza/{tex_path}"
                }

# Add default materials just in case
materials["default"] = {
    "kAmbient": 0.0,
    "kDiffuse": [0.8, 0.8, 0.8],
    "kSpecular": 0.0,
    "nS": 0.0,
    "roughness": 1.0,
    "metallic": 0.0
}

# 2. Split OBJ
print("Splitting OBJ...")
vertices = []
texcoords = []
normals = []

# Map of mat_name -> list of face lines
faces_by_mat = {}
current_mat = "default"

with open(obj_path, "r") as f:
    for line in f:
        if line.startswith("v "):
            vertices.append(line)
        elif line.startswith("vt "):
            texcoords.append(line)
        elif line.startswith("vn "):
            normals.append(line)
        elif line.startswith("usemtl "):
            current_mat = line.split()[1].strip()
            if current_mat not in faces_by_mat:
                faces_by_mat[current_mat] = []
        elif line.startswith("f "):
            if current_mat not in faces_by_mat:
                faces_by_mat[current_mat] = []
            faces_by_mat[current_mat].append(line)

objects = []
# Write sub-objs
for mat_name, face_lines in faces_by_mat.items():
    if not face_lines: continue
    # Safe filename
    safe_mat_name = "".join([c if c.isalnum() else "_" for c in mat_name])
    sub_obj_name = f"sponza_{safe_mat_name}.obj"
    sub_obj_path = os.path.join(sponza_dir, sub_obj_name)
    
    # We could just write the original v/vt/vn and then the faces.
    # It's inefficient on disk but fine for this one-time script!
    with open(sub_obj_path, "w") as f:
        f.writelines(vertices)
        f.writelines(texcoords)
        f.writelines(normals)
        f.writelines(face_lines)
    
    objects.append({
        "type": "mesh",
        "file": f"models/sponza/{sub_obj_name}",
        "material": mat_name,
        "smooth_shading": True
    })

# 3. Create Scene JSON
scene = {
    "camera": {
        "origin": [0.0, 200.0, 0.0],
        "look_at": [1000.0, 200.0, 0.0],
        "up": [0.0, 1.0, 0.0],
        "distance": 1.0,
        "vpwidth": 1000.0,
        "vpheight": 1000.0,
        "samples_per_pixel": 25,
        "aperture": 0.0,
        "focal_distance": 1.0
    },
    "integrator": "pathtracer",
    "materials": materials,
    "lights": [
        {
            "origin": [0.0, 1500.0, 0.0],
            "color": [150.0, 140.0, 130.0],
            "radius": 150.0
        },
        {
            "origin": [0.0, 200.0, 0.0],
            "color": [30.0, 30.0, 35.0],
            "radius": 50.0
        },
        {
            "origin": [-800.0, 200.0, 0.0],
            "color": [25.0, 25.0, 30.0],
            "radius": 50.0
        },
        {
            "origin": [800.0, 200.0, 0.0],
            "color": [25.0, 25.0, 30.0],
            "radius": 50.0
        }
    ],
    "objects": objects
}

with open("scenes/path_trace/pbr/sponza.json", "w") as f:
    json.dump(scene, f, indent=2)

print("Done! Scene written to scenes/path_trace/pbr/sponza.json")
