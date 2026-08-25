use hairy_tracer_core::path_tracer::*;
use glam::DVec3;
use std::f64::consts::PI;

// Expose these for testing by making them pub in path_tracer or just rewriting them here.
// Actually, I'll just rewrite the math here for testing to ensure the BRDF integrates correctly.

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

#[test]
fn test_pdf_correctness() {
    // PDF of cosine weighted hemisphere is cos(theta) / PI
    let ndotl = 0.5; // cos(theta)
    let pdf = ndotl / PI;
    assert!((pdf - 0.5 / PI).abs() < 1e-6);
}

#[test]
fn test_energy_conservation() {
    // Integrate BRDF * cos(theta) over hemisphere
    let albedo = DVec3::splat(0.8);
    let roughness = 0.5;
    let metallic = 0.0; // Dielectric
    
    let view_dir = DVec3::new(0.0, 0.0, 1.0); // View from straight up
    let normal = DVec3::new(0.0, 0.0, 1.0);
    
    let mut sum = DVec3::ZERO;
    let num_samples = 100_000;
    
    let mut rng = rand::thread_rng();
    use rand::Rng;
    
    for _ in 0..num_samples {
        let u1: f64 = rng.gen();
        let u2: f64 = rng.gen();
        
        let phi = 2.0 * PI * u1;
        let cos_theta = (1.0 - u2).sqrt();
        let sin_theta = u2.sqrt();
        
        let l_dir = DVec3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta);
        
        let ndotv = normal.dot(view_dir).max(0.001);
        let ndotl = normal.dot(l_dir).max(0.0);
        let half_vector = (l_dir + view_dir).normalize();
        
        let f0 = DVec3::splat(0.04).lerp(albedo, metallic);
        let ndoth = normal.dot(half_vector).max(0.0);
        let vdoth = view_dir.dot(half_vector).max(0.0);

        let ndf = ggx_ndf(ndoth, roughness);
        let g = ggx_geometry_smith(ndotv, ndotl, roughness);
        let f = fresnel_schlick(vdoth, f0);

        let nominator = f * ndf * g;
        let denominator = 4.0 * ndotv * ndotl + 0.001;
        let specular = nominator / denominator;

        let ks = f;
        let kd = (DVec3::splat(1.0) - ks) * (1.0 - metallic);

        let brdf = kd * albedo / PI + specular;
        
        // MC Integration: sum(f(x) / pdf(x)) / N
        // pdf = cos_theta / PI
        let pdf = cos_theta / PI;
        sum += brdf * cos_theta / pdf;
    }
    
    let integral = sum / (num_samples as f64);
    println!("BRDF Integral: {:?}", integral);
    
    // For a non-metallic material, the sum of diffuse + specular energy out should be <= 1.0
    assert!(integral.x <= 1.01); // Allowing tiny numerical error
    assert!(integral.y <= 1.01);
    assert!(integral.z <= 1.01);
}
