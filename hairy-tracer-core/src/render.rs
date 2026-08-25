use crate::integrator::Integrator;
use crate::hit::Hit;
use crate::material::TextureRef;
use crate::ray::Ray;
use crate::scene::Scene;
use glam::DVec3;
use rand::Rng;
use rayon::prelude::*;

/// Reflect vector `invec` around `normal`.
/// Expects `invec` pointing AWAY from the surface (i.e. -ray.direction).
/// Returns vector pointing AWAY from the surface.
fn calc_reflection_vector(invec: DVec3, normal: DVec3) -> DVec3 {
    let n = normal;
    let l = invec;
    (n * (n.dot(l) * 2.0)) - l
}

/// Refract vector `invec` around `normal` given index of refraction `ior`.
/// Expects `invec` pointing TOWARD the surface (i.e. ray.direction).
fn calc_refraction_vector(invec: DVec3, normal: DVec3, ior: f64) -> Option<DVec3> {
    let mut cosi = invec.dot(normal);
    let mut etai = 1.0;
    let mut etat = ior;
    let mut n = normal;

    if cosi < 0.0 {
        cosi = -cosi;
    } else {
        std::mem::swap(&mut etai, &mut etat);
        n = -normal;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        None
    } else {
        Some(invec * eta + n * (eta * cosi - k.sqrt()))
    }
}

/// Schlick's approximation for Fresnel reflectance.
/// Returns the fraction of light that is reflected (0..1).
fn fresnel_schlick(cos_theta: f64, ior: f64) -> f64 {
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

/// Sample the material's diffuse color, applying texture if present.
fn sample_diffuse(material: &crate::material::Material, hit: &Hit, scene: &Scene) -> DVec3 {
    match &material.texture {
        TextureRef::None => material.k_diffuse,
        TextureRef::Checker {
            color_a,
            color_b,
            scale,
        } => {
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
                // Image texture replaces diffuse: sample returns [0..255], normalize to [0..1] like k_diffuse
                tex.sample_bilinear(hit.u, hit.v) / 255.0
            } else {
                material.k_diffuse
            }
        }
    }
}

pub fn does_intersect(
    scene: &Scene,
    origin: DVec3,
    direction: DVec3,
    skip_object: Option<usize>,
) -> bool {
    let ray = Ray::new(origin, direction);
    for (idx, obj) in scene.objects.iter().enumerate() {
        
        if let Some(hit) = obj.intersect(&ray, idx) {
            if hit.t > 0.0 {
                return true;
            }
        }
    }
    false
}

pub struct WhittedIntegrator;

