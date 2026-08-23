use glam::DVec3;

/// A ray with an origin and a normalized direction.
///
/// Matches the Python `Ray` class which normalizes direction on construction.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    /// Create a new ray. The direction is normalized internally,
    /// matching the Python Ray.__init__ behavior.
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Compute the point at parameter `t` along the ray.
    #[inline]
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}
