use glam::DVec3;
use hairy_tracer_core::scene_parser::parse_scene_json;
use hairy_tracer_core::ray::Ray;
use hairy_tracer_core::render::trace_ray_worker;

#[test]
fn trace_single() {
    let json = r#"{
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
    }"#;
    let (scene, _) = parse_scene_json(json).unwrap();
    let ray = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.33, 0.0, -5.0));
    let color = trace_ray_worker(&ray, 1, &scene, 2, None);
    println!("COLOR: {:?}", color);
    
    let hit = scene.objects[0].intersect(&ray, 0);
    println!("HIT: {:?}", hit);
    
    println!("MATERIAL: {:?}", scene.materials[0]);
}
