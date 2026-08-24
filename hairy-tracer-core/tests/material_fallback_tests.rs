use glam::DVec3;
use hairy_tracer_core::intersect::Intersectable;
use hairy_tracer_core::ray::Ray;
use hairy_tracer_core::scene_parser::parse_scene_json;

#[test]
fn test_ambient_fallback_and_override() {
    let json = r#"{
        "camera": { "origin": [0,0,0], "distance": 1, "vpwidth": 1, "vpheight": 1 },
        "materials": {
            "legacy_mat": { "kAmbient": 1.0, "kDiffuse": [1,1,1], "kSpecular": 1.0, "nS": 10.0 },
            "modern_mat": { "kAmbient": 1.0, "kDiffuse": [1,1,1], "kSpecular": 1.0, "nS": 10.0, "ambientColor": [99.0, 99.0, 99.0] }
        },
        "objects": [
            { "type": "sphere", "center": [0, 0, -5], "radius": 1, "material": "legacy_mat" },
            { "type": "plane", "normal": [0, 1, 0], "distance": -2, "material": "legacy_mat" },
            { "type": "sphere", "center": [5, 0, -5], "radius": 1, "material": "modern_mat" },
            { "type": "checkered_plane", "normal": [0, 1, 0], "distance": -4, "material1": "legacy_mat", "material2": "legacy_mat" }
        ],
        "lights": []
    }"#;

    let (scene, _) = parse_scene_json(json).expect("Failed to parse JSON");

    // 1. Sphere with legacy material (fallback should be [15, 75, 255])
    let hit_sphere = scene.objects[0]
        .intersect(&Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, -1.0)), 0)
        .expect("Missed sphere");
    let mat_sphere = &scene.materials[hit_sphere.material_id.0];
    assert_eq!(
        mat_sphere.ambient_color,
        DVec3::new(15.0, 75.0, 255.0),
        "Sphere legacy fallback failed"
    );
    assert_eq!(mat_sphere.has_explicit_ambient, false);

    // 2. Plane with legacy material (fallback should be [0, 0, 0] because it doesn't get overridden)
    let hit_plane = scene.objects[1]
        .intersect(&Ray::new(DVec3::ZERO, DVec3::new(0.0, 1.0, 0.0)), 1)
        .expect("Missed plane");
    let mat_plane = &scene.materials[hit_plane.material_id.0];
    assert_eq!(
        mat_plane.ambient_color,
        DVec3::new(0.0, 0.0, 0.0),
        "Plane legacy fallback failed"
    );
    assert_eq!(mat_plane.has_explicit_ambient, false);

    // 3. Sphere with modern material (override should be respected, [99, 99, 99])
    let hit_modern = scene.objects[2]
        .intersect(
            &Ray::new(DVec3::new(5.0, 0.0, 0.0), DVec3::new(0.0, 0.0, -1.0)),
            2,
        )
        .expect("Missed modern sphere");
    let mat_modern = &scene.materials[hit_modern.material_id.0];
    assert_eq!(
        mat_modern.ambient_color,
        DVec3::new(99.0, 99.0, 99.0),
        "Explicit ambient color override failed"
    );
    assert_eq!(mat_modern.has_explicit_ambient, true);

    // 4. Checkered plane with legacy materials (material 1 falls back to [10, 10, 250], material 2 to [150, 10, 10])
    // Hit at origin (x=0, z=0 => both even => XOR false => material 2)
    let hit_checker2 = scene.objects[3]
        .intersect(&Ray::new(DVec3::ZERO, DVec3::new(0.0, 1.0, 0.0)), 3)
        .expect("Missed checker");
    let mat_checker2 = &scene.materials[hit_checker2.material_id.0];
    assert_eq!(
        mat_checker2.ambient_color,
        DVec3::new(150.0, 10.0, 10.0),
        "CheckeredPlane mat 2 fallback failed"
    );

    // Hit at x=1, z=0 => odd/even => XOR true => material 1
    let hit_checker1 = scene.objects[3]
        .intersect(
            &Ray::new(DVec3::new(1.0, 0.0, 0.0), DVec3::new(0.0, 1.0, 0.0)),
            3,
        )
        .expect("Missed checker");
    let mat_checker1 = &scene.materials[hit_checker1.material_id.0];
    assert_eq!(
        mat_checker1.ambient_color,
        DVec3::new(10.0, 10.0, 250.0),
        "CheckeredPlane mat 1 fallback failed"
    );
}
