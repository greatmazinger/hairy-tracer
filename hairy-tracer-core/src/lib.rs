pub mod normals_integrator;
pub mod integrator;
pub mod path_tracer;
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
pub mod camera;
pub mod cube;
pub mod cylinder;
pub mod csg;
pub mod transform;

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

    // Default look_at if None
    let distance = cam.distance.unwrap_or(1.0);
    let look_at = if let Some(la) = cam.look_at {
        glam::DVec3::from_array(la)
    } else {
        glam::DVec3::new(0.0, 0.0, distance)
    };

    // Calculate vpwidth and vpheight, with fov_degrees taking precedence
    let mut vpwidth = cam.vpwidth.unwrap_or(1.0);
    if let Some(fov) = cam.fov_degrees {
        let fov_rad = fov.to_radians();
        vpwidth = 2.0 * distance * (fov_rad / 2.0).tan();
    }
    
    // Dynamic viewport scaling matching Python
    let aspect_ratio = width as f64 / height as f64;
    let vpheight = vpwidth / aspect_ratio;

    let up = if let Some(up_vec) = cam.up {
        glam::DVec3::from_array(up_vec)
    } else {
        glam::DVec3::new(0.0, 1.0, 0.0)
    };

    let samples_per_pixel = cam.samples_per_pixel.unwrap_or(1);
    let aperture = cam.aperture.unwrap_or(0.0);
    let focal_distance = cam.focal_distance.unwrap_or(distance); // focal_distance defaults to camera distance (w_norm)


    println!("Integrator from scene: {}", scene.integrator);
    let integrator: Box<dyn crate::integrator::Integrator> = match scene.integrator.as_str() {
        "whitted" => Box::new(render::WhittedIntegrator),
        "pathtracer" => Box::new(crate::path_tracer::PathTracingIntegrator),
        "normals" => Box::new(normals_integrator::NormalsIntegrator),
        _ => Box::new(render::WhittedIntegrator),
    };

    // Release GIL while rendering in parallel!
    let pixels = py.allow_threads(|| {
        render::render_image_parallel(integrator.as_ref(),
            &scene,
            cam_origin,
            look_at,
            up,
            distance,
            vpwidth,
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