impl Integrator for WhittedIntegrator {
    fn trace_ray(
        &self,
        ray: &Ray,
        depth: u32,
        scene: &Scene,
        max_depth: u32,
        skip_object: Option<usize>,
    ) -> DVec3 {
    if depth > max_depth {
        return DVec3::ZERO;
    }

    // Manual closest hit to mimic world.findIntersectionAndColor skipping srcobject
    let mut best: Option<Hit> = None;
    for (idx, obj) in scene.objects.iter().enumerate() {
        
        if let Some(hit) = obj.intersect(ray, idx) {
            if best.as_ref().map_or(true, |b| hit.t < b.t) {
                best = Some(hit);
            }
        }
    }

    let hit = match best {
        Some(h) => h,
        None => {
            // Miss: sample environment map or return black
            return if let Some(ref env) = scene.environment_map {
                env.sample(ray.direction)
            } else {
                DVec3::ZERO
            };
        }
    };

    let obj_index = hit.object_index;
    let material = &scene.materials[hit.material_id.0];

    // Sample diffuse color (solid or textured)
    let diffuse_color = sample_diffuse(material, &hit, scene);

    // Base color from lights
    let mut base_color = DVec3::ZERO;
    let viewvec = -ray.direction; // points to camera

    for light in &scene.lights {
        let jittered_light_origin = if light.radius > 0.0 {
            let mut rng = rand::thread_rng();
            let mut pt;
            loop {
                pt = DVec3::new(
                    rng.gen_range(-1.0..=1.0),
                    rng.gen_range(-1.0..=1.0),
                    rng.gen_range(-1.0..=1.0),
                );
                if pt.length_squared() <= 1.0 {
                    break;
                }
            }
            light.origin + pt * light.radius
        } else {
            light.origin
        };

        let lightvec = (jittered_light_origin - hit.point).normalize();

        let is_shadowed = does_intersect(scene, hit.point, lightvec, None);

        let mut color_for_light = material.ambient_color * material.k_ambient;

        if !is_shadowed {
            let ldv = lightvec.dot(hit.normal).max(0.0);

            let diffuse = diffuse_color * ldv * light.color;
            let diffuse = diffuse.clamp(DVec3::ZERO, DVec3::splat(255.0));

            let specular = if hit.normal.dot(viewvec) < 0.0 {
                DVec3::ZERO
            } else {
                let h = (lightvec + viewvec).normalize();
                let spec_val = hit.normal.dot(h).max(0.0).powf(material.ns);
                light.color * (material.k_specular * spec_val)
            };
            let specular = specular.clamp(DVec3::ZERO, DVec3::splat(255.0));

            color_for_light += diffuse + specular;
        }

        // Clamp per light
        color_for_light = color_for_light.clamp(DVec3::ZERO, DVec3::splat(255.0));
        base_color += color_for_light;
    }

    let mut total_color = base_color;

    // Reflection and Refraction (with optional Fresnel weighting)
    if material.use_fresnel && (material.is_reflector || material.is_refractor) {
        // Fresnel-Schlick: compute reflectance fraction
        let cos_theta = (-ray.direction).dot(hit.normal).abs();
        let reflectance = fresnel_schlick(cos_theta, material.ior);

        // Reflection component
        if material.is_reflector {
            let invec = -ray.direction;
            let rvec = calc_reflection_vector(invec, hit.normal).normalize();
            let refray = Ray::new(hit.point, rvec);
            let refl_color =
                self.trace_ray(&refray, depth + 1, scene, max_depth, None);
            total_color += refl_color * reflectance;
        }

        // Refraction component
        if material.is_refractor {
            if let Some(refract_vec) =
                calc_refraction_vector(ray.direction, hit.normal, material.ior)
            {
                let refract_vec = refract_vec.normalize();
                let offset_point = hit.point + refract_vec * 0.001;
                let refract_ray = Ray::new(offset_point, refract_vec);
                let mut refr_color =
                    self.trace_ray(&refract_ray, depth + 1, scene, max_depth, None);

                // Beer-Lambert absorption
                if material.absorption != DVec3::ZERO {
                    // Find exit point to compute distance through medium
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

                total_color += refr_color * (1.0 - reflectance);
            }
        }
    } else {
        // Legacy behavior: additive, no Fresnel weighting
        if material.is_reflector {
            let invec = -ray.direction;
            let rvec = calc_reflection_vector(invec, hit.normal).normalize();
            let refray = Ray::new(hit.point, rvec);
            let refl_color =
                self.trace_ray(&refray, depth + 1, scene, max_depth, None);
            total_color += refl_color;
        }

        if material.is_refractor {
            if let Some(refract_vec) = calc_refraction_vector(ray.direction, hit.normal, 1.5) {
                let refract_vec = refract_vec.normalize();
                let offset_point = hit.point + refract_vec * 0.001;
                let refract_ray = Ray::new(offset_point, refract_vec);
                let refr_color =
                    self.trace_ray(&refract_ray, depth + 1, scene, max_depth, None);
                total_color += refr_color;
            }
        }
    }

    // Final clamp
    total_color.clamp(DVec3::ZERO, DVec3::splat(255.0))
}
}

pub fn render_image_serial(integrator: &dyn Integrator,
    scene: &Scene,
    cam_origin: DVec3,
    look_at: DVec3,
    up: DVec3,
    vpwidth: f64,
    vpheight: f64,
    width: usize,
    height: usize,
    max_depth: u32,
    samples_per_pixel: u32,
    aperture: f64,
    focal_distance: f64,
) -> Vec<u8> {
    let mut pixels = vec![0u8; width * height * 3];

    let w = cam_origin - look_at;
    let w_norm = w.length();
    let w = if w_norm == 0.0 {
        DVec3::new(0.0, 0.0, 1.0)
    } else {
        w / w_norm
    };

    let u = up.cross(w);
    let u_norm = u.length();
    let u = if u_norm == 0.0 {
        DVec3::new(1.0, 0.0, 0.0)
    } else {
        u / u_norm
    };

    let v = w.cross(u);

    let xd = vpwidth / width as f64;
    let yd = vpheight / height as f64;
    let xright = vpwidth / 2.0;
    let yright = vpheight / 2.0;

    for ytmp in 0..height {
        for xtmp in 0..width {
            let mut color_sum = DVec3::ZERO;

            for _ in 0..samples_per_pixel {
                let (dx, dy) = if samples_per_pixel == 1 {
                    (1.0, 0.0) // Legacy exact pixel mapping
                } else {
                    let mut rng = rand::thread_rng();
                    (rng.gen::<f64>(), -rng.gen::<f64>()) // Anti-aliasing jitter
                };

                let x_scalar = -xright + (xtmp as f64 + dx) * xd;
                let y_scalar = -yright + (ytmp as f64 + dy) * yd;

                let target_point = look_at + x_scalar * u + y_scalar * v;
                let ray_dir = target_point - cam_origin;

                let (ray_origin, final_dir) = if aperture > 0.0 {
                    let mut rng = rand::thread_rng();
                    let mut pt;
                    loop {
                        pt = DVec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);
                        if pt.length_squared() <= 1.0 {
                            break;
                        }
                    }

                    let offset = u * (pt.x * aperture) + v * (pt.y * aperture);
                    let focal_point = cam_origin + ray_dir * (focal_distance / w_norm);
                    let new_origin = cam_origin + offset;
                    (new_origin, focal_point - new_origin)
                } else {
                    (cam_origin, ray_dir)
                };

                let ray = Ray::new(ray_origin, final_dir);
                color_sum += integrator.trace_ray(&ray, 1, scene, max_depth, None);
            }

            let color = color_sum / (samples_per_pixel as f64);

            let idx = (ytmp * width + xtmp) * 3;
            pixels[idx] = color.x.clamp(0.0, 255.0) as u8;
            pixels[idx + 1] = color.y.clamp(0.0, 255.0) as u8;
            pixels[idx + 2] = color.z.clamp(0.0, 255.0) as u8;
        }
    }

