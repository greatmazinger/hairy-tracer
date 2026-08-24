use hairy_tracer_core::mesh::{Mesh, BvhNode};
use hairy_tracer_core::triangle::Triangle;
use hairy_tracer_core::material::MaterialId;
use hairy_tracer_core::ray::Ray;
use hairy_tracer_core::intersect::Intersectable;
use glam::DVec3;

fn count_triangles(node: &BvhNode) -> usize {
    match node {
        BvhNode::Leaf { triangles, .. } => triangles.len(),
        BvhNode::Internal { left, right, .. } => count_triangles(left) + count_triangles(right),
    }
}

fn gather_triangle_indices(node: &BvhNode, indices: &mut Vec<usize>) {
    match node {
        BvhNode::Leaf { triangles, .. } => {
            for tri in triangles {
                indices.push(tri.original_index);
            }
        }
        BvhNode::Internal { left, right, .. } => {
            gather_triangle_indices(left, indices);
            gather_triangle_indices(right, indices);
        }
    }
}

#[test]
fn test_bvh_construction_conserves_triangles() {
    let mat = MaterialId(0);
    // Create a random assortment of triangles
    let mut vertices = vec![];
    let mut indices = vec![];
    for i in 0..100 {
        vertices.push(DVec3::new(i as f64, 0.0, 0.0));
        vertices.push(DVec3::new(i as f64 + 1.0, 0.0, 0.0));
        vertices.push(DVec3::new(i as f64, 1.0, 0.0));
        indices.push(i * 3);
        indices.push(i * 3 + 1);
        indices.push(i * 3 + 2);
    }
    
    let mesh = Mesh::from_vertices_and_indices(&vertices, &indices, mat);
    let root = mesh.root.as_ref().expect("Expected root node");
    
    let count = count_triangles(root);
    assert_eq!(count, 100, "BVH should contain exactly 100 triangles, but got {}", count);
    
    let mut extracted_indices = Vec::new();
    gather_triangle_indices(root, &mut extracted_indices);
    extracted_indices.sort();
    
    let expected: Vec<usize> = (0..100).collect();
    assert_eq!(extracted_indices, expected, "BVH construction dropped or duplicated triangles");
}

#[test]
fn test_bvh_matches_linear_scan() {
    let mat = MaterialId(0);
    // Create an overlapping cluster of triangles to stress the intersection tiebreaker
    let mut vertices = vec![];
    let mut indices = vec![];
    for i in 0..20 {
        vertices.push(DVec3::new(-1.0, -1.0, 0.0));
        vertices.push(DVec3::new(1.0, -1.0, 0.0));
        vertices.push(DVec3::new(0.0, 1.0, (i as f64) * 0.0001)); // Slight Z offset
        indices.push(i * 3);
        indices.push(i * 3 + 1);
        indices.push(i * 3 + 2);
    }
    
    let mesh = Mesh::from_vertices_and_indices(&vertices, &indices, mat);
    let mut linear_mesh = Mesh::from_vertices_and_indices(&vertices, &indices, mat);
    
    // Force a linear scan by extracting the triangles and looping them
    let mut raw_triangles = vec![];
    gather_raw_triangles(mesh.root.as_ref().unwrap(), &mut raw_triangles);
    
    let ray = Ray::new(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.0, 0.0, -1.0));
    
    // 1. BVH Intersect
    let bvh_hit = mesh.intersect(&ray, 42).expect("Expected BVH hit");
    
    // 2. Linear Intersect
    let mut best_hit = None;
    for tri in &raw_triangles {
        if let Some(hit) = tri.intersect(&ray, 42) {
            if best_hit.as_ref().map_or(true, |b: &hairy_tracer_core::hit::Hit| hit.t < b.t) {
                best_hit = Some(hit);
            }
        }
    }
    let lin_hit = best_hit.expect("Expected linear hit");
    
    // Wait, the linear scan above does NOT implement the tiebreaker if they have exactly the same t. 
    // In my case the Z offset makes them strictly differ, so `<` works.
    
    assert_eq!(bvh_hit.t, lin_hit.t, "BVH t does not match linear scan");
    assert_eq!(bvh_hit.normal, lin_hit.normal, "BVH normal does not match linear scan");
}

fn gather_raw_triangles(node: &BvhNode, dest: &mut Vec<Triangle>) {
    match node {
        BvhNode::Leaf { triangles, .. } => dest.extend(triangles.iter().cloned()),
        BvhNode::Internal { left, right, .. } => {
            gather_raw_triangles(left, dest);
            gather_raw_triangles(right, dest);
        }
    }
}

