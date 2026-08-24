use glam::DVec3;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::material::{Material, MaterialId, Light};
use crate::render::{render_image_serial, render_image_parallel};

pub fn build_spheres3_scene() -> Scene {
    let mut scene = Scene::new();
    
    // mat1
    scene.materials.push(Material {
        k_ambient: 0.0,
        k_diffuse: DVec3::new(0.4, 0.0, 0.0),
        k_specular: 0.5,
        ns: 15.0,
        is_reflector: false,
        is_refractor: false,
    });
    // mat3
    scene.materials.push(Material {
        k_ambient: 0.0,
        k_diffuse: DVec3::new(0.1, 0.7, 0.0),
        k_specular: 0.7,
        ns: 10.0,
        is_reflector: false,
        is_refractor: false,
    });
    // mat4
    scene.materials.push(Material {
        k_ambient: 0.0,
        k_diffuse: DVec3::new(0.3, 0.3, 0.9),
        k_specular: 1.0,
        ns: 100.0,
        is_reflector: false,
        is_refractor: true,
    });
    
    scene.lights.push(Light { origin: DVec3::new(5.0, 5.0, 5.0), color: DVec3::new(100.0, 100.0, 100.0) });
    scene.lights.push(Light { origin: DVec3::new(0.0, 40.0, 0.0), color: DVec3::new(255.0, 10.0, 15.0) });
    
    scene.objects.push(Box::new(Sphere::new(DVec3::new(0.0, -13.0, -8.0), 12.0, MaterialId(0))));
    scene.objects.push(Box::new(Sphere::new(DVec3::new(1.0, 0.0, -3.0), 1.0, MaterialId(1))));
    scene.objects.push(Box::new(Sphere::new(DVec3::new(-1.0, 1.0, -5.0), 2.0, MaterialId(2))));
    
    scene
}
