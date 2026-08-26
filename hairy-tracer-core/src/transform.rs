use crate::intersect::{Intersectable, Interval};
use crate::hit::Hit;
use crate::ray::Ray;
use glam::{DVec3, DMat4, DMat3};
use std::boxed::Box;

pub struct TransformNode {
    pub child: Box<dyn Intersectable>,
    pub inv_transform: DMat4,
    pub fwd_transform: DMat4,
    pub fwd_rotation: DMat3,
}

impl TransformNode {
    pub fn new(child: Box<dyn Intersectable>, translate: DVec3, rotate_degrees: DVec3) -> Self {
        let rx = rotate_degrees.x.to_radians();
        let ry = rotate_degrees.y.to_radians();
        let rz = rotate_degrees.z.to_radians();

        let translation_mat = DMat4::from_translation(translate);
        let rotation_mat = DMat4::from_euler(glam::EulerRot::XYZ, rx, ry, rz);
        
        // Composition order: translate first, then rotate.
        // In matrix math (column-major), `A * B * v` means apply B first, then A.
        // So `rotation_mat * translation_mat` applies the translation to the local vertex,
        // and THEN rotates that already-offset position around the origin.
        // This allows placing objects (like gear teeth) at a radius and sweeping them around the center.
        let fwd_transform = rotation_mat * translation_mat;
        let inv_transform = fwd_transform.inverse();
        
        let fwd_rotation = DMat3::from_euler(glam::EulerRot::XYZ, rx, ry, rz);

        Self {
            child,
            inv_transform,
            fwd_transform,
            fwd_rotation,
        }
    }

    fn transform_ray(&self, ray: &Ray) -> Ray {
        let local_origin = self.inv_transform.transform_point3(ray.origin);
        // Transform the direction without normalizing it immediately.
        // Wait, Ray::new normalizes the direction. If there is no scale, the length is preserved anyway.
        let local_dir = self.inv_transform.transform_vector3(ray.direction);
        Ray::new(local_origin, local_dir)
    }

    fn transform_hit(&self, mut hit: Hit) -> Hit {
        hit.point = self.fwd_transform.transform_point3(hit.point);
        hit.normal = self.fwd_rotation.mul_vec3(hit.normal).normalize();
        hit
    }
}

impl Intersectable for TransformNode {
    fn intersect(&self, ray: &Ray, object_index: usize) -> Option<Hit> {
        let local_ray = self.transform_ray(ray);
        self.child.intersect(&local_ray, object_index).map(|hit| self.transform_hit(hit))
    }

    fn intervals(&self, ray: &Ray, object_index: usize) -> Vec<Interval> {
        let local_ray = self.transform_ray(ray);
        let local_intervals = self.child.intervals(&local_ray, object_index);
        
        local_intervals.into_iter().map(|inv| Interval {
            t_enter: inv.t_enter,
            t_exit: inv.t_exit,
            hit_enter: self.transform_hit(inv.hit_enter),
            hit_exit: self.transform_hit(inv.hit_exit),
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::Cube;
    use crate::material::MaterialId;

    #[test]
    fn test_transform_rotation_and_translation() {
        // Un-rotated box from [-1, -1, -1] to [1, 1, 1]
        let cube = Box::new(Cube::new(DVec3::new(-1.0, -1.0, -1.0), DVec3::new(1.0, 1.0, 1.0), MaterialId(0)));
        
        // Transform: translate by [0, 0, 2], then rotate 90 degrees around Y
        let transform_node = TransformNode::new(cube, DVec3::new(0.0, 0.0, 2.0), DVec3::new(0.0, 90.0, 0.0));
        
        // The original +Z face center was at (0, 0, 1).
        // Translated by 2 along Z: (0, 0, 3).
        // Rotated 90 deg around Y: (3, 0, 0).
        // Original normal was (0, 0, 1). Rotated 90 deg around Y: (1, 0, 0).
        
        // Ray origin at (10, 0, 0), pointing in -X direction (-1, 0, 0).
        // It should hit the transformed face at (3, 0, 0) with normal (1, 0, 0).
        let ray = Ray::new(DVec3::new(10.0, 0.0, 0.0), DVec3::new(-1.0, 0.0, 0.0));
        
        let hit = transform_node.intersect(&ray, 0).expect("Ray should hit the transformed cube");
        
        // The distance from (10, 0, 0) to (3, 0, 0) is 7.0
        assert!((hit.t - 7.0).abs() < 1e-5, "t should be 7.0, got {}", hit.t);
        
        // The hit point should be exactly (3, 0, 0)
        assert!((hit.point.x - 3.0).abs() < 1e-5);
        assert!((hit.point.y - 0.0).abs() < 1e-5);
        assert!((hit.point.z - 0.0).abs() < 1e-5);
        
        // The normal should be exactly (1, 0, 0)
        assert!((hit.normal.x - 1.0).abs() < 1e-5);
        assert!((hit.normal.y - 0.0).abs() < 1e-5);
        assert!((hit.normal.z - 0.0).abs() < 1e-5);
    }
}
