use glam::DVec3;
use crate::intersect::{Intersectable, Interval};
use crate::hit::Hit;
use crate::ray::Ray;
use crate::material::MaterialId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis { X, Y, Z }

#[derive(Debug, Clone)]
pub struct Cylinder {
    pub center: DVec3,
    pub radius: f64,
    pub height: f64,
    pub axis: Axis,
    pub material_id: MaterialId,
}

impl Intersectable for Cylinder {
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
        // Shift ray to cylinder's local space (centered at origin)
        let oc = ray.origin - self.center;
        
        let (oc_a, oc_b, oc_c, d_a, d_b, d_c) = match self.axis {
            Axis::X => (oc.y, oc.z, oc.x, ray.direction.y, ray.direction.z, ray.direction.x),
            Axis::Y => (oc.x, oc.z, oc.y, ray.direction.x, ray.direction.z, ray.direction.y),
            Axis::Z => (oc.x, oc.y, oc.z, ray.direction.x, ray.direction.y, ray.direction.z),
        };

        let a = d_a * d_a + d_b * d_b;
        let half_b = oc_a * d_a + oc_b * d_b;
        let c = oc_a * oc_a + oc_b * oc_b - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        let mut t_body1 = f64::NEG_INFINITY;
        let mut t_body2 = f64::INFINITY;

        if a > 1e-8 && discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            t_body1 = (-half_b - sqrtd) / a;
            t_body2 = (-half_b + sqrtd) / a;
            if t_body1 > t_body2 {
                std::mem::swap(&mut t_body1, &mut t_body2);
            }
        } else if a <= 1e-8 && c > 0.0 {
            // Ray is parallel to the axis and completely outside the cylinder tube
            return vec![];
        }

        let half_h = self.height / 2.0;
        
        // Cap planes: oc_c + t * d_c = +/- half_h
        let mut t_cap1 = f64::NEG_INFINITY;
        let mut t_cap2 = f64::INFINITY;

        if d_c.abs() > 1e-8 {
            t_cap1 = (-half_h - oc_c) / d_c;
            t_cap2 = (half_h - oc_c) / d_c;
            if t_cap1 > t_cap2 {
                std::mem::swap(&mut t_cap1, &mut t_cap2);
            }
        } else if oc_c.abs() > half_h {
            // Ray is parallel to caps and outside the height bounds
            return vec![];
        }

        let t_enter = t_body1.max(t_cap1);
        let t_exit = t_body2.min(t_cap2);

        if t_enter > t_exit {
            return vec![]; // Miss
        }

        let create_hit = |t: f64| -> Hit {
            let p = ray.at(t);
            let lp = p - self.center;
            
            let normal = if (t - t_cap1).abs() < 1e-5 || (t - t_cap2).abs() < 1e-5 {
                // Hit a cap
                let sign = if t == t_cap2 && d_c > 0.0 { 1.0 } else if t == t_cap1 && d_c < 0.0 { -1.0 } else { 
                    // determine from lp
                    match self.axis {
                        Axis::X => if lp.x > 0.0 { 1.0 } else { -1.0 },
                        Axis::Y => if lp.y > 0.0 { 1.0 } else { -1.0 },
                        Axis::Z => if lp.z > 0.0 { 1.0 } else { -1.0 },
                    }
                };
                match self.axis {
                    Axis::X => DVec3::new(sign, 0.0, 0.0),
                    Axis::Y => DVec3::new(0.0, sign, 0.0),
                    Axis::Z => DVec3::new(0.0, 0.0, sign),
                }
            } else {
                // Hit the tube
                let mut n = match self.axis {
                    Axis::X => DVec3::new(0.0, lp.y, lp.z),
                    Axis::Y => DVec3::new(lp.x, 0.0, lp.z),
                    Axis::Z => DVec3::new(lp.x, lp.y, 0.0),
                };
                if n.length_squared() > 0.0 { n = n.normalize(); }
                n
            };

            Hit {
                t,
                point: p,
                normal,
                object_index,
                material_id: self.material_id,
                u: 0.0,
                v: 0.0,
            }
        };

        vec![Interval {
            t_enter,
            t_exit,
            hit_enter: create_hit(t_enter),
            hit_exit: create_hit(t_exit),
        }]
    }
}
