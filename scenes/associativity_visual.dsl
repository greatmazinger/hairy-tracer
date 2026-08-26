material grey { kDiffuse: [0.7, 0.7, 0.7] kAmbient: 0.1 kSpecular: 0.2 nS: 20.0 }
material red { kDiffuse: [0.8, 0.1, 0.1] kAmbient: 0.1 kSpecular: 0.2 nS: 20.0 }
material floor { kDiffuse: [0.3, 0.3, 0.3] kAmbient: 0.1 kSpecular: 0.1 nS: 10.0 }

let A1 = translate(cube(min: [-1.0, 0.0, -1.0], max: [1.0, 2.0, 1.0]), x: -2.0)
let B1 = translate(cube(min: [0.0, 0.0, -1.0], max: [2.0, 2.0, 1.0]), x: -2.0)
let C1 = translate(cube(min: [0.5, 0.0, -1.0], max: [2.5, 2.0, 1.0]), x: -2.0)
let left_assoc = A1 - B1 - C1

let A2 = translate(cube(min: [-1.0, 0.0, -1.0], max: [1.0, 2.0, 1.0]), x: 2.0)
let B2 = translate(cube(min: [0.0, 0.0, -1.0], max: [2.0, 2.0, 1.0]), x: 2.0)
let C2 = translate(cube(min: [0.5, 0.0, -1.0], max: [2.5, 2.0, 1.0]), x: 2.0)
let right_assoc = A2 - (B2 - C2)

scene {
  camera { origin: [0.0, 4.0, 6.0], look_at: [0.0, 1.0, 0.0], up: [0.0, 1.0, 0.0], distance: 8.0, fov_degrees: 45deg, samples_per_pixel: 16 }
  light  { origin: [0.0, 8.0, 5.0], color: [1.0, 1.0, 1.0], radius: 0.0 }
  
  object(plane(normal: [0.0, 1.0, 0.0], distance: 0.0), material: floor)
  object(left_assoc, material: grey)
  object(right_assoc, material: red)
}
