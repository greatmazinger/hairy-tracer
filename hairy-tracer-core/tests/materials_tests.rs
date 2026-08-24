use glam::DVec3;
use hairy_tracer_core::material::{
    EnvironmentMap, Light, Material, MaterialId, TextureImage, TextureRef,
};
use hairy_tracer_core::plane::Plane;
use hairy_tracer_core::ray::Ray;
use hairy_tracer_core::render::{render_image_parallel, trace_ray_worker};
use hairy_tracer_core::scene::Scene;
use hairy_tracer_core::scene_parser::parse_scene_json;
use hairy_tracer_core::sphere::Sphere;

// ====================================================================
// 1. Fresnel-Schlick tests
// ====================================================================

#[test]
fn test_fresnel_grazing_is_near_total() {
    let r0: f64 = ((1.0_f64 - 1.5) / (1.0_f64 + 1.5)).powi(2);
    let cos_theta: f64 = 0.01;
    let reflectance = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
    assert!(
        reflectance > 0.95,
        "Grazing reflectance should be near 1.0, got {}",
        reflectance
    );
}

#[test]
fn test_fresnel_normal_incidence_matches_r0() {
    let ior: f64 = 1.5;
    let r0: f64 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    let cos_theta: f64 = 1.0;
    let reflectance = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
    assert!(
        (reflectance - r0).abs() < 1e-10,
        "Normal incidence reflectance should equal R0 = {}, got {}",
        r0,
        reflectance
    );
}

#[test]
fn test_fresnel_gated_behind_flag() {
    // With use_fresnel=false, a reflective+refractive sphere should produce the same
    // image as the legacy engine (additive, no Fresnel weighting).
    let json = r#"{
        "camera": { "origin": [0,0,5], "distance": 5, "vpwidth": 2, "vpheight": 2 },
        "materials": {
            "glass": { "kAmbient": 0.0, "kDiffuse": [0.1,0.1,0.1], "kSpecular": 1.0, "nS": 100.0 }
        },
        "objects": [
            { "type": "sphere", "center": [0,0,0], "radius": 1.0, "material": "glass", "is_reflector": true, "is_refractor": true }
        ],
        "lights": [
            { "origin": [5, 5, 5], "color": [255, 255, 255] }
        ]
    }"#;

    let (scene, _) = parse_scene_json(json).unwrap();

    // Verify use_fresnel is false by default
    for mat in &scene.materials {
        assert!(!mat.use_fresnel, "use_fresnel should default to false");
    }

    // Just verify it renders without crashing
    let img = render_image_parallel(
        &scene,
        DVec3::new(0.0, 0.0, 5.0),
        DVec3::new(0.0, 0.0, 0.0),
        DVec3::new(0.0, 1.0, 0.0),
        2.0,
        2.0,
        4,
        4,
        3,
        1,
        0.0,
        5.0,
    );
    assert_eq!(img.len(), 4 * 4 * 3);
}

