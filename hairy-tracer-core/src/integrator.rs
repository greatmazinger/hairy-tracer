use glam::DVec3;
use crate::ray::Ray;
use crate::scene::Scene;

pub trait Integrator: Send + Sync {
    fn trace_ray(
        &self,
        ray: &Ray,
        depth: u32,
        scene: &Scene,
        max_depth: u32,
    ) -> DVec3;
}
