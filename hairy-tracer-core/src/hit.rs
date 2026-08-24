use crate::material::MaterialId;
use glam::DVec3;

/// Information about a ray–object intersection.
///
/// Contains everything a shader would need later (but no shading math).
#[derive(Debug, Clone, Copy)]
pub struct Hit {
    /// The parameter along the ray where the intersection occurred.
    pub t: f64,
    /// The 3-D world-space point of intersection.
    pub point: DVec3,
    /// The outward-facing unit surface normal at the hit point.
    /// For triangles this is the geometric normal, flipped to face the ray
    /// if the ray hits the back face (matching the Python behavior).
    pub normal: DVec3,
    /// Index of the object that was hit (position in the scene object list).
    pub object_index: usize,
    /// Material identifier for the surface that was hit.
    pub material_id: MaterialId,
    /// Texture U coordinate (0.0 to 1.0)
    pub u: f64,
    /// Texture V coordinate (0.0 to 1.0)
    pub v: f64,
}