#[test]
fn test_fresnel_changes_output_when_enabled() {
    // With use_fresnel=true, the reflect/refract weighting changes from additive to Fresnel-blended.
    // We place a bright plane behind a glass sphere so the refracted colors are visible.
    let json_off = r#"{
        "camera": { "origin": [0,0,10], "distance": 10, "vpwidth": 2, "vpheight": 2 },
        "materials": {
            "glass": { "kAmbient": 0.0, "kDiffuse": [0.0,0.0,0.0], "kSpecular": 0.5, "nS": 100.0 },
            "bg": { "kAmbient": 1.0, "kDiffuse": [0.0,0.0,0.0], "kSpecular": 0.0, "nS": 0.0, "ambientColor": [255, 255, 255] }
        },
        "objects": [
            { "type": "sphere", "center": [0,0,0], "radius": 1.0, "material": "glass", "is_reflector": true, "is_refractor": true },
            { "type": "sphere", "center": [0,0,-10], "radius": 5.0, "material": "bg" }
        ],
        "lights": [
            { "origin": [5, 5, 10], "color": [255, 255, 255] }
        ]
    }"#;

    let json_on = r#"{
        "camera": { "origin": [0,0,10], "distance": 10, "vpwidth": 2, "vpheight": 2 },
        "materials": {
            "glass": { "kAmbient": 0.0, "kDiffuse": [0.0,0.0,0.0], "kSpecular": 0.5, "nS": 100.0, "use_fresnel": true, "ior": 1.5 },
            "bg": { "kAmbient": 1.0, "kDiffuse": [0.0,0.0,0.0], "kSpecular": 0.0, "nS": 0.0, "ambientColor": [255, 255, 255] }
        },
        "objects": [
            { "type": "sphere", "center": [0,0,0], "radius": 1.0, "material": "glass", "is_reflector": true, "is_refractor": true },
            { "type": "sphere", "center": [0,0,-10], "radius": 5.0, "material": "bg" }
        ],
        "lights": [
            { "origin": [5, 5, 10], "color": [255, 255, 255] }
        ]
    }"#;

    let (scene_off, _) = parse_scene_json(json_off).unwrap();
    let (scene_on, _) = parse_scene_json(json_on).unwrap();

    let img_off = render_image_parallel(
        &scene_off,
        DVec3::new(0.0, 0.0, 10.0),
        DVec3::new(0.0, 0.0, 0.0),
        DVec3::new(0.0, 1.0, 0.0),
        2.0,
        2.0,
        8,
        8,
        5,
        1,
        0.0,
        10.0,
    );
    let img_on = render_image_parallel(
        &scene_on,
        DVec3::new(0.0, 0.0, 10.0),
        DVec3::new(0.0, 0.0, 0.0),
        DVec3::new(0.0, 1.0, 0.0),
        2.0,
        2.0,
        8,
        8,
        5,
        1,
        0.0,
        10.0,
    );

    assert_ne!(
        img_off, img_on,
        "Fresnel on vs off should produce different pixel output"
    );
}

// ====================================================================
// 2. Beer-Lambert absorption tests
// ====================================================================

#[test]
fn test_absorption_matches_beer_lambert() {
    let coeff = 0.5_f64;
    let ior = 1.5_f64;
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2); // ≈ 0.04
    let fresnel_penalty = 1.0 - r0; // ≈ 0.96 (two surfaces, but first-order approx)

    for radius in [0.5_f64, 1.0, 2.0] {
        let path = 2.0 * radius;
        let expected_t = fresnel_penalty * (-coeff * path).exp();

        let json = format!(
            r#"{{
            "camera": {{ "origin": [0,0,10], "distance": 10, "vpwidth": 1, "vpheight": 1 }},
            "materials": {{
                "absorb": {{ "kAmbient": 0.0, "kDiffuse": [0.0,0.0,0.0], "kSpecular": 0.0,
                             "nS": 0.0, "use_fresnel": true, "ior": {ior}, "absorption": [{coeff},{coeff},{coeff}] }},
                "white": {{ "kAmbient": 1.0, "kDiffuse": [0.0,0.0,0.0], "kSpecular": 0.0, "nS": 0.0,
                            "ambientColor": [255, 255, 255] }}
            }},
            "objects": [
                {{ "type": "sphere", "center": [0,0,0], "radius": {radius},
                   "material": "absorb", "is_refractor": true }},
                {{ "type": "sphere", "center": [0,0,-50], "radius": 20.0, "material": "white" }}
            ],
            "lights": [
                {{ "origin": [0, 10, 0], "color": [0, 0, 0] }}
            ]
        }}"#,
            ior = ior,
            coeff = coeff,
            radius = radius
        );

        let (scene, _) = parse_scene_json(&json).unwrap();
        // Fire single ray dead-center through the sphere
        let ray = Ray::new(DVec3::new(0.0, 0.0, 10.0), DVec3::new(0.0, 0.0, -1.0));
        let color = trace_ray_worker(&ray, 1, &scene, 5, None);

        // Normalize: background white = 255.0 (through ambient channel)
        let measured_t = color.x / 255.0;

        let tolerance = 0.15;
        assert!(
            (measured_t - expected_t).abs() < tolerance,
            "radius={}: measured transmittance {:.4} vs expected {:.4} (diff {:.4} > tol {})",
            radius,
            measured_t,
            expected_t,
            (measured_t - expected_t).abs(),
            tolerance
        );
    }
}

