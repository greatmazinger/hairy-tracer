material gear_mat {
  kDiffuse: [0.8, 0.5, 0.2]
  kAmbient: 0.15
  kSpecular: 0.8
  nS: 60.0
}

material floor_mat {
  kDiffuse: [0.15, 0.2, 0.23]
  kAmbient: 0.15
  kSpecular: 0.3
  nS: 20.0
}

let tooth = cube(min: [-0.25, -0.55, 0.0], max: [0.25, 0.55, 0.35])
let disk  = cylinder(center: [0.0, 0.0, 0.0], radius: 2.0, height: 1.0, axis: "y")
          - cylinder(center: [0.0, 0.0, 0.0], radius: 0.5, height: 1.1, axis: "y")

let gear = disk | for i in 0..24 {
    rotate(translate(tooth, z: 1.9), y: i * 15deg)
}

scene {
  camera { origin: [0.0, 5.0, 7.0], look_at: [0.0, 0.0, 0.0], up: [0.0, 1.0, 0.0], distance: 8.0, fov_degrees: 45deg, samples_per_pixel: 16 }
  light  { origin: [5.0, 8.0, 6.0], color: [1.0, 0.95, 0.9], radius: 0.8 }
  light  { origin: [-6.0, 3.0, -2.0], color: [0.3, 0.4, 0.6], radius: 0.0 }
  light  { origin: [-2.0, 6.0, -8.0], color: [0.5, 0.5, 0.5], radius: 0.0 }
  object(gear, material: gear_mat)
}
