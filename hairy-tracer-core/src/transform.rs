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
        
        let fwd_transform = translation_mat * rotation_mat;
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
        
        // Transform: rotate 90 degrees around Y, then translate by [5, 0, 0]
        let transform_node = TransformNode::new(cube, DVec3::new(5.0, 0.0, 0.0), DVec3::new(0.0, 90.0, 0.0));
        
        // Let's test a ray that targets the front face (+Z face normally, but rotated 90 deg around Y means it's now facing +X).
        // Wait, original +Z face has normal (0, 0, 1).
        // Rotate +90 degrees around Y (using right-hand rule, +Z rotates towards +X).
        // New normal should be (1, 0, 0).
        // New position of the center of that face was (0, 0, 1).
        // Rotated 90 deg around Y: (1, 0, 0).
        // Translated by 5 along X: (6, 0, 0).
        
        // Ray origin at (10, 0, 0), pointing in -X direction (-1, 0, 0).
        // It should hit the transformed face at (6, 0, 0) with normal (1, 0, 0).
        let ray = Ray::new(DVec3::new(10.0, 0.0, 0.0), DVec3::new(-1.0, 0.0, 0.0));
        
        let hit = transform_node.intersect(&ray, 0).expect("Ray should hit the transformed cube");
        
        // The distance from (10, 0, 0) to (6, 0, 0) is 4.0
        assert!((hit.t - 4.0).abs() < 1e-5, "t should be 4.0, got {}", hit.t);
        
        // The hit point should be exactly (6, 0, 0)
        assert!((hit.point.x - 6.0).abs() < 1e-5);
        assert!((hit.point.y - 0.0).abs() < 1e-5);
        assert!((hit.point.z - 0.0).abs() < 1e-5);
        
        // The normal should be exactly (1, 0, 0)
        assert!((hit.normal.x - 1.0).abs() < 1e-5);
        assert!((hit.normal.y - 0.0).abs() < 1e-5);
        assert!((hit.normal.z - 0.0).abs() < 1e-5);
    }
}
