use hairy_tracer_core::camera::CameraOrientation;
use hairy_tracer_core::scene_parser::parse_scene_json;
use hairy_tracer_core::render::{render_image_parallel, WhittedIntegrator};
use glam::DVec3;
use std::fs;
use image::{ImageBuffer, Rgb};

fn main() {
    let json_str = fs::read_to_string("../scenes/whitted/legacy/spheres3.json").unwrap();
    let (scene, mut cam) = parse_scene_json(&json_str).unwrap();
    
    // Original orientation: looking straight at -Z
    let origin = DVec3::new(0.0, 0.0, -5.0);
    let look_at1 = DVec3::new(0.0, 0.0, -4.0);
    let up1 = DVec3::new(0.0, 1.0, 0.0);
    let q1 = CameraOrientation::from_look_at(origin, look_at1, up1);
    
    // Target orientation: looking right and slightly down
    let look_at2 = DVec3::new(3.0, -1.0, -4.0);
    let q2 = CameraOrientation::from_look_at(origin, look_at2, up1);
    
    let frames = 5;
    for i in 0..frames {
        let t = i as f64 / (frames - 1) as f64;
        let qt = q1.slerp(&q2, t);
        
        let (u, v, w) = qt.basis_vectors();
        
        let new_look_at = origin - w;
        let new_up = v;
        
        let width = 200;
        let height = 200;
        
        let integrator = WhittedIntegrator;
        
        let pixels = render_image_parallel(
            &integrator,
            &scene,
            origin,
            new_look_at,
            new_up,
            1.0,
            1.0,
            1.0,
            width,
            height,
            2,
            1,
            0.0,
            1.0,
        );
        
        let img = ImageBuffer::<Rgb<u8>, _>::from_raw(width as u32, height as u32, pixels).unwrap();
        img.save(format!("../slerp_frame_{}.bmp", i)).unwrap();
        println!("Saved frame {}", i);
    }
}
