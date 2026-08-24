use glam::DVec3;
use hairy_tracer_core::material::{Light, Material, MaterialId, TextureRef};
use hairy_tracer_core::render::{render_image_parallel, render_image_serial};
use hairy_tracer_core::scene::Scene;
use hairy_tracer_core::sphere::Sphere;
use std::fs::File;
use std::io::Write;

fn build_spheres3_scene() -> Scene {
    let mut scene = Scene::new();

    scene.materials.push(Material {
        k_ambient: 0.0,
        k_diffuse: DVec3::new(0.4, 0.0, 0.0),
        k_specular: 0.5,
        ns: 15.0,
        is_reflector: false,
        is_refractor: false,
        ambient_color: DVec3::new(15.0, 75.0, 255.0),
        has_explicit_ambient: false,
        ior: 1.5,
        use_fresnel: false,
        absorption: DVec3::ZERO,
        texture: TextureRef::None,
    });
    scene.materials.push(Material {
        k_ambient: 0.0,
        k_diffuse: DVec3::new(0.1, 0.7, 0.0),
        k_specular: 0.7,
        ns: 10.0,
        is_reflector: false,
        is_refractor: false,
        ambient_color: DVec3::new(15.0, 75.0, 255.0),
        has_explicit_ambient: false,
        ior: 1.5,
        use_fresnel: false,
        absorption: DVec3::ZERO,
        texture: TextureRef::None,
    });
    scene.materials.push(Material {
        k_ambient: 0.0,
        k_diffuse: DVec3::new(0.3, 0.3, 0.9),
        k_specular: 1.0,
        ns: 100.0,
        is_reflector: false,
        is_refractor: true,
        ambient_color: DVec3::new(15.0, 75.0, 255.0),
        has_explicit_ambient: false,
        ior: 1.5,
        use_fresnel: false,
        absorption: DVec3::ZERO,
        texture: TextureRef::None,
    });

    scene.lights.push(Light {
        origin: DVec3::new(5.0, 5.0, 5.0),
        color: DVec3::new(100.0, 100.0, 100.0),
        radius: 0.0,
    });
    scene.lights.push(Light {
        origin: DVec3::new(0.0, 40.0, 0.0),
        color: DVec3::new(255.0, 10.0, 15.0),
        radius: 0.0,
    });

    scene.objects.push(Box::new(Sphere::new(
        DVec3::new(0.0, -13.0, -8.0),
        12.0,
        MaterialId(0),
    )));
    scene.objects.push(Box::new(Sphere::new(
        DVec3::new(1.0, 0.0, -3.0),
        1.0,
        MaterialId(1),
    )));
    scene.objects.push(Box::new(Sphere::new(
        DVec3::new(-1.0, 1.0, -5.0),
        2.0,
        MaterialId(2),
    )));

    scene
}

#[test]
fn test_output_json() {
    let scene = build_spheres3_scene();
    let cam_origin = DVec3::new(0.0, 0.0, 20.0);
    let look_at = DVec3::new(0.0, 0.0, 10.0);
    let up = DVec3::new(0.0, 1.0, 0.0);

    let vpwidth = 5.76;
    let vpheight = 5.76;

    let pixels = render_image_serial(
        &scene, cam_origin, look_at, up, vpwidth, vpheight, 10, 10, 2, 1, 0.0, 10.0
    );
    let pixels_parallel = render_image_parallel(
        &scene, cam_origin, look_at, up, vpwidth, vpheight, 10, 10, 2, 1, 0.0, 10.0
    );

    assert_eq!(
        pixels, pixels_parallel,
        "Serial and Parallel renders differ!"
    );

    let mut out = String::new();
    out.push_str("[\n");
    for i in 0..(10 * 10) {
        out.push_str(&format!(
            "  [{}, {}, {}]",
            pixels[i * 3],
            pixels[i * 3 + 1],
            pixels[i * 3 + 2]
        ));
        if i < (10 * 10) - 1 {
            out.push_str(",\n");
        } else {
            out.push_str("\n");
        }
    }
    out.push_str("]\n");

    let mut file = File::create("../rust_out.json").unwrap();
    file.write_all(out.as_bytes()).unwrap();
}
