use glam::DVec3;
use crate::integrator::Integrator;
use crate::ray::Ray;
use crate::scene::Scene;

pub struct NormalsIntegrator;

impl Integrator for NormalsIntegrator {
    fn trace_ray(&self, ray: &Ray, _depth: u32, scene: &Scene, _max_depth: u32) -> DVec3 {
        let mut best_hit = None;
        let mut min_t = std::f64::INFINITY;
        
        for (idx, obj) in scene.objects.iter().enumerate() {
            if let Some(hit) = obj.intersect(ray, idx) {
                if hit.t > 0.0 && hit.t < min_t {
                    min_t = hit.t;
                    best_hit = Some(hit);
                }
            }
        }
        
        if let Some(hit) = best_hit {
            // Map UV from [0, 1] to [0, 255]
            let u = hit.u.fract();
            let v = hit.v.fract();
            let u = if u < 0.0 { u + 1.0 } else { u };
            let v = if v < 0.0 { v + 1.0 } else { v };
            return DVec3::new(u * 255.0, v * 255.0, 0.0);
        }
        
        DVec3::new(0.0, 0.0, 0.0) // Background black
    }
}
