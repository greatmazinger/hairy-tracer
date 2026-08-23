use crate::hit::Hit;
use crate::intersect::Intersectable;
use crate::ray::Ray;

/// A scene is a collection of objects that can be intersected by a ray.
///
/// This is the top-level entry point for intersection queries: given a ray,
/// find the closest hit across all scene objects (or `None`).
///
/// Matches the Python `World.findIntersectionAndColor` loop that iterates
/// over all objects and picks the nearest hit by comparing distances,
/// except that here we compare `t` values directly (equivalent, since
/// direction is unit-length).
pub struct Scene {
    objects: Vec<Box<dyn Intersectable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Intersectable + 'static) {
        self.objects.push(Box::new(object));
    }

    /// Number of objects in the scene.
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Find the closest intersection of `ray` with any object in the scene.
    ///
    /// Returns `Some(Hit)` for the nearest hit, or `None` if the ray
    /// misses everything. The `Hit::object_index` identifies which object
    /// was hit (its index in insertion order).
    pub fn trace_ray(&self, ray: &Ray) -> Option<Hit> {
        let mut best: Option<Hit> = None;

        for (idx, obj) in self.objects.iter().enumerate() {
            if let Some(hit) = obj.intersect(ray, idx) {
                if best.as_ref().map_or(true, |b| hit.t < b.t) {
                    best = Some(hit);
                }
            }
        }

        best
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::MaterialId;
    use crate::plane::Plane;
    use crate::sphere::Sphere;
    use crate::triangle::Triangle;
    use glam::DVec3;

    #[test]
    fn empty_scene_returns_none() {
        let scene = Scene::new();
        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        assert!(scene.trace_ray(&r).is_none());
    }

    #[test]
    fn closest_hit_across_multiple_objects() {
        let mut scene = Scene::new();

        // Sphere at z=0, radius 1 — near
        scene.add(Sphere::new(
            DVec3::new(0.0, 0.0, 0.0),
            1.0,
            MaterialId(0),
        ));
        // Sphere at z=-5, radius 1 — far
        scene.add(Sphere::new(
            DVec3::new(0.0, 0.0, -5.0),
            1.0,
            MaterialId(1),
        ));

        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        let hit = scene.trace_ray(&r).expect("expected hit");

        // Should hit the near sphere (object_index=0) at t=4
        assert_eq!(hit.object_index, 0);
        assert!((hit.t - 4.0).abs() < 1e-9);
        assert_eq!(hit.material_id, MaterialId(0));
    }

    #[test]
    fn mixed_object_types() {
        let mut scene = Scene::new();

        // Plane at y=-2, facing +Y
        scene.add(Plane::new(
            DVec3::new(0.0, 1.0, 0.0),
            2.0, // normal·P + d = 0 → y + 2 = 0 → y = -2
            MaterialId(10),
        ));

        // Triangle floating at y=1
        scene.add(Triangle::new(
            DVec3::new(-1.0, 1.0, -1.0),
            DVec3::new(1.0, 1.0, -1.0),
            DVec3::new(0.0, 1.0, 1.0),
            MaterialId(20),
        ));

        // Ray from above pointing down
        let r = Ray::new(DVec3::new(0.0, 5.0, 0.0), DVec3::new(0.0, -1.0, 0.0));
        let hit = scene.trace_ray(&r).expect("expected hit");

        // Triangle at y=1 is closer than plane at y=-2
        assert_eq!(hit.object_index, 1);
        assert!((hit.t - 4.0).abs() < 1e-9);
        assert_eq!(hit.material_id, MaterialId(20));
    }
}
