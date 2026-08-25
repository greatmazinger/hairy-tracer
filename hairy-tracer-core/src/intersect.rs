use crate::hit::Hit;
use crate::ray::Ray;

/// Trait for objects that can be tested for ray intersection.
///
/// Designed to be `Send + Sync` from the start so that a future rayon-parallel
/// render loop can share scene objects across threads without rework.
pub trait Intersectable: Send + Sync {
    /// Test whether `ray` intersects this object.
    ///
    /// Returns `Some(Hit)` for the nearest intersection with `t > 0`
    /// (subject to per-primitive epsilon rules), or `None` if the ray misses.
    ///
    /// `object_index` is the caller-assigned index of this object in the scene
    /// list, threaded through so that `Hit` can report which object was hit.
    fn intersect(&self, ray: &Ray, object_index: usize) -> Option<Hit>;

    /// Get all intervals inside this object along the ray.
    /// 
    /// Used for Constructive Solid Geometry (CSG) operations. 
    /// Primitives that support CSG must return the entry/exit intervals.
    fn intervals(&self, _ray: &Ray, _object_index: usize) -> Vec<Interval> {
        vec![]
    }
}

/// A segment of a ray that is "inside" a solid primitive, used for CSG.
#[derive(Debug, Clone)]
pub struct Interval {
    pub t_enter: f64,
    pub t_exit: f64,
    pub hit_enter: Hit,
    pub hit_exit: Hit,
}
