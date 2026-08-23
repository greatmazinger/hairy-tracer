use glam::DVec3;

use crate::hit::Hit;
use crate::intersect::Intersectable;
use crate::material::MaterialId;
use crate::ray::Ray;

/// Floating-point near-zero check, matching the Python `fequal` function
/// which uses `abs(a - b) < 1e-6`.
const PARALLEL_EPSILON: f64 = 1e-6;

/// An infinite plane defined by `normal · P + distance = 0`.
///
/// Matches the Python `Plane.findIntersection`:
/// - `vd = dot(normal, Rd)`, reject if `|vd| < 1e-6` (parallel)
/// - `v0 = -(dot(normal, Rorig) + distance)`
/// - `t = v0 / vd`, reject if `t < 0`
pub struct Plane {
    /// Unit normal of the plane (normalized on construction, matching Python).
    pub normal: DVec3,
    /// Signed distance from the origin.
    pub distance: f64,
    pub material_id: MaterialId,
}

impl Plane {
    pub fn new(normal: DVec3, distance: f64, material_id: MaterialId) -> Self {
        Self {
            normal: normal.normalize(),
            distance,
            material_id,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray, object_index: usize) -> Option<Hit> {
        let rd = ray.direction;
        let vd = self.normal.dot(rd);

        // Ray is parallel to the plane (or nearly so).
        if vd.abs() < PARALLEL_EPSILON {
            return None;
        }

        let v0 = -(self.normal.dot(ray.origin) + self.distance);
        let t = v0 / vd;

        // Python rejects `t < 0` (no epsilon here, unlike sphere/triangle).
        if t < 0.0 {
            return None;
        }

        let point = ray.at(t);

        Some(Hit {
            t,
            point,
            normal: self.normal,
            object_index,
            material_id: self.material_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAT: MaterialId = MaterialId(0);

    #[test]
    fn perpendicular_ray_hits_plane() {
        // Plane at z=0 facing +Z
        let p = Plane::new(DVec3::new(0.0, 0.0, 1.0), 0.0, MAT);
        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        let hit = p.intersect(&r, 0).expect("expected hit");

        assert!((hit.t - 5.0).abs() < 1e-9);
        assert!(hit.point.distance(DVec3::ZERO) < 1e-9);
        assert!(hit.normal.distance(DVec3::new(0.0, 0.0, 1.0)) < 1e-9);
    }

    #[test]
    fn parallel_ray_misses_plane() {
        let p = Plane::new(DVec3::new(0.0, 0.0, 1.0), 0.0, MAT);
        // Ray along X, parallel to the Z=0 plane from z=5
        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(1.0, 0.0, 0.0));
        assert!(p.intersect(&r, 0).is_none());
    }

    #[test]
    fn ray_behind_plane_misses() {
        let p = Plane::new(DVec3::new(0.0, 0.0, 1.0), 0.0, MAT);
        // Ray pointing away from the plane
        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, 1.0));
        assert!(p.intersect(&r, 0).is_none());
    }

    #[test]
    fn angled_ray_hits_offset_plane() {
        // Plane at y = 3: normal=(0,1,0), distance=-3 ⟹ y + (-3) = 0 ⟹ y=3
        let p = Plane::new(DVec3::new(0.0, 1.0, 0.0), -3.0, MAT);
        let r = Ray::new(DVec3::new(0.0, 0.0, 0.0), DVec3::new(0.0, 1.0, 0.0));
        let hit = p.intersect(&r, 0).expect("expected hit");

        assert!((hit.t - 3.0).abs() < 1e-9);
        assert!(hit.point.distance(DVec3::new(0.0, 3.0, 0.0)) < 1e-9);
    }
}