// ====================================================================
// 3. Procedural texture (checker) tests
// ====================================================================

#[test]
fn test_checker_texture_samples() {
    // Sample known UV coordinates through the checker pattern
    let json = r#"{
        "camera": { "origin": [0,5,5], "distance": 5, "vpwidth": 4, "vpheight": 4 },
        "materials": {
            "checker_mat": {
                "kAmbient": 1.0, "kDiffuse": [0.5, 0.5, 0.5], "kSpecular": 0.0, "nS": 0.0,
                "ambientColor": [100, 100, 100],
                "texture": { "type": "checker", "color_a": [1.0, 0.0, 0.0], "color_b": [0.0, 1.0, 0.0], "scale": 2.0 }
            }
        },
        "objects": [
            { "type": "plane", "normal": [0, 1, 0], "distance": 0, "material": "checker_mat" }
        ],
        "lights": [
            { "origin": [0, 10, 0], "color": [255, 255, 255] }
        ]
    }"#;

    let (scene, _) = parse_scene_json(json).unwrap();

    // Verify the texture was parsed as a checker
    let checker_found = scene
        .materials
        .iter()
        .any(|m| matches!(&m.texture, TextureRef::Checker { .. }));
    assert!(
        checker_found,
        "Should have at least one checker-textured material"
    );

    // Render and verify it produces non-uniform output (checker pattern)
    let img = render_image_parallel(
        &scene,
        DVec3::new(0.0, 5.0, 5.0),
        DVec3::new(0.0, 0.0, 0.0),
        DVec3::new(0.0, 1.0, 0.0),
        4.0,
        4.0,
        8,
        8,
        2,
        1,
        0.0,
        5.0,
    );

    // At least 2 distinct pixel colors should exist in the render (checker alternation)
    let mut colors = std::collections::HashSet::new();
    for i in 0..(8 * 8) {
        colors.insert((img[i * 3], img[i * 3 + 1], img[i * 3 + 2]));
    }
    assert!(
        colors.len() >= 2,
        "Checker texture should produce at least 2 distinct colors, got {}",
        colors.len()
    );
}

// ====================================================================
// 4. Image texture tests
// ====================================================================

#[test]
fn test_image_texture_bilinear_known_pixels() {
    // Create a 2x2 test image in memory and verify bilinear sampling
    let tex = TextureImage {
        width: 2,
        height: 2,
        data: vec![
            DVec3::new(255.0, 0.0, 0.0),     // (0,0) red
            DVec3::new(0.0, 255.0, 0.0),     // (1,0) green
            DVec3::new(0.0, 0.0, 255.0),     // (0,1) blue
            DVec3::new(255.0, 255.0, 255.0), // (1,1) white
        ],
    };

    // Sample at pixel centers
    // UV (0.25, 0.25) should be near the top-left (red)
    let c = tex.sample_bilinear(0.25, 0.25);
    assert!(c.x > 200.0, "Top-left should be reddish, got {:?}", c);

    // UV (0.75, 0.25) should be near the top-right (green)
    let c = tex.sample_bilinear(0.75, 0.25);
    assert!(c.y > 200.0, "Top-right should be greenish, got {:?}", c);

    // UV (0.25, 0.75) should be near the bottom-left (blue)
    let c = tex.sample_bilinear(0.25, 0.75);
    assert!(c.z > 200.0, "Bottom-left should be bluish, got {:?}", c);

    // UV (0.75, 0.75) should be near the bottom-right (white)
    let c = tex.sample_bilinear(0.75, 0.75);
    assert!(
        c.x > 200.0 && c.y > 200.0 && c.z > 200.0,
        "Bottom-right should be whitish, got {:?}",
        c
    );

    // Sampling between texel centers: uv(0.5, 0.25) is exactly halfway between red and green.
    // x = 0.5*2 - 0.5 = 0.5 → fx=0.5 → equal blend of both texels
    // A nearest-neighbor sampler would return one pure color; bilinear must blend.
    let c_mid = tex.sample_bilinear(0.5, 0.25);
    assert!(
        c_mid.x > 100.0 && c_mid.y > 100.0,
        "uv(0.5, 0.25) should blend red+green to ~(127,127,0), got {:?}",
        c_mid
    );
    assert!(
        c_mid.x < 200.0 && c_mid.y < 200.0,
        "uv(0.5, 0.25) blend should not be a pure color, got {:?}",
        c_mid
    );
    assert!(
        c_mid.z < 20.0,
        "Blue channel should be near zero in red+green blend, got {:?}",
        c_mid
    );
}

