use std::f64::consts::PI;

fn main() {
    let r0: f64 = ((1.0_f64 - 1.5) / (1.0_f64 + 1.5)).powi(2);
    let cos_theta_grazing: f64 = 0.01;
    let reflectance_grazing = r0 + (1.0 - r0) * (1.0 - cos_theta_grazing).powi(5);
    let cos_theta_normal: f64 = 1.0;
    let reflectance_normal = r0 + (1.0 - r0) * (1.0 - cos_theta_normal).powi(5);
    println!("Fresnel Normal: {}", reflectance_normal);
    println!("Fresnel Grazing: {}", reflectance_grazing);

    let scale = 2.0;
    let u = 0.25; let v = 0.25;
    let u_cell = (u * scale).floor() as i32;
    let v_cell = (v * scale).floor() as i32;
    println!("Checker (0.25, 0.25) -> Cell {},{} (Sum {})", u_cell, v_cell, (u_cell+v_cell)%2);
    let u = 0.75; let v = 0.25;
    let u_cell = (u * scale).floor() as i32;
    let v_cell = (v * scale).floor() as i32;
    println!("Checker (0.75, 0.25) -> Cell {},{} (Sum {})", u_cell, v_cell, (u_cell+v_cell)%2);
}
