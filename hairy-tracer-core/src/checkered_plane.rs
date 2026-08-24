use glam::DVec3;

use crate::hit::Hit;
use crate::intersect::Intersectable;
use crate::material::MaterialId;
use crate::plane::Plane;
use crate::ray::Ray;

/// A checkered infinite plane — two materials tiled in a checkerboard pattern.
///
/// Inherits the intersection math from `Plane`; the only difference is which
/// `MaterialId` is returned in the `Hit`.
///
/// **Tiling logic** (from Python `CheckeredPlane.GetColor`, lines 154-157):
/// ```text
/// let ix = round(point.x) as i64;
/// let iz = round(point.z) as i64;
/// if (ix % 2 != 0) XOR (iz % 2 != 0)  →  material_a
/// else  →  material_b
/// ```
///
/// The Python code uses `int(round(intpoint[0] * 1.0)) % 2` — the `* 1.0` is
/// a no-op, and it checks whether exactly one of x, z rounds to odd.
pub struct CheckeredPlane {
    plane: Plane,
    pub material_a: MaterialId,
    pub material_b: MaterialId,
}

impl CheckeredPlane {
    pub fn new(
        normal: DVec3,
        distance: f64,
        material_a: MaterialId,
        material_b: MaterialId,
    ) -> Self {
        // The inner Plane's material_id is unused; we override it per-hit.
        Self {
            plane: Plane::new(normal, distance, material_a),
            material_a,
            material_b,
        }
    }

    /// Determine which material applies at the given world-space point.
    ///
    /// Uses the same `round → int → mod 2` XOR logic as the Python code.
    fn pick_material(&self, point: DVec3) -> MaterialId {
        let ix = point.x.round() as i64;
        let iz = point.z.round() as i64;

        let x_odd = ix.rem_euclid(2) == 1;
        let z_odd = iz.rem_euclid(2) == 1;

        if x_odd ^ z_odd {
            self.material_a
        } else {
            self.material_b
        }
    }
}

impl Intersectable for CheckeredPlane {
    fn intersect(&self, ray: &Ray, object_index: usize) -> Option<Hit> {
        self.plane.intersect(ray, object_index).map(|mut hit| {
            hit.material_id = self.pick_material(hit.point);
            hit
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAT_A: MaterialId = MaterialId(1);
    const MAT_B: MaterialId = MaterialId(2);

    fn make_floor() -> CheckeredPlane {
        // Floor plane at y=0 facing +Y
        CheckeredPlane::new(DVec3::new(0.0, 1.0, 0.0), 0.0, MAT_A, MAT_B)
    }

    #[test]
    fn checker_picks_material_a_for_odd_xor() {
        let floor = make_floor();
        // x=1, z=0 → ix=1 (odd), iz=0 (even) → XOR true → material_a
        let r = Ray::new(DVec3::new(1.0, 5.0, 0.0), DVec3::new(0.0, -1.0, 0.0));
        let hit = floor.intersect(&r, 0).expect("expected hit");
        assert_eq!(hit.material_id, MAT_A);
    }

    #[test]
    fn checker_picks_material_b_for_both_even() {
        let floor = make_floor();
        // x=0, z=0 → both even → XOR false → material_b
        let r = Ray::new(DVec3::new(0.0, 5.0, 0.0), DVec3::new(0.0, -1.0, 0.0));
        let hit = floor.intersect(&r, 0).expect("expected hit");
        assert_eq!(hit.material_id, MAT_B);
    }

    #[test]
    fn checker_picks_material_b_for_both_odd() {
        let floor = make_floor();
        // x=1, z=1 → both odd → XOR false → material_b
        let r = Ray::new(DVec3::new(1.0, 5.0, 1.0), DVec3::new(0.0, -1.0, 0.0));
        let hit = floor.intersect(&r, 0).expect("expected hit");
        assert_eq!(hit.material_id, MAT_B);
    }

    #[test]
    fn checker_negative_coordinates() {
        let floor = make_floor();
        // x = -1.0, z = 0.0 → ix = -1, rem_euclid(2) = 1 (odd), iz = 0 (even)
        // XOR true → material_a
        let r = Ray::new(DVec3::new(-1.0, 5.0, 0.0), DVec3::new(0.0, -1.0, 0.0));
        let hit = floor.intersect(&r, 0).expect("expected hit");
        assert_eq!(hit.material_id, MAT_A);
    }
}
