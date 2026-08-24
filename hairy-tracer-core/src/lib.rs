pub mod aabb;
pub mod checkered_plane;
pub mod hit;
pub mod intersect;
pub mod material;
pub mod mesh;
pub mod plane;
pub mod ray;
pub mod render;
pub mod scene;
pub mod scene_parser;
pub mod sphere;
pub mod triangle;

use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyfunction]
fn render_image(
    py: Python<'_>,
    scene_json: &str,
    width: usize,
    height: usize,
    max_depth: u32,
) -> PyResult<Py<PyBytes>> {
    let (scene, cam) = scene_parser::parse_scene_json(scene_json)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        
    let cam_origin = glam::DVec3::from_array(cam.origin);
    
    // Dynamic viewport scaling matching Python
    let aspect_ratio = width as f64 / height as f64;
    let vpheight = cam.vpwidth / aspect_ratio;
    
    // Default look_at if None
    let look_at = if let Some(la) = cam.look_at {
        glam::DVec3::from_array(la)
    } else {
        glam::DVec3::new(0.0, 0.0, cam.distance)
    };
    
    let up = if let Some(up_vec) = cam.up {
        glam::DVec3::from_array(up_vec)
    } else {
        glam::DVec3::new(0.0, 1.0, 0.0)
    };
    
    let samples_per_pixel = cam.samples_per_pixel.unwrap_or(1);
    let aperture = cam.aperture.unwrap_or(0.0);
    let focal_distance = cam.focal_distance.unwrap_or(cam.distance); // focal_distance defaults to camera distance (w_norm)

    // Release GIL while rendering in parallel!
    let pixels = py.allow_threads(|| {
        render::render_image_parallel(
            &scene,
            cam_origin,
            look_at,
            up,
            cam.vpwidth,
            vpheight,
            width,
            height,
            max_depth,
            samples_per_pixel,
            aperture,
            focal_distance,
        )
    });
    
    Ok(PyBytes::new_bound(py, &pixels).into())
}

#[pymodule]
fn hairy_tracer_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_image, m)?)?;
    Ok(())
}
