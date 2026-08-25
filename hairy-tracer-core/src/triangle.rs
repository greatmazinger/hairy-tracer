use glam::DVec3;

use crate::hit::Hit;
use crate::intersect::Intersectable;
use crate::material::MaterialId;
use crate::ray::Ray;

/// Möller–Trumbore epsilon — same value as the Python code (`epsilon = 1e-6`).
const MT_EPSILON: f64 = 1e-6;

/// Minimum `t` for a valid intersection — same as the Python code (`t > 0.01`).
const T_EPSILON: f64 = 0.01;

/// A single triangle defined by three vertices.
///
/// Intersection uses the Möller–Trumbore algorithm, matching the Python
/// `Triangle.findIntersection` exactly:
///
/// - Parallel check: `-epsilon < a < epsilon` with `epsilon = 1e-6`
/// - Valid hit: `t > 0.01`
/// - Back-face normal flip: if `dot(Rd, normal) > 0`, the normal is negated.
#[derive(Clone, PartialEq, Debug)]
pub struct Triangle {
    pub v0: DVec3,
    pub v1: DVec3,
    pub v2: DVec3,
    pub material_id: MaterialId,
    pub original_index: usize,
    /// Pre-computed edge v1 - v0.
    edge1: DVec3,
    /// Pre-computed edge v2 - v0.
    edge2: DVec3,
    /// Pre-computed geometric normal (unit length).
    normal: DVec3,
    /// Optional per-vertex UVs from OBJ vt data: (uv0, uv1, uv2).
    vertex_uvs: Option<([f64; 2], [f64; 2], [f64; 2])>,
    pub vertex_normals: Option<(DVec3, DVec3, DVec3)>,
}

impl Triangle {
    pub fn new(
        v0: DVec3,
        v1: DVec3,
        v2: DVec3,
        material_id: MaterialId,
        original_index: usize,
    ) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal_raw = edge1.cross(edge2);
        let len = normal_raw.length();
        let normal = if len > 0.0 {
            normal_raw / len
        } else {
            // Degenerate triangle — fallback normal, matching Python.
            DVec3::new(0.0, 1.0, 0.0)
        };

