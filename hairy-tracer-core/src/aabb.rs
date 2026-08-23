use glam::DVec3;

use crate::ray::Ray;

/// Axis-Aligned Bounding Box, using the slab method for ray intersection.
///
/// Matches the Python `Mesh._intersect_aabb` implementation:
/// - Axes with zero direction component get replaced with `1e-8` to avoid
///   division by zero.
/// - Per-axis t intervals are sorted so `t1 = min(tmin, tmax)` and
///   `t2 = max(tmin, tmax)`.
/// - `tnear = max(t1[0], t1[1], t1[2])`, `tfar = min(t2[0], t2[1], t2[2])`.
/// - Reject if `tnear > tfar` or `tfar < 0`.
///
/// This currently wraps an entire mesh as one box (matching the Python
/// behavior). To upgrade to a recursive BVH later, replace the flat
/// triangle list in `Mesh` with a tree of AABB nodes — the `Intersectable`
/// trait interface stays the same.
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: DVec3,
    pub max: DVec3,
}

/// Same epsilon the Python code uses for zero-direction components.
const DIR_EPSILON: f64 = 1e-8;

impl Aabb {
    /// Build an AABB from a set of points (e.g. all triangle vertices).
    pub fn from_points(points: impl IntoIterator<Item = DVec3>) -> Self {
        let mut min = DVec3::splat(f64::INFINITY);
        let mut max = DVec3::splat(f64::NEG_INFINITY);

        for p in points {
            min = min.min(p);
            max = max.max(p);
        }

        Self { min, max }
    }

    /// Test whether `ray` intersects this AABB using the slab method.
    ///
    /// Returns `true` if the ray passes through the box (at any `t`,
    /// including behind the origin when `tfar >= 0`).
    pub fn intersects(&self, ray: &Ray) -> bool {
        // Replace zero components with a tiny epsilon, matching the Python code:
        // `dir = numpy.where(dir == 0, 1e-8, dir)`
        let dir = DVec3::new(
            if ray.direction.x == 0.0 { DIR_EPSILON } else { ray.direction.x },
            if ray.direction.y == 0.0 { DIR_EPSILON } else { ray.direction.y },
            if ray.direction.z == 0.0 { DIR_EPSILON } else { ray.direction.z },
        );

        let inv_dir = DVec3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);

        let tmin = (self.min - ray.origin) * inv_dir;
        let tmax = (self.max - ray.origin) * inv_dir;

        // Per-axis sort
        let t1 = tmin.min(tmax);
        let t2 = tmin.max(tmax);

        let tnear = t1.x.max(t1.y).max(t1.z);
        let tfar = t2.x.min(t2.y).min(t2.z);

        // Python: `if tnear > tfar or tfar < 0: return False`
        !(tnear > tfar || tfar < 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_box() -> Aabb {
        Aabb {
            min: DVec3::new(-1.0, -1.0, -1.0),
            max: DVec3::new(1.0, 1.0, 1.0),
        }
    }

    #[test]
    fn ray_hits_aabb() {
        let b = unit_box();
        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        assert!(b.intersects(&r));
    }

    #[test]
    fn ray_misses_aabb() {
        let b = unit_box();
        // Ray far above the box, parallel to it
        let r = Ray::new(DVec3::new(0.0, 5.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        assert!(!b.intersects(&r));
    }

    #[test]
    fn ray_axis_aligned_with_zero_component() {
        let b = unit_box();
        // Ray along +X from far left — direction has y=0, z=0
        let r = Ray::new(DVec3::new(-5.0, 0.0, 0.0), DVec3::new(1.0, 0.0, 0.0));
        assert!(b.intersects(&r));
    }

    #[test]
    fn ray_origin_inside_aabb() {
        let b = unit_box();
        let r = Ray::new(DVec3::ZERO, DVec3::new(1.0, 0.0, 0.0));
        assert!(b.intersects(&r));
    }

    #[test]
    fn ray_pointing_away_from_aabb() {
        let b = unit_box();
        // Origin beyond the box, pointing further away
        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, 1.0));
        assert!(!b.intersects(&r));
    }
}
