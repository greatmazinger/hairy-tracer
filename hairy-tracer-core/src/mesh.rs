use glam::DVec3;

use crate::aabb::Aabb;
use crate::hit::Hit;
use crate::intersect::Intersectable;
use crate::material::MaterialId;
use crate::ray::Ray;
use crate::triangle::Triangle;

/// A triangle mesh with a single AABB acceleration structure.
///
/// The mesh is handed in as pre-built triangles (vertex/index data has
/// already been parsed from OBJ or constructed programmatically). The mesh
/// computes a bounding AABB at construction time and uses it to reject rays
/// that miss the box before doing per-triangle intersection.
///
/// **Future BVH note:** To upgrade from a single AABB to a recursive BVH,
/// replace the flat `triangles: Vec<Triangle>` with a BVH tree structure.
/// Each leaf node would hold a small group of triangles and its own AABB,
/// while internal nodes would hold AABBs covering their children. The
/// `Intersectable::intersect` implementation would recursively traverse the
/// tree, testing the AABB at each node before descending. The external
/// interface (the `Intersectable` trait) stays unchanged.
pub struct Mesh {
    triangles: Vec<Triangle>,
    aabb: Aabb,
    pub material_id: MaterialId,
    /// Whether the AABB was actually tested on the last `intersect` call.
    /// This is only used for testing; the fields are behind `cfg(test)`.
    #[cfg(test)]
    aabb_was_tested: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    aabb_rejected: std::sync::atomic::AtomicBool,
}