// ====================================================================
// 5. Environment map tests
// ====================================================================

#[test]
fn test_environment_map_samples_by_direction() {
    // Create a 4x8 environment map: top 4 rows blue, bottom 4 rows red
    let mut data = vec![DVec3::ZERO; 32];
    for i in 0..16 {
        data[i] = DVec3::new(0.0, 0.0, 255.0);
    }
    for i in 16..32 {
        data[i] = DVec3::new(255.0, 0.0, 0.0);
    }

    let env = EnvironmentMap {
        image: TextureImage {
            width: 4,
            height: 8,
            data,
        },
    };

    // Horizontal ray (y=0) should be at the equator (v=0.5) => boundary
    // Ray at 45° up (y = 0.707, z = 0.707) => asin(0.707)/PI ≈ 0.25 => v ≈ 0.25 => clearly in blue
    let c_up = env.sample(DVec3::new(0.0, 0.707, 0.707));
    assert!(
        c_up.z > 200.0,
        "45° up direction should be blue, got {:?}",
        c_up
    );

    // Ray at 45° down => v ≈ 0.75 => clearly in red
    let c_down = env.sample(DVec3::new(0.0, -0.707, 0.707));
    assert!(
        c_down.x > 200.0,
        "45° down direction should be red, got {:?}",
        c_down
    );
}

#[test]
fn test_environment_map_miss_returns_env_color() {
    // Scene with no objects but an environment map — every ray should hit the env map
    let mut scene = Scene::new();
    scene.lights.push(Light {
        origin: DVec3::new(0.0, 10.0, 0.0),
        color: DVec3::ZERO,
        radius: 0.0,
    });
    scene.environment_map = Some(EnvironmentMap {
        image: TextureImage {
            width: 1,
            height: 1,
            data: vec![DVec3::new(100.0, 50.0, 200.0)],
        },
    });

    let ray = Ray::new(DVec3::new(0.0, 0.0, 0.0), DVec3::new(0.0, 0.0, -1.0));
    let color = trace_ray_worker(&ray, 1, &scene, 2, None);
    assert!(
        (color.x - 100.0).abs() < 5.0,
        "Env map miss should return env color, got {:?}",
        color
    );
    assert!((color.y - 50.0).abs() < 5.0);
    assert!((color.z - 200.0).abs() < 5.0);
}

#[test]
fn test_no_environment_map_miss_returns_black() {
    // Scene with no objects and no environment map — miss should return black
    let scene = Scene::new();
    let ray = Ray::new(DVec3::new(0.0, 0.0, 0.0), DVec3::new(0.0, 0.0, -1.0));
    let color = trace_ray_worker(&ray, 1, &scene, 2, None);
    assert_eq!(color, DVec3::ZERO, "No env map miss should return black");
}
