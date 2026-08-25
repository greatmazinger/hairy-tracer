use glam::DVec3;
use crate::intersect::{Intersectable, Interval};
use crate::hit::Hit;
use crate::ray::Ray;
use crate::material::MaterialId;

#[derive(Debug, Clone)]
pub struct Cube {
    pub min: DVec3,
    pub max: DVec3,
    pub material_id: MaterialId,
}

impl Cube {
    pub fn new(min: DVec3, max: DVec3, material_id: MaterialId) -> Self {
        Self { min, max, material_id }
    }

    fn normal_at_t(&self, ray: &Ray, t: f64, is_tmin: bool) -> DVec3 {
        let p = ray.at(t);
        let center = (self.min + self.max) * 0.5;
        let d = p - center;
        let extents = (self.max - self.min) * 0.5;

        // Find which face we're on by finding the dimension where distance from center
        // is closest to the extent.
        let dx = (d.x.abs() - extents.x).abs();
        let dy = (d.y.abs() - extents.y).abs();
        let dz = (d.z.abs() - extents.z).abs();

        if dx < dy && dx < dz {
            DVec3::new(if d.x > 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0)
        } else if dy < dx && dy < dz {
            DVec3::new(0.0, if d.y > 0.0 { 1.0 } else { -1.0 }, 0.0)
        } else {
            DVec3::new(0.0, 0.0, if d.z > 0.0 { 1.0 } else { -1.0 })
        }
    }
}

const DIR_EPSILON: f64 = 1e-8;

impl Intersectable for Cube {
    fn intersect(&self, ray: &Ray, object_index: usize) -> Option<Hit> {
        let intervals = self.intervals(ray, object_index);
        for inv in intervals {
            if inv.t_enter > 1e-4 {
                return Some(inv.hit_enter);
            } else if inv.t_exit > 1e-4 {
                return Some(inv.hit_exit);
            }
        }
        None
    }

    fn intervals(&self, ray: &Ray, object_index: usize) -> Vec<Interval> {
        let dir = DVec3::new(
            if ray.direction.x == 0.0 { DIR_EPSILON } else { ray.direction.x },
            if ray.direction.y == 0.0 { DIR_EPSILON } else { ray.direction.y },
            if ray.direction.z == 0.0 { DIR_EPSILON } else { ray.direction.z },
        );

        let inv_dir = DVec3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);

        let tmin = (self.min - ray.origin) * inv_dir;
        let tmax = (self.max - ray.origin) * inv_dir;

        let t1 = tmin.min(tmax);
        let t2 = tmin.max(tmax);

        let tnear = t1.x.max(t1.y).max(t1.z);
        let tfar = t2.x.min(t2.y).min(t2.z);

        if tnear > tfar {
            return vec![]; // Miss
        }

        let hit_enter = Hit {
            t: tnear,
            point: ray.at(tnear),
            normal: self.normal_at_t(ray, tnear, true),
            object_index,
            material_id: self.material_id,
            u: 0.0, // UV mapping not strictly defined for CSG cube out of the box, keep 0
            v: 0.0,
        };

        let hit_exit = Hit {
            t: tfar,
            point: ray.at(tfar),
            normal: self.normal_at_t(ray, tfar, false),
            object_index,
            material_id: self.material_id,
            u: 0.0,
            v: 0.0,
        };

        vec![Interval {
            t_enter: tnear,
            t_exit: tfar,
            hit_enter,
            hit_exit,
        }]
    }
}