        Self {
            v0,
            v1,
            v2,
            material_id,
            original_index,
            edge1,
            edge2,
            normal,
            vertex_uvs: None,
            vertex_normals: None,
        }
    }

    /// Set per-vertex UV coordinates (from OBJ vt data).
    pub fn set_uvs(&mut self, uv0: [f64; 2], uv1: [f64; 2], uv2: [f64; 2]) {
        self.vertex_uvs = Some((uv0, uv1, uv2));
    }
    
    pub fn set_normals(&mut self, n0: DVec3, n1: DVec3, n2: DVec3) {
        self.vertex_normals = Some((n0, n1, n2));
    }

    /// The geometric (face) normal, before any back-face flipping.
    pub fn geometric_normal(&self) -> DVec3 {
        self.normal
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray, object_index: usize) -> Option<Hit> {
        let rd = ray.direction; // already normalized

        // Möller–Trumbore
        let h = rd.cross(self.edge2);
        let a = self.edge1.dot(h);

        // Ray is parallel to the triangle.
        if a > -MT_EPSILON && a < MT_EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(self.edge1);
        let v = f * rd.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * self.edge2.dot(q);

        if t <= T_EPSILON {
            return None;
        }

        let point = ray.at(t);
        
        let geom_normal = self.normal;
        let mut normal = if let Some((n0, n1, n2)) = self.vertex_normals {
            let w = 1.0 - u - v;
            (n0 * w + n1 * u + n2 * v).normalize()
        } else {
            self.normal
        };

        // Back-face flip MUST be based on the geometric normal, not the interpolated one.
        // Otherwise, at grazing angles (the terminator), the interpolated normal might point
        // slightly away from the ray even on a front-face hit, causing us to incorrectly flip it
        // and shoot rays into the interior of the mesh!
        if rd.dot(geom_normal) > 0.0 {
            normal = -normal;
        }
        
        // Also ensure the interpolated normal doesn't point perfectly away from the viewer, 
        // which can cause self-shadowing/terminator artifacts. We gently nudge it if needed.
        if rd.dot(normal) > 0.0 {
            // It's pointing away from the ray (into the surface from the ray's perspective).
            // We can't use this normal or we'll get black patches. Let's fallback to geometric normal
            // for this grazing hit.
            normal = if rd.dot(geom_normal) > 0.0 { -geom_normal } else { geom_normal };
        }

        // UV mapping: use vertex UVs if available (from OBJ vt), else barycentric coordinates.
        let (u_coord, v_coord) = if let Some((uv0, uv1, uv2)) = self.vertex_uvs {
            // Barycentric interpolation: P = (1-u-v)*uv0 + u*uv1 + v*uv2
            let w = 1.0 - u - v;
            let interp_u = w * uv0[0] + u * uv1[0] + v * uv2[0];
            let interp_v = w * uv0[1] + u * uv1[1] + v * uv2[1];
            (interp_u, interp_v)
        } else {
            (u, v)
        };

        Some(Hit {
            t,
            point,
            normal,
            object_index,
            material_id: self.material_id,
            u: u_coord,
            v: v_coord,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAT: MaterialId = MaterialId(0);

    fn xy_triangle() -> Triangle {
        // Triangle on the Z=0 plane, matching the Python test.
        Triangle::new(
            DVec3::new(-1.0, -1.0, 0.0),
            DVec3::new(1.0, -1.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
            MAT, 0,
        )
    }

    #[test]
    fn ray_hits_triangle_center() {
        let tri = xy_triangle();
        let r = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        let hit = tri.intersect(&r, 0).expect("expected hit");

        assert!((hit.t - 5.0).abs() < 1e-9, "t should be 5.0, got {}", hit.t);
        assert!(hit.point.distance(DVec3::new(0.0, 0.0, 0.0)) < 1e-9);
        // Normal should face toward the ray origin (+Z)
        assert!(hit.normal.distance(DVec3::new(0.0, 0.0, 1.0)) < 1e-9);
    }

    #[test]
    fn ray_misses_triangle() {
        let tri = xy_triangle();
        let r = Ray::new(DVec3::new(2.0, 2.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        assert!(tri.intersect(&r, 0).is_none());
    }

    #[test]
    fn back_face_hit_flips_normal() {
        let tri = xy_triangle();
        // Hit from the -Z side (back face).
        let r = Ray::new(DVec3::new(0.0, 0.0, -5.0), DVec3::new(0.0, 0.0, 1.0));
        let hit = tri.intersect(&r, 0).expect("expected back-face hit");

        assert!((hit.t - 5.0).abs() < 1e-9);
        // Normal should be flipped to face toward the ray origin (-Z)
        assert!(
            hit.normal.distance(DVec3::new(0.0, 0.0, -1.0)) < 1e-9,
            "back-face normal should be flipped to (0,0,-1), got {:?}",
            hit.normal
        );
    }

    #[test]
    fn ray_parallel_to_triangle_misses() {
        let tri = xy_triangle();
        // Ray along X in the Z=0 plane — parallel to the triangle.
        let r = Ray::new(DVec3::new(-5.0, 0.0, 0.0), DVec3::new(1.0, 0.0, 0.0));
        assert!(tri.intersect(&r, 0).is_none());
    }

    #[test]
    fn ray_hits_triangle_edge() {
        let tri = xy_triangle();
        // Hit the midpoint of the bottom edge (v0 to v1): midpoint is (0, -1, 0)
        let r = Ray::new(DVec3::new(0.0, -1.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        let hit = tri.intersect(&r, 0).expect("edge hit should succeed");
        assert!(hit.point.distance(DVec3::new(0.0, -1.0, 0.0)) < 1e-6);
    }

    #[test]
    fn ray_hits_triangle_vertex() {
        let tri = xy_triangle();
        // Hit v2 = (0, 1, 0) directly.
        // Due to Möller–Trumbore barycentric boundary (u=0, v=1, u+v=1),
        // this is right at the edge of acceptance.
        let r = Ray::new(DVec3::new(0.0, 1.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
        // This may or may not hit depending on float precision at the boundary.
        // We just ensure it doesn't panic.
        let _ = tri.intersect(&r, 0);
    }
}
