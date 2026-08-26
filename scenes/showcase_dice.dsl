material body_mat {
  kDiffuse: [0.8, 0.05, 0.05]
  kAmbient: 0.1
  kSpecular: 0.6
  nS: 40.0
  roughness: 0.2
  metallic: 0.1
  use_fresnel: true
}

material pip_mat {
  kDiffuse: [0.7, 0.7, 0.7]
  kAmbient: 0.1
  kSpecular: 0.1
  nS: 20.0
  roughness: 0.5
  metallic: 0.0
}

material floor_mat {
  kDiffuse: [0.2, 0.2, 0.25]
  kAmbient: 0.05
  kSpecular: 0.1
  nS: 10.0
  roughness: 0.9
}

let body = cube(min: [-1.0, -1.0, -1.0], max: [1.0, 1.0, 1.0], material: "body_mat")

let p1_1 = sphere(center: [0.0, 0.0, 1.0], radius: 0.2, material: "pip_mat")

let p6_1 = sphere(center: [-0.5, -0.6, -1.0], radius: 0.2, material: "pip_mat")
let p6_2 = sphere(center: [-0.5,  0.0, -1.0], radius: 0.2, material: "pip_mat")
let p6_3 = sphere(center: [-0.5,  0.6, -1.0], radius: 0.2, material: "pip_mat")
let p6_4 = sphere(center: [ 0.5, -0.6, -1.0], radius: 0.2, material: "pip_mat")
let p6_5 = sphere(center: [ 0.5,  0.0, -1.0], radius: 0.2, material: "pip_mat")
let p6_6 = sphere(center: [ 0.5,  0.6, -1.0], radius: 0.2, material: "pip_mat")

let p3_1 = sphere(center: [1.0, -0.6, -0.6], radius: 0.2, material: "pip_mat")
let p3_2 = sphere(center: [1.0,  0.0,  0.0], radius: 0.2, material: "pip_mat")
let p3_3 = sphere(center: [1.0,  0.6,  0.6], radius: 0.2, material: "pip_mat")

let p4_1 = sphere(center: [-1.0, -0.5, -0.5], radius: 0.2, material: "pip_mat")
let p4_2 = sphere(center: [-1.0,  0.5, -0.5], radius: 0.2, material: "pip_mat")
let p4_3 = sphere(center: [-1.0, -0.5,  0.5], radius: 0.2, material: "pip_mat")
let p4_4 = sphere(center: [-1.0,  0.5,  0.5], radius: 0.2, material: "pip_mat")

let p5_1 = sphere(center: [-0.5, 1.0, -0.5], radius: 0.2, material: "pip_mat")
let p5_2 = sphere(center: [ 0.5, 1.0, -0.5], radius: 0.2, material: "pip_mat")
let p5_3 = sphere(center: [-0.5, 1.0,  0.5], radius: 0.2, material: "pip_mat")
let p5_4 = sphere(center: [ 0.5, 1.0,  0.5], radius: 0.2, material: "pip_mat")
let p5_5 = sphere(center: [ 0.0, 1.0,  0.0], radius: 0.2, material: "pip_mat")

let p2_1 = sphere(center: [-0.5, -1.0, -0.5], radius: 0.2, material: "pip_mat")
let p2_2 = sphere(center: [ 0.5, -1.0,  0.5], radius: 0.2, material: "pip_mat")

let die = body - p1_1 - p6_1 - p6_2 - p6_3 - p6_4 - p6_5 - p6_6 - p3_1 - p3_2 - p3_3 - p4_1 - p4_2 - p4_3 - p4_4 - p5_1 - p5_2 - p5_3 - p5_4 - p5_5 - p2_1 - p2_2

let posed_die = translate(rotate(rotate(die, y: -45deg), x: 30deg), y: 1.5)

scene {
  environment_map: "scenes/studio_env.png"
  camera { 
    origin: [0.0, 4.2, 7.0], 
    look_at: [0.0, 1.0, 0.0], 
    up: [0.0, 1.0, 0.0], 
    distance: 1.0, 
    fov_degrees: 35deg, 
    samples_per_pixel: 256,
    aperture: 0.15,
    focal_distance: 7.7
  }
  
  light { origin: [4.0, 6.0, 4.0], color: [1.2, 1.2, 1.2], radius: 1.0 }
  
  light { origin: [-5.0, 3.0, 2.0], color: [0.3, 0.4, 0.5], radius: 2.0 }
  
  light { origin: [0.0, 5.0, -5.0], color: [0.6, 0.5, 0.4], radius: 2.0 }
  
  object(posed_die)
  object(plane(normal: [0.0, 1.0, 0.0], distance: 0.0), material: floor_mat)
}
