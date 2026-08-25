use glam::DVec3;
use rand::Rng;

use crate::integrator::Integrator;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::hit::Hit;
use crate::material::{TextureRef, Material};
use std::f64::consts::PI;

pub struct PathTracingIntegrator;

impl Integrator for PathTracingIntegrator {
    fn trace_ray(
        &self,
        ray: &Ray,
        depth: u32,
        scene: &Scene,
        max_depth: u32,
        skip_object: Option<usize>,
    ) -> DVec3 {
        // Find closest hit
        let mut best_hit = None;
        for (i, obj) in scene.objects.iter().enumerate() {
            if Some(i) == skip_object {
                continue;
            }
            if let Some(hit) = obj.intersect(ray, i) {
                if best_hit.as_ref().map_or(true, |b: &Hit| hit.t < b.t) {
                    best_hit = Some(hit);
                }
            }
        }

        if depth == 1 {
    // println!("best_hit for depth 1: {:?}", best_hit.as_ref().map(|h| h.t));
}
let hit = match best_hit {
            Some(h) => h,
            None => {
                // Environment Map
                if let Some(env) = &scene.environment_map {
                    return env.sample(ray.direction);
                }
                return DVec3::ZERO; // Black background if no env map
            }
        };

        let material = &scene.materials[hit.material_id.0];
        let obj_index = hit.object_index;

        // Base color
        let base_color = match &material.texture {
            TextureRef::None => material.k_diffuse,
            TextureRef::Checker { color_a, color_b, scale } => {
                let u_cell = (hit.u * scale).floor() as i32;
                let v_cell = (hit.v * scale).floor() as i32;
                if (u_cell + v_cell) % 2 == 0 {
                    *color_a
                } else {
                    *color_b
                }
            }
            TextureRef::Image(idx) => {
                if let Some(tex) = scene.textures.get(*idx) {
                    tex.sample_bilinear(hit.u, hit.v) / 255.0
                } else {
                    material.k_diffuse
                }
            }
        };

        // If Russian Roulette terminates, return emission. 
        // For now, no emissive materials, so just return 0.
        let mut rng = rand::thread_rng();
        let p_survive = if depth > 3 {
            let max_c = base_color.max_element() / 255.0;
            let p = max_c.clamp(0.1, 0.95);
            if rng.gen::<f64>() > p {
                return DVec3::ZERO;
            }
            p
        } else {
            1.0
        };

        // Normalize base color to [0, 1] range for energy calculations
        let albedo = base_color; // Already [0, 1]
        let mut L_out = DVec3::ZERO;

        // Path Tracing Logic:
        // We do a hybrid path tracer:
        // If material is a reflector/refractor, we trace deterministic rays.
        // Otherwise, it's diffuse, we sample the hemisphere.

        // We can use Schlick's approximation for fresnel here for hybrid.
        let mut is_specular = false;
        
        if material.is_reflector || material.is_refractor {
            is_specular = true;
            // Mirror or Glass
            // Re-use legacy logic with hybrid integration:
            if material.is_refractor {
                // Glass (Schlick Fresnel)
                let mut cosi = ray.direction.dot(hit.normal);
                let mut etai = 1.0;
                let mut etat = 1.5; // Fixed index of refraction for legacy compatibility
                let mut n = hit.normal;

                if cosi > 0.0 {
                    std::mem::swap(&mut etai, &mut etat);
                } else {
                    cosi = -cosi;
                }

                let eta = etai / etat;
                let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

                let r0 = ((etai - etat) / (etai + etat)).powi(2);
                let reflectance = r0 + (1.0 - r0) * (1.0 - cosi).powi(5);

                // Reflected ray
                let invec = -ray.direction;
                let rvec = ((n * (n.dot(invec) * 2.0)) - invec).normalize();
                let refray = Ray::new(hit.point + rvec * 1e-4, rvec);
                let refl_color = self.trace_ray(&refray, depth + 1, scene, max_depth, Some(obj_index));

                L_out += refl_color * reflectance;

                // Refracted ray
                if k >= 0.0 {
                    let refract_vec = (ray.direction * eta + n * (eta * cosi - k.sqrt())).normalize();
                    let refract_ray = Ray::new(hit.point + refract_vec * 1e-4, refract_vec);
                    let mut refr_color = self.trace_ray(&refract_ray, depth + 1, scene, max_depth, Some(obj_index));

                    // Beer-Lambert absorption
                    if material.absorption != DVec3::ZERO {
                        let exit_hit = scene
                            .objects
                            .iter()
                            .enumerate()
                            .filter(|(idx, _)| *idx == obj_index)
                            .filter_map(|(idx, obj)| obj.intersect(&refract_ray, idx))
                            .next();
                        let dist = exit_hit.map_or(0.0, |h| h.t);
                        let attenuation = DVec3::new(
                            (-material.absorption.x * dist).exp(),
                            (-material.absorption.y * dist).exp(),
                            (-material.absorption.z * dist).exp(),
                        );
                        refr_color *= attenuation;
                    }

                    L_out += refr_color * (1.0 - reflectance);
                }
            } else if material.is_reflector {
                // Mirror
                let invec = -ray.direction;
                let n = hit.normal;
                let rvec = ((n * (n.dot(invec) * 2.0)) - invec).normalize();
                let refray = Ray::new(hit.point + rvec * 1e-4, rvec);
                let refl_color = self.trace_ray(&refray, depth + 1, scene, max_depth, Some(obj_index));
                L_out += refl_color;
            }
        } else {
            // Diffuse material (Lambertian)
            
            // 1. Direct Light (Next Event Estimation)
            let mut direct_light = DVec3::ZERO;
            for light in &scene.lights {
                let light_pos = if light.radius > 0.0 {
                    // Sample sphere area light
                    let u: f64 = rng.gen();
                    let v: f64 = rng.gen();
                    let theta = u * 2.0 * PI;
                    let phi = (1.0 - 2.0 * v).acos();
                    let dx = phi.sin() * theta.cos();
                    let dy = phi.sin() * theta.sin();
                    let dz = phi.cos();
                    light.origin + DVec3::new(dx, dy, dz) * light.radius
                } else {
                    light.origin
                };

                let l_dir = light_pos - hit.point;
                let dist = l_dir.length();
                let l_dir = l_dir / dist;

                let ndotl = hit.normal.dot(l_dir);
                if ndotl > 0.0 {
                    // Shadow ray
                    let shadow_ray = Ray::new(hit.point + hit.normal * 1e-4, l_dir);
                    let mut shadowed = false;
                    for (j, obj) in scene.objects.iter().enumerate() {
                        if Some(j) == Some(obj_index) { continue; } // ignore self intersection safely
                        if let Some(sh_hit) = obj.intersect(&shadow_ray, j) {
                            if sh_hit.t < dist {
                                shadowed = true;
                                break;
                            }
                        }
                    }

                    if !shadowed {
                        let brdf = albedo / PI;
                        // Light intensity is color (we interpret color as radiant intensity for point light)
                        let l_intensity = light.color;
                        
                        // Solid angle or inverse square falloff.
                        // The Whitted integrator doesn't do inverse square falloff, it just adds the color. 
                        // To match expectations somewhat but stay physically based, let's just do NDotL * Color.
                        // Wait, true path tracing needs inverse square falloff if it's a point light.
                        // But let's just use `light.color * ndotl * brdf` for simplicity, assuming light.color is pre-scaled.
                        direct_light += l_intensity * brdf * ndotl * PI; // Multiply by PI to cancel the /PI in BRDF, preserving intuitive light color.
                    }
                }
            }

            // 2. Indirect Light (Cosine-Weighted Hemisphere Sampling)
            let (u, v) = create_orthonormal_basis(hit.normal);
            let r1: f64 = rng.gen();
            let r2: f64 = rng.gen();

            // Cosine-weighted hemisphere
            let theta = r1.sqrt().acos();
            let phi = 2.0 * PI * r2;
            let dir_local = DVec3::new(theta.sin() * phi.cos(), theta.sin() * phi.sin(), theta.cos());
            
            // Transform to world space
            let bounce_dir = (dir_local.x * u + dir_local.y * v + dir_local.z * hit.normal).normalize();

            let bounce_ray = Ray::new(hit.point + hit.normal * 1e-4, bounce_dir);
            let indirect_radiance = self.trace_ray(&bounce_ray, depth + 1, scene, max_depth, Some(obj_index));
            
            let view_dir = -ray.direction;
            let ndotv = hit.normal.dot(view_dir).max(0.001);
            let l_dir = bounce_dir;
            let ndotl = hit.normal.dot(l_dir).max(0.0);
            let half_vector = (l_dir + view_dir).normalize();

            let mut brdf_weight = albedo;
            if let Some(r) = material.roughness {
                let m = material.metallic.unwrap_or(0.0);
                let f0 = DVec3::splat(0.04).lerp(albedo, m);
                let ndoth = hit.normal.dot(half_vector).max(0.0);
                let vdoth = view_dir.dot(half_vector).max(0.0);

                let ndf = ggx_ndf(ndoth, r);
                let g = ggx_geometry_smith(ndotv, ndotl, r);
                let f = fresnel_schlick(vdoth, f0);

                let nominator = f * ndf * g;
                let denominator = 4.0 * ndotv * ndotl + 0.001;
                let specular = nominator / denominator;

                let ks = f;
                let kd = (DVec3::splat(1.0) - ks) * (1.0 - m);

                let brdf = kd * albedo / PI + specular;
                // Rendering eq: L_in * BRDF * ndotl / PDF
                // PDF = ndotl / PI
                // Weight = BRDF * PI
                brdf_weight = brdf * PI;
            }

            let indirect_light = indirect_radiance * brdf_weight;

            L_out += direct_light + indirect_light;
        }

        // Apply Russian Roulette weighting
                        let result = L_out / p_survive;
        if ray.origin.x == 0.0 && ray.origin.y == 1.0 && ray.origin.z == 3.0 && depth == 1 {
            // println!("pixel direct_light: {:?}, indirect_light: {:?}, L_out: {:?}", direct_light, indirect_light, L_out);
        }
        
        result.clamp(DVec3::ZERO, DVec3::splat(255.0))
    }
}

