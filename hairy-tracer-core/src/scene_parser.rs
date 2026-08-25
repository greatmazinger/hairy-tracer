use glam::DVec3;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::checkered_plane::CheckeredPlane;
use crate::material::{EnvironmentMap, Light, Material, MaterialId, TextureImage, TextureRef};
use crate::mesh::Mesh;
use crate::plane::Plane;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::triangle::Triangle;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SceneJson {
    pub camera: CameraJson,
    #[serde(default)]
    pub materials: HashMap<String, MaterialJson>,
    #[serde(default)]
    pub lights: Vec<LightJson>,
    #[serde(default)]
    pub objects: Vec<ObjectJson>,
    pub environment_map: Option<String>,
    pub integrator: Option<String>, // Path to equirectangular HDR/LDR image
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CameraJson {
    pub origin: [f64; 3],
    pub distance: f64,
    pub vpwidth: f64,
    pub vpheight: f64,
    pub look_at: Option<[f64; 3]>,
    pub up: Option<[f64; 3]>,

    // Sampling parameters
    pub samples_per_pixel: Option<u32>,
    pub aperture: Option<f64>,
    pub focal_distance: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
#[serde(deny_unknown_fields)]
pub struct MaterialJson {
    pub kAmbient: f64,
    pub kDiffuse: [f64; 3],
    pub kSpecular: f64,
    pub nS: f64,
    pub ambientColor: Option<[f64; 3]>,

    // Phase 3: Material quality
    pub ior: Option<f64>,
    #[serde(default)]
    pub use_fresnel: bool,
    pub absorption: Option<[f64; 3]>,
    pub roughness: Option<f64>,
    pub metallic: Option<f64>,
    pub texture: Option<TextureJson>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TextureJson {
    #[serde(rename = "checker")]
    Checker {
        color_a: [f64; 3],
        color_b: [f64; 3],
        #[serde(default = "default_checker_scale")]
        scale: f64,
    },
    #[serde(rename = "image")]
    Image { path: String },
}

fn default_checker_scale() -> f64 {
    1.0
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LightJson {
    pub origin: [f64; 3],
    pub color: [f64; 3],
    pub radius: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ObjectJson {
    #[serde(rename = "type")]
    pub obj_type: String,

    // Fields that might be present
    pub center: Option<[f64; 3]>,
    pub radius: Option<f64>,
    pub normal: Option<[f64; 3]>,
    pub distance: Option<f64>,
    pub v0: Option<[f64; 3]>,
    pub v1: Option<[f64; 3]>,
    pub v2: Option<[f64; 3]>,

    pub file: Option<String>,

    pub material: Option<String>,
    pub material1: Option<String>,
    pub material2: Option<String>,

    #[serde(default)]
    pub is_reflector: bool,
    #[serde(default)]
    pub is_refractor: bool,
}

fn load_texture_image(path: &str) -> Result<TextureImage, String> {
    let img = image::open(path).map_err(|e| format!("Failed to load image {}: {}", path, e))?;
    let rgb = img.to_rgb8();
    let (w, h) = rgb.dimensions();
    let data: Vec<DVec3> = rgb
        .pixels()
        .map(|p| DVec3::new(p[0] as f64, p[1] as f64, p[2] as f64))
        .collect();
    Ok(TextureImage {
        width: w,
        height: h,
        data,
    })
}

pub fn parse_scene_json(json_str: &str) -> Result<(Scene, CameraJson), String> {
    let data: SceneJson = serde_json::from_str(json_str).map_err(|e| e.to_string())?;

    let mut scene = Scene::new();
    if let Some(i) = &data.integrator {
        scene.integrator = i.clone();
    }
    let mut mat_map: HashMap<String, MaterialId> = HashMap::new();

    // 1. Materials
    for (name, mat) in &data.materials {
        let id = MaterialId(scene.materials.len());
        let explicit_ambient = mat.ambientColor.map(DVec3::from_array);

        let texture = match &mat.texture {
            Some(TextureJson::Checker {
                color_a,
                color_b,
                scale,
            }) => TextureRef::Checker {
                color_a: DVec3::from_array(*color_a),
                color_b: DVec3::from_array(*color_b),
                scale: *scale,
            },
            Some(TextureJson::Image { path }) => {
                let tex = load_texture_image(path)?;
                let idx = scene.textures.len();
                scene.textures.push(tex);
                TextureRef::Image(idx)
            }
            None => TextureRef::None,
        };

        scene.materials.push(Material {
            k_ambient: mat.kAmbient,
            k_diffuse: DVec3::from_array(mat.kDiffuse),
            k_specular: mat.kSpecular,
            ns: mat.nS,
            is_reflector: false,
            is_refractor: false,
            ambient_color: explicit_ambient.unwrap_or(DVec3::ZERO),
            has_explicit_ambient: explicit_ambient.is_some(),
            ior: mat.ior.unwrap_or(1.5),
            use_fresnel: mat.use_fresnel,
            absorption: mat.absorption.map(DVec3::from_array).unwrap_or(DVec3::ZERO),
            roughness: mat.roughness,
            metallic: mat.metallic,
            texture,
        });
        mat_map.insert(name.clone(), id);
    }

    // 2. Lights
    for light in &data.lights {
        scene.lights.push(Light {
            origin: DVec3::from_array(light.origin),
            color: DVec3::from_array(light.color),
            radius: light.radius.unwrap_or(0.0),
        });
    }

    // 3. Environment map
    if let Some(ref env_path) = data.environment_map {
        let tex = load_texture_image(env_path)?;
        scene.environment_map = Some(EnvironmentMap { image: tex });
    }

    // 4. Objects
    for obj in &data.objects {
        let t = obj.obj_type.as_str();

        let mut mat_id = MaterialId(0);
        if let Some(ref m) = obj.material {
            if let Some(id) = mat_map.get(m) {
                mat_id = *id;
            } else {
                return Err(format!("Unknown material '{}' referenced by object", m));
            }
        }

        // Override reflection/refraction
        if obj.is_reflector || obj.is_refractor {
            let mut new_mat = scene.materials[mat_id.0].clone();
            new_mat.is_reflector = obj.is_reflector;
            new_mat.is_refractor = obj.is_refractor;
            mat_id = MaterialId(scene.materials.len());
            scene.materials.push(new_mat);
        }

        match t {
            "sphere" => {
                let center = DVec3::from_array(obj.center.unwrap());
                let radius = obj.radius.unwrap();
                let mut mat = scene.materials[mat_id.0].clone();
                if !mat.has_explicit_ambient {
                    mat.ambient_color = DVec3::new(15.0, 75.0, 255.0);
                }
                let new_mat_id = MaterialId(scene.materials.len());
                scene.materials.push(mat);
                scene
                    .objects
                    .push(Box::new(Sphere::new(center, radius, new_mat_id)));
            }
            "plane" => {
                let normal = DVec3::from_array(obj.normal.unwrap());
                let distance = obj.distance.unwrap();
                scene
                    .objects
                    .push(Box::new(Plane::new(normal, distance, mat_id)));
            }
            "checkered_plane" => {
                let normal = DVec3::from_array(obj.normal.unwrap());
                let distance = obj.distance.unwrap();

                let mat1_id = *mat_map.get(obj.material1.as_ref().unwrap()).unwrap();
                let mut mat1 = scene.materials[mat1_id.0].clone();
                if !mat1.has_explicit_ambient {
                    mat1.ambient_color = DVec3::new(10.0, 10.0, 250.0);
                }
                let new_mat1_id = MaterialId(scene.materials.len());
                scene.materials.push(mat1);

                let mat2_id = *mat_map.get(obj.material2.as_ref().unwrap()).unwrap();
                let mut mat2 = scene.materials[mat2_id.0].clone();
                if !mat2.has_explicit_ambient {
                    mat2.ambient_color = DVec3::new(150.0, 10.0, 10.0);
                }
                let new_mat2_id = MaterialId(scene.materials.len());
                scene.materials.push(mat2);

                scene.objects.push(Box::new(CheckeredPlane::new(
                    normal,
                    distance,
                    new_mat1_id,
                    new_mat2_id,
                )));
            }
            "triangle" => {
                let v0 = DVec3::from_array(obj.v0.unwrap());
                let v1 = DVec3::from_array(obj.v1.unwrap());
                let v2 = DVec3::from_array(obj.v2.unwrap());
                let mut mat = scene.materials[mat_id.0].clone();
                if !mat.has_explicit_ambient {
                    mat.ambient_color = DVec3::new(15.0, 75.0, 255.0);
                }
                let new_mat_id = MaterialId(scene.materials.len());
                scene.materials.push(mat);
                scene
                    .objects
                    .push(Box::new(Triangle::new(v0, v1, v2, new_mat_id, 0)));
            }
            "mesh" => {
                let filepath = obj.file.as_ref().unwrap();
                let (tris, tex_coords, norms) = load_obj(filepath)
                    .map_err(|e| format!("Failed to load {}: {}", filepath, e))?;

                let mut mat = scene.materials[mat_id.0].clone();
                if !mat.has_explicit_ambient {
                    mat.ambient_color = DVec3::new(15.0, 75.0, 255.0);
                }
                let new_mat_id = MaterialId(scene.materials.len());
                scene.materials.push(mat);

                let mut tri_objects = Vec::new();
                for (i, (v0, v1, v2)) in tris.iter().enumerate() {
                    let mut tri = Triangle::new(*v0, *v1, *v2, new_mat_id, i);
                    if let Some(ref uvs) = tex_coords {
                        let (uv0, uv1, uv2) = uvs[i];
                        tri.set_uvs(uv0, uv1, uv2);
                    }
                    if let Some(ref ns) = norms {
                        let (n0, n1, n2) = ns[i];
                        tri.set_normals(n0, n1, n2);
                    }
                    tri_objects.push(tri);
                }
                scene
                    .objects
                    .push(Box::new(Mesh::from_triangles(tri_objects, new_mat_id)));
            }
            _ => return Err(format!("Unknown object type: {}", t)),
        }
    }

    Ok((scene, data.camera))
}

/// Load an OBJ file, returning triangles and optional per-triangle UV coordinates.
fn load_obj(
    filepath: &str,
) -> Result<
    (
        Vec<(DVec3, DVec3, DVec3)>,
        Option<Vec<([f64; 2], [f64; 2], [f64; 2])>>,
        Option<Vec<(DVec3, DVec3, DVec3)>>,
    ),
    String,
> {
    let content = fs::read_to_string(filepath).map_err(|e| e.to_string())?;

    let mut vertices = Vec::new();
    let mut tex_coords: Vec<[f64; 2]> = Vec::new();
    let mut normals: Vec<DVec3> = Vec::new();
    let mut triangles = Vec::new();
    let mut tri_indices = Vec::new();
    let mut tri_uvs: Vec<([f64; 2], [f64; 2], [f64; 2])> = Vec::new();
    let mut tri_normals: Vec<(DVec3, DVec3, DVec3)> = Vec::new();
    let mut has_uvs = false;
    let mut has_normals = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let tag = parts.next().unwrap();
        if tag == "v" {
            let x: f64 = parts.next().unwrap().parse().unwrap();
            let y: f64 = parts.next().unwrap().parse().unwrap();
            let z: f64 = parts.next().unwrap().parse().unwrap();
            vertices.push(DVec3::new(x, y, z));
        } else if tag == "vt" {
            let u: f64 = parts.next().unwrap().parse().unwrap();
            let v: f64 = parts.next().unwrap().parse().unwrap();
            tex_coords.push([u, v]);
            has_uvs = true;
        } else if tag == "vn" {
            let nx: f64 = parts.next().unwrap().parse().unwrap();
            let ny: f64 = parts.next().unwrap().parse().unwrap();
            let nz: f64 = parts.next().unwrap().parse().unwrap();
            normals.push(DVec3::new(nx, ny, nz));
            has_normals = true;
        } else if tag == "f" {
            let mut face_v_indices = Vec::new();
            let mut face_vt_indices = Vec::new();
            let mut face_vn_indices = Vec::new();
            for p in parts {
                let mut splits = p.split('/');
                let vi: usize = splits.next().unwrap().parse().unwrap();
                face_v_indices.push(vi - 1);
                if let Some(vt_str) = splits.next() {
                    if !vt_str.is_empty() {
                        if let Ok(vti) = vt_str.parse::<usize>() {
                            face_vt_indices.push(vti - 1);
                        }
                    }
                    if let Some(vn_str) = splits.next() {
                        if !vn_str.is_empty() {
                            if let Ok(vni) = vn_str.parse::<usize>() {
                                face_vn_indices.push(vni - 1);
                            }
                        }
                    }
                }
            }
            // Triangulate n-gon using triangle fan
            for i in 1..(face_v_indices.len() - 1) {
                tri_indices.push((
                    face_v_indices[0],
                    face_v_indices[i],
                    face_v_indices[i + 1],
                ));
                triangles.push((
                    vertices[face_v_indices[0]],
                    vertices[face_v_indices[i]],
                    vertices[face_v_indices[i + 1]],
                ));
                if face_vt_indices.len() == face_v_indices.len() {
                    tri_uvs.push((
                        tex_coords[face_vt_indices[0]],
                        tex_coords[face_vt_indices[i]],
                        tex_coords[face_vt_indices[i + 1]],
                    ));
                }
                if face_vn_indices.len() == face_v_indices.len() {
                    tri_normals.push((
                        normals[face_vn_indices[0]],
                        normals[face_vn_indices[i]],
                        normals[face_vn_indices[i + 1]],
                    ));
                }
            }
        }
    }

    let uvs = if has_uvs && tri_uvs.len() == triangles.len() {
        Some(tri_uvs)
    } else {
        None
    };
    
    let norms = if has_normals && tri_normals.len() == triangles.len() {
        Some(tri_normals)
    } else {
        // Generate angle-weighted vertex normals!
        let mut vertex_normals_accum = vec![glam::DVec3::ZERO; vertices.len()];
        
        for (i, &(i0, i1, i2)) in tri_indices.iter().enumerate() {
            let v0 = vertices[i0];
            let v1 = vertices[i1];
            let v2 = vertices[i2];
            
            let e1 = (v1 - v0).normalize();
            let e2 = (v2 - v0).normalize();
            let e3 = (v2 - v1).normalize();
            let e4 = (v0 - v1).normalize();
            let e5 = (v0 - v2).normalize();
            let e6 = (v1 - v2).normalize();
            
            // Geometric face normal
            let face_normal = (v1 - v0).cross(v2 - v0).normalize();
            
            // Angles at each vertex
            let angle0 = e1.dot(e2).clamp(-1.0, 1.0).acos();
            let angle1 = e3.dot(e4).clamp(-1.0, 1.0).acos();
            let angle2 = e5.dot(e6).clamp(-1.0, 1.0).acos();
            
            vertex_normals_accum[i0] += face_normal * angle0;
            vertex_normals_accum[i1] += face_normal * angle1;
            vertex_normals_accum[i2] += face_normal * angle2;
        }
        
        let mut gen_tri_normals = Vec::with_capacity(triangles.len());
        for &(i0, i1, i2) in &tri_indices {
            let n0 = vertex_normals_accum[i0].normalize();
            let n1 = vertex_normals_accum[i1].normalize();
            let n2 = vertex_normals_accum[i2].normalize();
            
            // Fallback for degenerate normals (e.g. vertices with 0 area faces)
            let fn0 = if n0.is_nan() || n0.length_squared() == 0.0 { glam::DVec3::new(0.0, 1.0, 0.0) } else { n0 };
            let fn1 = if n1.is_nan() || n1.length_squared() == 0.0 { glam::DVec3::new(0.0, 1.0, 0.0) } else { n1 };
            let fn2 = if n2.is_nan() || n2.length_squared() == 0.0 { glam::DVec3::new(0.0, 1.0, 0.0) } else { n2 };
            
            gen_tri_normals.push((fn0, fn1, fn2));
        }
        
        Some(gen_tri_normals)
    };

    Ok((triangles, uvs, norms))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_dynamic_vertex_normals() {
        let obj_data = "\
v 0.0 0.0 1.0
v 1.0 0.0 0.0
v 0.0 1.0 0.0
v -1.0 0.0 0.0
f 1 2 3
f 1 3 4
";
        let filepath = "test_dynamic.obj";
        fs::write(filepath, obj_data).unwrap();

        let (triangles, _uvs, norms) = load_obj(filepath).unwrap();
        fs::remove_file(filepath).unwrap();

        let normals = norms.expect("Normals should be generated");
        assert_eq!(triangles.len(), 2);
        assert_eq!(normals.len(), 2);

        // For the first triangle (1, 2, 3), the tip is v0.
        // We expect the generated normal for v0 (which is the first normal of both triangles)
        // to be (0, 1/sqrt(2), 1/sqrt(2)).
        
        let expected_tip_normal = DVec3::new(0.0, 1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt());
        
        let n0_tri1 = normals[0].0;
        let n0_tri2 = normals[1].0;
        
        // They should be identical since they share the vertex
        assert!((n0_tri1 - n0_tri2).length() < 1e-6);
        assert!((n0_tri1 - expected_tip_normal).length() < 1e-6, "Expected {:?}, got {:?}", expected_tip_normal, n0_tri1);
    }
}
