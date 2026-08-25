use glam::DVec3;
use hairy_tracer_core::render::render_image_parallel;
use hairy_tracer_core::scene_parser::parse_scene_json;

#[test]
fn test_anti_aliasing_smoothes_edges() {
    let json = r#"{
        "camera": { "origin": [0,0,5], "distance": 5, "vpwidth": 1, "vpheight": 1 },
        "materials": {
            "white": { "kAmbient": 1.0, "kDiffuse": [0,0,0], "kSpecular": 0.0, "nS": 0.0, "ambientColor": [255,255,255] }
        },
        "objects": [
            { "type": "sphere", "center": [10.125, 0, 0], "radius": 10.0, "material": "white" }
        ],
        "lights": [
            { "origin": [0, 10, 0], "color": [0, 0, 0], "radius": 0.0 }
        ]
    }"#;

    let (scene, _) = parse_scene_json(json).unwrap();
    let cam_origin = DVec3::new(0.0, 0.0, 5.0);
    let look_at = DVec3::new(0.0, 0.0, 0.0);
    let up = DVec3::new(0.0, 1.0, 0.0);

    // Render at 1 SPP (width=4, height=1)
    let img_1spp = render_image_parallel(&hairy_tracer_core::render::WhittedIntegrator, &scene, cam_origin, look_at, up, 1.0, 1.0, 4, 1, 1, 1, 0.0, 5.0,
    );

    let mut has_intermediate_1spp = false;
    for i in 0..4 {
        let val = img_1spp[i * 3];
        if val > 0 && val < 255 {
            has_intermediate_1spp = true;
        }
    }
    assert!(!has_intermediate_1spp, "1 SPP should be completely aliased");

    // Render at 100 SPP
    let img_100spp = render_image_parallel(&hairy_tracer_core::render::WhittedIntegrator, &scene, cam_origin, look_at, up, 1.0, 1.0, 4, 1, 1, 100, 0.0, 5.0,
    );
    let mut has_intermediate_100spp = false;
    for i in 0..4 {
        let val = img_100spp[i * 3];
        if val > 0 && val < 255 {
            has_intermediate_100spp = true;
            println!("Found intermediate 100spp at {}: {}", i, val);
        }
        println!("100spp pixel {}: {}", i, val);
    }
    assert!(
        has_intermediate_100spp,
        "100 SPP should have intermediate pixel colors at the boundary"
    );
}

#[test]
fn test_soft_shadows_penumbra() {
    let json = r#"{
        "camera": { "origin": [0,5,5], "distance": 5, "vpwidth": 5, "vpheight": 5 },
        "materials": {
            "matte": { "kAmbient": 0.0, "kDiffuse": [1,1,1], "kSpecular": 0.0, "nS": 0.0 }
        },
        "objects": [
            { "type": "plane", "normal": [0, 1, 0], "distance": 0, "material": "matte" },
            { "type": "sphere", "center": [0, 2, 0], "radius": 1, "material": "matte" }
        ],
        "lights": [
            { "origin": [2, 4, 0], "color": [255, 255, 255], "radius": 0.0 }
        ]
    }"#;

    // Light is at (2,4,0). Blocker at (0,2,0). Shadow cast on plane at y=0.
    // Hard shadow will be binary. Soft shadow will have intermediate occlusion.

    let (mut scene, _) = parse_scene_json(json).unwrap();
    let cam_origin = DVec3::new(0.0, 5.0, 5.0);
    let look_at = DVec3::new(0.0, 0.0, 0.0);
    let up = DVec3::new(0.0, 1.0, 0.0);

    let img_hard = render_image_parallel(&hairy_tracer_core::render::WhittedIntegrator, &scene, cam_origin, look_at, up, 5.0, 5.0, 20, 20, 1, 10, 0.0, 5.0,
    );

    // Make light soft
    scene.lights[0].radius = 1.0;
    let img_soft = render_image_parallel(&hairy_tracer_core::render::WhittedIntegrator, &scene, cam_origin, look_at, up, 5.0, 5.0, 20, 20, 1, 100, 0.0, 5.0,
    );

    // Check for intermediate values in the shadow region that didn't exist in the hard shadow.
    // We expect some pixels to be lit in img_hard but dim in img_soft, or black in hard but lit in soft.
    let mut penumbra_detected = false;
    for i in 0..(20 * 20) {
        let hard = img_hard[i * 3] as i32;
        let soft = img_soft[i * 3] as i32;

        // A penumbra pixel is one where the soft shadow is somewhere between fully black and the hard lit value,
        // or a pixel that was hard-shadowed but is now partially lit.
        if (hard - soft).abs() > 30 && soft > 10 && soft < 200 {
            penumbra_detected = true;
            break;
        }
    }

    assert!(
        penumbra_detected,
        "Soft shadow should create a penumbra region with intermediate lighting"
    );
}