fn create_orthonormal_basis(n: DVec3) -> (DVec3, DVec3) {
    let w = n.normalize();
    let a = if w.x.abs() > 0.9 {
        DVec3::new(0.0, 1.0, 0.0)
    } else {
        DVec3::new(1.0, 0.0, 0.0)
    };
    let v = w.cross(a).normalize();
    let u = w.cross(v);
    (u, v)
}

fn ggx_ndf(ndoth: f64, roughness: f64) -> f64 {
    let alpha = roughness * roughness;
    let alpha2 = alpha * alpha;
    let ndoth2 = ndoth * ndoth;
    let denom = ndoth2 * (alpha2 - 1.0) + 1.0;
    alpha2 / (PI * denom * denom)
}

fn ggx_geometry_schlick_ggx(ndotv: f64, roughness: f64) -> f64 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    let nom = ndotv;
    let denom = ndotv * (1.0 - k) + k;
    nom / denom
}

fn ggx_geometry_smith(ndotv: f64, ndotl: f64, roughness: f64) -> f64 {
    let ggx2 = ggx_geometry_schlick_ggx(ndotv, roughness);
    let ggx1 = ggx_geometry_schlick_ggx(ndotl, roughness);
    ggx1 * ggx2
}

fn fresnel_schlick(cos_theta: f64, f0: DVec3) -> DVec3 {
    f0 + (DVec3::splat(1.0) - f0) * (1.0 - cos_theta).powi(5)
}