impl Mesh {
    /// Create a mesh from pre-built triangles.
    ///
    /// The triangles should already have the correct winding order — this
    /// constructor does not modify them. If the mesh came from OBJ data,
    /// the caller should have already applied triangle-fan triangulation
    /// with the pivot vertex at `face_indices[0]` and winding order
    /// `(0, i, i+1)` for `i in 1..n-1`, matching the Python `load_obj`.
    pub fn from_triangles(triangles: Vec<Triangle>, material_id: MaterialId) -> Self {
        let points = triangles
            .iter()
            .flat_map(|t| [t.v0, t.v1, t.v2]);
        let aabb = Aabb::from_points(points);

        Self {
            triangles,
            aabb,
            material_id,
            #[cfg(test)]
            aabb_was_tested: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            aabb_rejected: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Convenience: build a mesh from flat vertex + index data.
    ///
    /// Each consecutive triple of indices defines one triangle.
    pub fn from_vertices_and_indices(
        vertices: &[DVec3],
        indices: &[usize],
        material_id: MaterialId,
    ) -> Self {
        assert!(
            indices.len() % 3 == 0,
            "index count must be a multiple of 3"
        );
        let triangles: Vec<Triangle> = indices
            .chunks_exact(3)
            .map(|tri| {
                Triangle::new(
                    vertices[tri[0]],
                    vertices[tri[1]],
                    vertices[tri[2]],
                    material_id,
                )
            })
            .collect();

        Self::from_triangles(triangles, material_id)
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray, object_index: usize) -> Option<Hit> {
        // 1. Fast AABB check
        #[cfg(test)]
        self.aabb_was_tested.store(true, std::sync::atomic::Ordering::Relaxed);

        if !self.aabb.intersects(ray) {
            #[cfg(test)]
            self.aabb_rejected.store(true, std::sync::atomic::Ordering::Relaxed);
            return None;
        }

        // 2. Detailed per-triangle check — find the closest hit.
        let mut best: Option<Hit> = None;

        for tri in &self.triangles {
            if let Some(hit) = tri.intersect(ray, object_index) {
                if best.as_ref().map_or(true, |b| hit.t < b.t) {
                    best = Some(Hit {
                        material_id: self.material_id,
                        ..hit
                    });
                }
            }
        }

        best
    }
}

// Compile-time assertion that Mesh is Send + Sync (required for future
// rayon parallelism). AtomicBool is used for test instrumentation fields,
// which is Sync-safe.
const _: () = {
    fn _assert_send_sync<T: Send + Sync>() {}
    fn _check() {
        _assert_send_sync::<Mesh>();
    }
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::MaterialId;
    use crate::ray::Ray;

    const MAT: MaterialId = MaterialId(0);

    /// Build a small mesh: two triangles forming a 2×2 square on the Z=0 plane.
    fn square_mesh() -> Mesh {
        let v0 = DVec3::new(-1.0, -1.0, 0.0);
        let v1 = DVec3::new(1.0, -1.0, 0.0);
        let v2 = DVec3::new(1.0, 1.0, 0.0);
        let v3 = DVec3::new(-1.0, 1.0, 0.0);

        // Triangle fan from v0: (v0,v1,v2) and (v0,v2,v3)
        let tris = vec![
            Triangle::new(v0, v1, v2, MAT),
            Triangle::new(v0, v2, v3, MAT),
        ];

        Mesh::from_triangles(tris, MAT)
    }

    #[test]
    fn mesh_hit_returns_closest_triangle() {
        let mesh = square_mesh();
        // Ray at (0.5, -0.5, 5) pointing down Z — should hit the first triangle
        let r = Ray::new(DVec3::new(0.5, -0.5, 5.0), DVec3::new(0.0, 0.0, -1.0));
        let hit = mesh.intersect(&r, 0).expect("expected hit");

        assert!((hit.t - 5.0).abs() < 1e-9);
        assert!(hit.point.distance(DVec3::new(0.5, -0.5, 0.0)) < 1e-9);
    }

    #[test]
    fn mesh_miss_is_rejected_by_aabb() {
        let mesh = square_mesh();
        // Ray far away — should miss the AABB entirely
        let r = Ray::new(DVec3::new(0.0, 10.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        assert!(mesh.intersect(&r, 0).is_none());
        // Verify the AABB was actually tested and rejected the ray
        assert!(mesh.aabb_was_tested.load(std::sync::atomic::Ordering::Relaxed), "AABB should have been tested");
        assert!(mesh.aabb_rejected.load(std::sync::atomic::Ordering::Relaxed), "AABB should have rejected the ray");
    }

    #[test]
    fn mesh_aabb_accepts_but_triangles_missed() {
        // Build a mesh with a single tiny triangle, but fire a ray that
        // hits the AABB but misses the triangle itself.
        let tri = Triangle::new(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
            MAT,
        );
        let mesh = Mesh::from_triangles(vec![tri], MAT);

        // Ray at (0.9, 0.9, 5) — inside the AABB [0..1, 0..1, 0..0]
        // but outside the triangle (u + v > 1).
        let r = Ray::new(DVec3::new(0.9, 0.9, 5.0), DVec3::new(0.0, 0.0, -1.0));
        assert!(mesh.intersect(&r, 0).is_none());
        assert!(mesh.aabb_was_tested.load(std::sync::atomic::Ordering::Relaxed));
        assert!(!mesh.aabb_rejected.load(std::sync::atomic::Ordering::Relaxed), "AABB should NOT have rejected — ray was inside box");
    }

    #[test]
    fn mesh_returns_closest_among_multiple_triangles() {
        // Two triangles at different Z depths, both facing the ray.
        let near = Triangle::new(
            DVec3::new(-1.0, -1.0, 2.0),
            DVec3::new(1.0, -1.0, 2.0),
            DVec3::new(0.0, 1.0, 2.0),
            MAT,
        );
        let far = Triangle::new(
            DVec3::new(-1.0, -1.0, 0.0),
            DVec3::new(1.0, -1.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
            MAT,
        );
        // Put far triangle first in the list to ensure we're not just
        // returning the first hit.
        let mesh = Mesh::from_triangles(vec![far, near], MAT);

        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        let hit = mesh.intersect(&r, 0).expect("expected hit");

        // Should hit the *near* triangle at z=2, t=3
        assert!(
            (hit.t - 3.0).abs() < 1e-9,
            "should hit the nearer triangle, t should be 3.0, got {}",
            hit.t
        );
        assert!(hit.point.distance(DVec3::new(0.0, 0.0, 2.0)) < 1e-9);
    }
}
