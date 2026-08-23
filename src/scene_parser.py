import json
import numpy

from rvmath.geometry import sphere, plane
from rvlight.light import Light
from rvlight.illuminationmodel import PhongIlluminationModel

def load_scene(filepath, world, raytracer):
    with open(filepath, 'r') as f:
        data = json.load(f)

    # Set up viewport
    cam = data.get("camera", {})
    if cam:
        vpwidth = cam.get("vpwidth", 5.76)
        # Dynamically calculate viewport height to match the image aspect ratio
        # This prevents squishing when running at different resolutions (like 16:9 1080p)
        aspect_ratio = raytracer.width / float(raytracer.height)
        vpheight = vpwidth / aspect_ratio
        
        raytracer.setViewport(cam_origin=cam.get("origin", [0.0, 0.0, 20.0]),
                              distance=cam.get("distance", 10.0),
                              vpwidth=vpwidth,
                              vpheight=vpheight)

    # Set up materials
    materials = {}
    for mat_name, mat_props in data.get("materials", {}).items():
        ill_model = PhongIlluminationModel(
            kAmbient=mat_props.get("kAmbient", 0.0),
            kDiffuse=numpy.array(mat_props.get("kDiffuse", [1.0, 1.0, 1.0])),
            kSpecular=mat_props.get("kSpecular", 0.0),
            nS=mat_props.get("nS", 10.0)
        )
        materials[mat_name] = ill_model

    # Set up lights
    for light_props in data.get("lights", []):
        world.addLight(Light(orig=numpy.array(light_props["origin"]),
                             color=numpy.array(light_props["color"])))

    # Set up objects
    for obj_props in data.get("objects", []):
        obj_type = obj_props.get("type", "").lower()
        
        is_reflector = obj_props.get("is_reflector", False)
        is_refractor = obj_props.get("is_refractor", False)
        
        if obj_type == "sphere":
            mat = materials.get(obj_props.get("material"))
            new_obj = sphere.Sphere(
                center=numpy.array(obj_props["center"]),
                radius=obj_props["radius"],
                ill_model=mat,
                world_intfn=world.doesIntersect,
                is_reflector=is_reflector,
                is_refractor=is_refractor
            )
            world.addObject(new_obj)
            
        elif obj_type == "plane":
            mat = materials.get(obj_props.get("material"))
            new_obj = plane.Plane(
                pnormal=numpy.array(obj_props["normal"]),
                distance=obj_props["distance"],
                ill_model=mat,
                world_intfn=world.doesIntersect,
                is_reflector=is_reflector,
                is_refractor=is_refractor
            )
            world.addObject(new_obj)
            
        elif obj_type == "checkered_plane":
            mat1 = materials.get(obj_props.get("material1"))
            mat2 = materials.get(obj_props.get("material2"))
            new_obj = plane.CheckeredPlane(
                pnormal=numpy.array(obj_props["normal"]),
                distance=obj_props["distance"],
                ill_model1=mat1,
                ill_model2=mat2,
                world_intfn=world.doesIntersect
            )
            world.addObject(new_obj)