#[test]
fn test_dof_blurs_off_focal_plane() {
    let json = r#"{
        "camera": { "origin": [0,0,10], "distance": 10, "vpwidth": 2, "vpheight": 2 },
        "materials": {
            "white": { "kAmbient": 1.0, "kDiffuse": [0,0,0], "kSpecular": 0.0, "nS": 0.0, "ambientColor": [255,255,255] }
        },
        "objects": [
            { "type": "sphere", "center": [-1, 0, 5], "radius": 0.5, "material": "white" },
            { "type": "sphere", "center": [1, 0, 0], "radius": 0.5, "material": "white" }
        ],
        "lights": [
            { "origin": [0, 10, 0], "color": [0, 0, 0], "radius": 0.0 }
        ]
    }"#;

    // Two spheres: one at z=5 (near), one at z=0 (far).
    // Camera is at z=10.

    let (scene, _) = parse_scene_json(json).unwrap();
    let cam_origin = DVec3::new(0.0, 0.0, 10.0);
    let look_at = DVec3::new(0.0, 0.0, 0.0); // Viewport plane at z=0? No, distance is 10, w_norm=10.
    let up = DVec3::new(0.0, 1.0, 0.0);

    // 1. Focus at z=5 (focal_distance = 5). Near sphere is sharp, far sphere is blurred.
    let img_focus_near = render_image_parallel(&hairy_tracer_core::render::WhittedIntegrator, &scene, cam_origin, look_at, up, 2.0, 2.0, 20, 20, 1, 50, 1.0, 5.0,
    );

    // 2. Focus at z=0 (focal_distance = 10). Far sphere is sharp, near sphere is blurred.
    let img_focus_far = render_image_parallel(&hairy_tracer_core::render::WhittedIntegrator, &scene, cam_origin, look_at, up, 2.0, 2.0, 20, 20, 1, 50, 1.0, 10.0,
    );

    // Since spheres are just white circles on black background, "blurred" means anti-aliased soft edges (intermediate pixels).
    // "Sharp" means mostly binary (0 or 255) with very few intermediate pixels due to standard AA.

    fn count_intermediate(img: &[u8], is_left: bool) -> usize {
        let mut count = 0;
        for y in 0..20 {
            for x in 0..20 {
                if (is_left && x < 10) || (!is_left && x >= 10) {
                    let val = img[(y * 20 + x) * 3];
                    if val > 10 && val < 245 {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    let near_blur_when_focus_near = count_intermediate(&img_focus_near, true);
    let far_blur_when_focus_near = count_intermediate(&img_focus_near, false);

    let near_blur_when_focus_far = count_intermediate(&img_focus_far, true);
    let far_blur_when_focus_far = count_intermediate(&img_focus_far, false);

    println!("near_blur_when_focus_near: {}", near_blur_when_focus_near);
    println!("far_blur_when_focus_near: {}", far_blur_when_focus_near);
    println!("near_blur_when_focus_far: {}", near_blur_when_focus_far);
    println!("far_blur_when_focus_far: {}", far_blur_when_focus_far);

    // When focused near, the far object should have way more intermediate (blurry) pixels than the near object.
    assert!(far_blur_when_focus_near > near_blur_when_focus_near + 5);

    // When focused far, the near object should have way more intermediate (blurry) pixels than the far object.
    assert!(near_blur_when_focus_far > far_blur_when_focus_far + 5);
}
