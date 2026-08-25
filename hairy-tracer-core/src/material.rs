use glam::DVec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub usize);

#[derive(Debug, Clone)]
pub struct Material {
    pub k_ambient: f64,
    pub k_diffuse: DVec3,
    pub k_specular: f64,
    pub ns: f64,
    pub is_reflector: bool,
    pub is_refractor: bool,
    pub ambient_color: DVec3,
    pub has_explicit_ambient: bool,

    // Phase 3: Material quality
    pub ior: f64, // Index of refraction (default 1.5, matching current hardcoded value)
    pub use_fresnel: bool, // Fresnel-Schlick weighting (default false — preserves legacy additive blend)
    pub absorption: DVec3,
    pub roughness: Option<f64>,
    pub metallic: Option<f64>, // Beer-Lambert absorption coefficient per channel (default ZERO — clear glass)
    pub texture: TextureRef, // Texture reference (default None — solid color)
}

/// A reference to a texture, either procedural or image-based.
#[derive(Debug, Clone)]
pub enum TextureRef {
    None,
    Checker {
        color_a: DVec3,
        color_b: DVec3,
        scale: f64,
    },
    Image(usize), // Index into Scene::textures
}

/// A loaded texture image, stored as f64 RGB [0..255] for direct use in the renderer.
#[derive(Debug, Clone)]
pub struct TextureImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<DVec3>, // row-major, width * height entries
}

impl TextureImage {
    /// Bilinear sample at UV coordinates in [0, 1].
    pub fn sample_bilinear(&self, u: f64, v: f64) -> DVec3 {
        let u = u.fract();
        let v = v.fract();
        let u = if u < 0.0 { u + 1.0 } else { u };
        let v = if v < 0.0 { v + 1.0 } else { v };

        let x = u * (self.width as f64) - 0.5;
        let y = v * (self.height as f64) - 0.5;

        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        let fx = x - x.floor();
        let fy = y - y.floor();

        let px = |xi: i32, yi: i32| -> DVec3 {
            let xi = ((xi % self.width as i32) + self.width as i32) as u32 % self.width;
            let yi = ((yi % self.height as i32) + self.height as i32) as u32 % self.height;
            self.data[(yi * self.width + xi) as usize]
        };

        let c00 = px(x0, y0);
        let c10 = px(x1, y0);
        let c01 = px(x0, y1);
        let c11 = px(x1, y1);

        let top = c00 * (1.0 - fx) + c10 * fx;
        let bot = c01 * (1.0 - fx) + c11 * fx;
        top * (1.0 - fy) + bot * fy
    }
}

/// An equirectangular environment map, sampled by direction.
#[derive(Debug, Clone)]
pub struct EnvironmentMap {
    pub image: TextureImage,
}

impl EnvironmentMap {
    /// Sample the environment map by ray direction.
    pub fn sample(&self, direction: DVec3) -> DVec3 {
        let d = direction.normalize();
        // Equirectangular mapping: longitude → u, latitude → v
        let u = 0.5 + d.z.atan2(d.x) / (2.0 * std::f64::consts::PI);
        let v = 0.5 - d.y.asin() / std::f64::consts::PI;
        self.image.sample_bilinear(u, v)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub origin: DVec3,
    pub color: DVec3,
    pub radius: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_defaults() {
        let mat = Material {
            k_ambient: 0.1,
            k_diffuse: DVec3::new(1.0, 1.0, 1.0),
            k_specular: 0.5,
            ns: 10.0,
            is_reflector: false,
            is_refractor: false,
            ambient_color: DVec3::new(15.0, 75.0, 255.0),
            has_explicit_ambient: false,
            ior: 1.5,
            use_fresnel: false,
            roughness: None,
            metallic: None,
            absorption: DVec3::ZERO,
            texture: TextureRef::None,
        };

        assert_eq!(mat.ambient_color, DVec3::new(15.0, 75.0, 255.0));
        assert!(!mat.use_fresnel);
        assert_eq!(mat.absorption, DVec3::ZERO);
        assert!(matches!(mat.texture, TextureRef::None));
    }

    #[test]
    fn test_texture_image_bilinear_center() {
        // 2x2 texture: TL=red, TR=green, BL=blue, BR=white
        let tex = TextureImage {
            width: 2,
            height: 2,
            data: vec![
                DVec3::new(255.0, 0.0, 0.0),     // (0,0)
                DVec3::new(0.0, 255.0, 0.0),     // (1,0)
                DVec3::new(0.0, 0.0, 255.0),     // (0,1)
                DVec3::new(255.0, 255.0, 255.0), // (1,1)
            ],
        };
        // Sample at center (0.5, 0.5) should be average of all 4
        let c = tex.sample_bilinear(0.5, 0.5);
        let expected = (DVec3::new(255.0, 0.0, 0.0)
            + DVec3::new(0.0, 255.0, 0.0)
            + DVec3::new(0.0, 0.0, 255.0)
            + DVec3::new(255.0, 255.0, 255.0))
            / 4.0;
        assert!(
            (c - expected).length() < 1.0,
            "Bilinear center should average all 4 pixels, got {:?} expected {:?}",
            c,
            expected
        );
    }

    #[test]
    fn test_environment_map_directions() {
        // 4x2 solid red environment map
        let env = EnvironmentMap {
            image: TextureImage {
                width: 4,
                height: 2,
                data: vec![DVec3::new(255.0, 0.0, 0.0); 8],
            },
        };
        // Any direction should return red
        let c = env.sample(DVec3::new(1.0, 0.0, 0.0));
        assert!((c.x - 255.0).abs() < 1.0);
        assert!(c.y.abs() < 1.0);
    }
}
