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
}
