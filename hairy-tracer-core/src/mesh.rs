use crate::aabb::Aabb;
use crate::hit::Hit;
use crate::intersect::Intersectable;
use crate::material::MaterialId;
use crate::ray::Ray;
use crate::triangle::Triangle;
use glam::DVec3;

pub struct BestHitTracker {
    pub max_t: f64,
    pub best_hit: Option<Hit>,
    pub best_index: usize,
}

pub enum BvhNode {
    Leaf {
        aabb: Aabb,
        triangles: Vec<Triangle>,
    },
    Internal {
        aabb: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

impl BvhNode {
    pub fn build(mut triangles: Vec<Triangle>) -> Option<Self> {
        if triangles.is_empty() {
            return None;
        }
        if triangles.len() <= 4 {
            let aabb = Aabb::from_points(triangles.iter().flat_map(|t| [t.v0, t.v1, t.v2]));
            let pad = glam::DVec3::splat(1e-6);
            return Some(BvhNode::Leaf {
                aabb: Aabb {
                    min: aabb.min - pad,
                    max: aabb.max + pad,
                },
                triangles,
            });
        }
        let aabb = Aabb::from_points(triangles.iter().flat_map(|t| [t.v0, t.v1, t.v2]));
        let extent = aabb.max - aabb.min;
        let mut axis = 0;
        if extent.y > extent.x {
            axis = 1;
        }
        if extent.z > extent[axis] {
            axis = 2;
        }
        triangles.sort_by(|a, b| {
            let ca = (a.v0[axis] + a.v1[axis] + a.v2[axis]) / 3.0;
            let cb = (b.v0[axis] + b.v1[axis] + b.v2[axis]) / 3.0;
            ca.partial_cmp(&cb).unwrap_or(std::cmp::Ordering::Equal)
        });
        let mid = triangles.len() / 2;
        let right_triangles = triangles.split_off(mid);
        let left = Box::new(BvhNode::build(triangles).unwrap());
        let right = Box::new(BvhNode::build(right_triangles).unwrap());
        let pad = glam::DVec3::splat(1e-6);
        Some(BvhNode::Internal {
            aabb: Aabb {
                min: aabb.min - pad,
                max: aabb.max + pad,
            },
            left,
            right,
        })
    }

    pub fn intersect(&self, ray: &Ray, object_index: usize, tracker: &mut BestHitTracker) {
        match self {
            BvhNode::Leaf { aabb, triangles } => {
                if !aabb.intersects_with_max(ray, tracker.max_t + 1e-6) {
                    return;
                }
                for tri in triangles {
                    if let Some(hit) = tri.intersect(ray, object_index) {
                        if hit.t < tracker.max_t
                            || (hit.t == tracker.max_t && tri.original_index < tracker.best_index)
                        {
                            tracker.max_t = hit.t;
                            tracker.best_hit = Some(hit);
                            tracker.best_index = tri.original_index;
                        }
                    }
                }
            }
            BvhNode::Internal { aabb, left, right } => {
                if !aabb.intersects_with_max(ray, tracker.max_t + 1e-6) {
                    return;
                }

                let (t_left, hit_l) = left.aabb().hit_distance(ray, tracker.max_t + 1e-6);
                let (t_right, hit_r) = right.aabb().hit_distance(ray, tracker.max_t + 1e-6);

                let mut first = None;
                let mut second = None;

                match (hit_l, hit_r) {
                    (true, true) => {
                        if t_left < t_right {
                            first = Some(left.as_ref());
                            second = Some(right.as_ref());
                        } else {
                            first = Some(right.as_ref());
                            second = Some(left.as_ref());
                        }
                    }
                    (true, false) => first = Some(left.as_ref()),
                    (false, true) => first = Some(right.as_ref()),
                    (false, false) => return,
                }

                if let Some(node) = first {
                    node.intersect(ray, object_index, tracker);
                }
                if let Some(node) = second {
                    if node.aabb().intersects_with_max(ray, tracker.max_t + 1e-6) {
                        node.intersect(ray, object_index, tracker);
                    }
                }
            }
        }
    }

    pub fn aabb(&self) -> &Aabb {
        match self {
            BvhNode::Leaf { aabb, .. } => aabb,
            BvhNode::Internal { aabb, .. } => aabb,
        }
    }
}

pub struct Mesh {
    pub material_id: MaterialId,
    pub root: Option<BvhNode>,
}

impl Mesh {
    pub fn from_triangles(triangles: Vec<Triangle>, material_id: MaterialId) -> Self {
        Self {
            material_id,
            root: BvhNode::build(triangles),
        }
    }

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
            .enumerate()
            .map(|(i, tri)| {
                Triangle::new(
                    vertices[tri[0]],
                    vertices[tri[1]],
                    vertices[tri[2]],
                    material_id,
                    i,
                )
            })
            .collect();
        Self::from_triangles(triangles, material_id)
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray, object_index: usize) -> Option<Hit> {
        let root = self.root.as_ref()?;
        let mut tracker = BestHitTracker {
            max_t: f64::MAX,
            best_hit: None,
            best_index: usize::MAX,
        };
        root.intersect(ray, object_index, &mut tracker);
        if let Some(ref mut h) = tracker.best_hit {
            h.material_id = self.material_id;
        }
        tracker.best_hit
    }
}
