use glam::DVec3;
use hairy_tracer_core::hit::Hit;
use hairy_tracer_core::intersect::Intersectable;
use hairy_tracer_core::material::MaterialId;
use hairy_tracer_core::plane::Plane;
use hairy_tracer_core::ray::Ray;
use hairy_tracer_core::sphere::Sphere;
use hairy_tracer_core::triangle::Triangle;

const MAT: MaterialId = MaterialId(0);

#[test]
fn test_sphere_uv_equator() {
    let s = Sphere::new(DVec3::ZERO, 1.0, MAT);
    // Hit at x=1, z=0, equator
    let r = Ray::new(DVec3::new(2.0, 0.0, 0.0), DVec3::new(-1.0, 0.0, 0.0));
    let hit = s.intersect(&r, 0).unwrap();
    // normal is (1, 0, 0). z.atan2(x) = 0.0. u = 0.5.
    // normal.y.asin() = 0.0. v = 0.5.
    assert!((hit.u - 0.5).abs() < 1e-6);
    assert!((hit.v - 0.5).abs() < 1e-6);
}

#[test]
fn test_sphere_uv_pole() {
    let s = Sphere::new(DVec3::ZERO, 1.0, MAT);
    // Hit at y=1, pole
    let r = Ray::new(DVec3::new(0.0, 2.0, 0.0), DVec3::new(0.0, -1.0, 0.0));
    let hit = s.intersect(&r, 0).unwrap();
    // normal is (0, 1, 0). normal.y.asin() = PI/2. v = 0.5 + 0.5 = 1.0.
    assert!((hit.v - 1.0).abs() < 1e-6);
}

#[test]
fn test_plane_uv() {
    let p = Plane::new(DVec3::new(0.0, 1.0, 0.0),
        0.0, MAT);
    let r = Ray::new(DVec3::new(1.25, 2.0, -0.75), DVec3::new(0.0, -1.0, 0.0));
    let hit = p.intersect(&r, 0).unwrap();
    // point is (1.25, 0.0, -0.75)
    // u = 1.25 - 1.0 = 0.25
    // v = -0.75 - (-1.0) = 0.25
    assert!((hit.u - 0.25).abs() < 1e-6);
    assert!((hit.v - 0.25).abs() < 1e-6);
}

#[test]
fn test_triangle_uv() {
    let t = Triangle::new(
        DVec3::new(-1.0, -1.0, 0.0),
        DVec3::new(1.0, -1.0, 0.0),
        DVec3::new(0.0, 1.0, 0.0),
        MAT, 0,
    );
    // Hit center
    let r = Ray::new(
        DVec3::new(0.0, -0.333333333, 5.0),
        DVec3::new(0.0, 0.0, -1.0),
    );
    let hit = t.intersect(&r, 0).unwrap();

    // barycentric center should have roughly equal u,v (~0.33)
    assert!((hit.u - 0.333333).abs() < 1e-3);
    assert!((hit.v - 0.333333).abs() < 1e-3);
}