    pixels
}

pub fn render_image_parallel(integrator: &dyn Integrator,
    scene: &Scene,
    cam_origin: DVec3,
    look_at: DVec3,
    up: DVec3,
    vpwidth: f64,
    vpheight: f64,
    width: usize,
    height: usize,
    max_depth: u32,
    samples_per_pixel: u32,
    aperture: f64,
    focal_distance: f64,
) -> Vec<u8> {
    let mut pixels = vec![0u8; width * height * 3];

    let w = cam_origin - look_at;
    let w_norm = w.length();
    let w = if w_norm == 0.0 {
        DVec3::new(0.0, 0.0, 1.0)
    } else {
        w / w_norm
    };

    let u = up.cross(w);
    let u_norm = u.length();
    let u = if u_norm == 0.0 {
        DVec3::new(1.0, 0.0, 0.0)
    } else {
        u / u_norm
    };

    let v = w.cross(u);

    let xd = vpwidth / width as f64;
    let yd = vpheight / height as f64;
    let xright = vpwidth / 2.0;
    let yright = vpheight / 2.0;

    // Process row by row in parallel
    pixels
        .par_chunks_exact_mut(width * 3)
        .enumerate()
        .for_each(|(ytmp, row)| {
            for xtmp in 0..width {
                let mut color_sum = DVec3::ZERO;

                for _ in 0..samples_per_pixel {
                    let (dx, dy) = if samples_per_pixel == 1 {
                        (1.0, 0.0) // Legacy exact pixel mapping
                    } else {
                        let mut rng = rand::thread_rng();
                        (rng.gen::<f64>(), -rng.gen::<f64>()) // Anti-aliasing jitter
                    };

                    let x_scalar = -xright + (xtmp as f64 + dx) * xd;
                    let y_scalar = -yright + (ytmp as f64 + dy) * yd;

                    let target_point = look_at + x_scalar * u + y_scalar * v;
                    let ray_dir = target_point - cam_origin;

                    let (ray_origin, final_dir) = if aperture > 0.0 {
                        let mut rng = rand::thread_rng();
                        let mut pt;
                        loop {
                            pt = DVec3::new(
                                rng.gen_range(-1.0..=1.0),
                                rng.gen_range(-1.0..=1.0),
                                0.0,
                            );
                            if pt.length_squared() <= 1.0 {
                                break;
                            }
                        }

                        let offset = u * (pt.x * aperture) + v * (pt.y * aperture);
                        let focal_point = cam_origin + ray_dir * (focal_distance / w_norm);
                        let new_origin = cam_origin + offset;
                        (new_origin, focal_point - new_origin)
                    } else {
                        (cam_origin, ray_dir)
                    };

                    let ray = Ray::new(ray_origin, final_dir);
                    color_sum += integrator.trace_ray(&ray, 1, scene, max_depth, None);
                }

                let color = color_sum / (samples_per_pixel as f64);

                let idx = xtmp * 3;
                row[idx] = color.x.clamp(0.0, 255.0) as u8;
                row[idx + 1] = color.y.clamp(0.0, 255.0) as u8;
                row[idx + 2] = color.z.clamp(0.0, 255.0) as u8;
            }
        });

    pixels
}
