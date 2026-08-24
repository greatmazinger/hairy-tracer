use glam::DVec3;

use crate::hit::Hit;
use crate::intersect::Intersectable;
use crate::material::MaterialId;
use crate::ray::Ray;

/// A sphere defined by its center and radius.
///
/// Intersection uses the standard quadratic formula with the ray direction
/// pre-normalized (so the `A` coefficient is 1).
///
/// Matches the Python `Sphere.findIntersection`:
/// - `B = 2 * dot(Rd, Og - Ce)`
/// - `C = |Og - Ce|² - r²`
/// - discriminant = `B² - 4C`
/// - `t` epsilon: `t > 0.01`
pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material_id: MaterialId,
    /// Pre-computed radius squared.
    r2: f64,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material_id: MaterialId) -> Self {
        Self {
            center,
            radius,
            material_id,
            r2: radius * radius,
        }
    }

    /// Compute the outward unit normal at a point on the surface.
    pub fn unit_normal(&self, point: DVec3) -> DVec3 {
        (point - self.center) / self.radius
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, object_index: usize) -> Option<Hit> {
        let rd = ray.direction; // Already normalized by Ray::new
        let og = ray.origin;
        let oc = og - self.center;

        // A = 1 because rd is unit-length
        let b = 2.0 * rd.dot(oc);
        let c = oc.dot(oc) - self.r2;
        let disc = b * b - 4.0 * c;

        if disc < 0.0 {
            return None;
        }

        let sq_disc = disc.sqrt();

        // Try the nearer root first.
        // Python uses `t > 0.01` — same epsilon here.
        let mut t = (-b - sq_disc) * 0.5;
        if t <= 0.01 {
            t = (-b + sq_disc) * 0.5;
        }
        if t <= 0.01 {
            return None;
        }

        let point = ray.at(t);
        let normal = self.unit_normal(point);

        let u = 0.5 + (normal.z.atan2(normal.x)) / (2.0 * std::f64::consts::PI);
        let v = 0.5 + normal.y.asin() / std::f64::consts::PI;

        Some(Hit {
            t,
            point,
            normal,
            object_index,
            material_id: self.material_id,
            u,
            v,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAT: MaterialId = MaterialId(0);

    #[test]
    fn ray_hits_unit_sphere_head_on() {
        let s = Sphere::new(DVec3::ZERO, 1.0, MAT);
        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        let hit = s.intersect(&r, 0).expect("expected hit");

        assert!((hit.t - 4.0).abs() < 1e-9, "t should be 4.0, got {}", hit.t);
        assert!(hit.point.distance(DVec3::new(0.0, 0.0, 1.0)) < 1e-9);
        assert!(hit.normal.distance(DVec3::new(0.0, 0.0, 1.0)) < 1e-9);
    }

    #[test]
    fn ray_misses_sphere() {
        let s = Sphere::new(DVec3::ZERO, 1.0, MAT);
        // Ray far above the sphere, moving along -Z
        let r = Ray::new(DVec3::new(0.0, 5.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        assert!(s.intersect(&r, 0).is_none());
    }

    #[test]
    fn ray_tangent_to_sphere() {
        // Ray grazing the top of a unit sphere at origin.
        // Origin at (0, 1, 5), direction (0, 0, -1). The closest approach is
        // y=1 which is exactly the radius — tangent hit.
        let s = Sphere::new(DVec3::ZERO, 1.0, MAT);
        let r = Ray::new(DVec3::new(0.0, 1.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        let hit = s.intersect(&r, 0).expect("expected tangent hit");

        // At tangent, hit point is (0, 1, 0)
        assert!(hit.point.distance(DVec3::new(0.0, 1.0, 0.0)) < 1e-6);
        // Normal at tangent should point straight up
        assert!(hit.normal.distance(DVec3::new(0.0, 1.0, 0.0)) < 1e-6);
    }

    #[test]
    fn ray_inside_sphere_picks_far_root() {
        // Origin inside the sphere — the near root is negative, so we take the far root.
        let s = Sphere::new(DVec3::ZERO, 2.0, MAT);
        let r = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, 1.0));
        let hit = s.intersect(&r, 0).expect("expected hit from inside");

        assert!((hit.t - 2.0).abs() < 1e-9, "t should be 2.0, got {}", hit.t);
    }
}
