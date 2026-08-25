use crate::intersect::{Intersectable, Interval};
use crate::hit::Hit;
use crate::ray::Ray;
use std::boxed::Box;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CsgOp {
    Union,
    Intersection,
    Difference,
}

pub struct CsgNode {
    pub left: Box<dyn Intersectable>,
    pub right: Box<dyn Intersectable>,
    pub op: CsgOp,
}

#[derive(Clone)]
struct Boundary {
    t: f64,
    hit: Hit,
    is_enter: bool,
    is_left: bool,
}

impl Intersectable for CsgNode {
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
        let left_intervals = self.left.intervals(ray, object_index);
        let right_intervals = self.right.intervals(ray, object_index);

        let mut boundaries = Vec::new();

        for inv in left_intervals {
            boundaries.push(Boundary { t: inv.t_enter, hit: inv.hit_enter, is_enter: true, is_left: true });
            boundaries.push(Boundary { t: inv.t_exit, hit: inv.hit_exit, is_enter: false, is_left: true });
        }
        for inv in right_intervals {
            boundaries.push(Boundary { t: inv.t_enter, hit: inv.hit_enter, is_enter: true, is_left: false });
            boundaries.push(Boundary { t: inv.t_exit, hit: inv.hit_exit, is_enter: false, is_left: false });
        }

        boundaries.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));

        let mut in_left = false;
        let mut in_right = false;
        let mut in_result = false;

        let mut result_intervals = Vec::new();
        let mut current_enter_t = f64::NEG_INFINITY;
        let mut current_enter_hit = None;

        for b in boundaries {
            if b.is_left {
                in_left = b.is_enter;
            } else {
                in_right = b.is_enter;
            }

            let new_in_result = match self.op {
                CsgOp::Union => in_left || in_right,
                CsgOp::Intersection => in_left && in_right,
                CsgOp::Difference => in_left && !in_right,
            };

            if new_in_result != in_result {
                let mut hit = b.hit;
                
                if self.op == CsgOp::Difference && !b.is_left {
                    hit.normal = -hit.normal;
                }

                if new_in_result {
                    current_enter_t = b.t;
                    current_enter_hit = Some(hit);
                } else {
                    if let Some(enter_hit) = current_enter_hit.take() {
                        if b.t > current_enter_t + 1e-7 {
                            result_intervals.push(Interval {
                                t_enter: current_enter_t,
                                t_exit: b.t,
                                hit_enter: enter_hit,
                                hit_exit: hit,
                            });
                        }
                    }
                }
                in_result = new_in_result;
            }
        }

        result_intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;
    use crate::material::MaterialId;
    use glam::DVec3;

    #[test]
    fn test_csg_intervals() {
        let ray = Ray::new(DVec3::new(-2.0, 0.0, 0.0), DVec3::new(1.0, 0.0, 0.0));

        let union_node = CsgNode {
            left: Box::new(Sphere::new(DVec3::new(0.0, 0.0, 0.0), 1.0, MaterialId(0))),
            right: Box::new(Sphere::new(DVec3::new(1.0, 0.0, 0.0), 1.0, MaterialId(0))),
            op: CsgOp::Union,
        };
        let ui = union_node.intervals(&ray, 0);
        assert_eq!(ui.len(), 1);
        assert!((ui[0].t_enter - 1.0).abs() < 1e-5);
        assert!((ui[0].t_exit - 4.0).abs() < 1e-5);

        let isect_node = CsgNode {
            left: Box::new(Sphere::new(DVec3::new(0.0, 0.0, 0.0), 1.0, MaterialId(0))),
            right: Box::new(Sphere::new(DVec3::new(1.0, 0.0, 0.0), 1.0, MaterialId(0))),
            op: CsgOp::Intersection,
        };
        let ii = isect_node.intervals(&ray, 0);
        assert_eq!(ii.len(), 1);
        assert!((ii[0].t_enter - 2.0).abs() < 1e-5);
        assert!((ii[0].t_exit - 3.0).abs() < 1e-5);

        let diff_node = CsgNode {
            left: Box::new(Sphere::new(DVec3::new(0.0, 0.0, 0.0), 1.0, MaterialId(0))),
            right: Box::new(Sphere::new(DVec3::new(1.0, 0.0, 0.0), 1.0, MaterialId(0))),
            op: CsgOp::Difference,
        };
        let di = diff_node.intervals(&ray, 0);
        assert_eq!(di.len(), 1);
        assert!((di[0].t_enter - 1.0).abs() < 1e-5);
        assert!((di[0].t_exit - 2.0).abs() < 1e-5);
        
        let diff2_node = CsgNode {
            left: Box::new(Sphere::new(DVec3::new(1.0, 0.0, 0.0), 1.0, MaterialId(0))),
            right: Box::new(Sphere::new(DVec3::new(0.0, 0.0, 0.0), 1.0, MaterialId(0))),
            op: CsgOp::Difference,
        };
        let di2 = diff2_node.intervals(&ray, 0);
        assert_eq!(di2.len(), 1);
        assert!((di2[0].t_enter - 3.0).abs() < 1e-5);
        assert!((di2[0].t_exit - 4.0).abs() < 1e-5);
        assert!((di2[0].hit_enter.normal.x - (-1.0)).abs() < 1e-5);
    }
}
