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
  kDiffuse: [0.55, 0.55, 0.55]
  kAmbient: 0.1
  kSpecular: 0.1
  nS: 20.0
  roughness: 0.5
  metallic: 0.0
}

material ivory_body_mat {
  kDiffuse: [0.94, 0.89, 0.80]
  kAmbient: 0.1
  kSpecular: 0.6
  nS: 40.0
  roughness: 0.2
  metallic: 0.1
  use_fresnel: true
}

material ivory_pip_mat {
  kDiffuse: [0.05, 0.08, 0.25]
  kAmbient: 0.1
  kSpecular: 0.5
  nS: 40.0
  roughness: 0.1
  metallic: 0.0
}

material floor_mat {
  kDiffuse: [0.2, 0.2, 0.25]
  kAmbient: 0.05
  kSpecular: 0.1
  nS: 10.0
  roughness: 0.9
}


let r_1_1 = sphere(center: [0.0, 0.0, 1.0], radius: 0.2, material: "pip_mat")

let r_6_1 = sphere(center: [-0.5, -0.6, -1.0], radius: 0.2, material: "pip_mat")
let r_6_2 = sphere(center: [-0.5,  0.0, -1.0], radius: 0.2, material: "pip_mat")
let r_6_3 = sphere(center: [-0.5,  0.6, -1.0], radius: 0.2, material: "pip_mat")
let r_6_4 = sphere(center: [ 0.5, -0.6, -1.0], radius: 0.2, material: "pip_mat")
let r_6_5 = sphere(center: [ 0.5,  0.0, -1.0], radius: 0.2, material: "pip_mat")
let r_6_6 = sphere(center: [ 0.5,  0.6, -1.0], radius: 0.2, material: "pip_mat")

let r_3_1 = sphere(center: [1.0, -0.6, -0.6], radius: 0.2, material: "pip_mat")
let r_3_2 = sphere(center: [1.0,  0.0,  0.0], radius: 0.2, material: "pip_mat")
let r_3_3 = sphere(center: [1.0,  0.6,  0.6], radius: 0.2, material: "pip_mat")

let r_4_1 = sphere(center: [-1.0, -0.5, -0.5], radius: 0.2, material: "pip_mat")
let r_4_2 = sphere(center: [-1.0,  0.5, -0.5], radius: 0.2, material: "pip_mat")
let r_4_3 = sphere(center: [-1.0, -0.5,  0.5], radius: 0.2, material: "pip_mat")
let r_4_4 = sphere(center: [-1.0,  0.5,  0.5], radius: 0.2, material: "pip_mat")

let r_5_1 = sphere(center: [-0.5, 1.0, -0.5], radius: 0.2, material: "pip_mat")
let r_5_2 = sphere(center: [ 0.5, 1.0, -0.5], radius: 0.2, material: "pip_mat")
let r_5_3 = sphere(center: [-0.5, 1.0,  0.5], radius: 0.2, material: "pip_mat")
let r_5_4 = sphere(center: [ 0.5, 1.0,  0.5], radius: 0.2, material: "pip_mat")
let r_5_5 = sphere(center: [ 0.0, 1.0,  0.0], radius: 0.2, material: "pip_mat")

let r_2_1 = sphere(center: [-0.5, -1.0, -0.5], radius: 0.2, material: "pip_mat")
let r_2_2 = sphere(center: [ 0.5, -1.0,  0.5], radius: 0.2, material: "pip_mat")

let r_all = r_1_1 | r_6_1 | r_6_2 | r_6_3 | r_6_4 | r_6_5 | r_6_6 | r_3_1 | r_3_2 | r_3_3 | r_4_1 | r_4_2 | r_4_3 | r_4_4 | r_5_1 | r_5_2 | r_5_3 | r_5_4 | r_5_5 | r_2_1 | r_2_2


let i_1_1 = sphere(center: [0.0, 0.0, 1.0], radius: 0.2, material: "ivory_pip_mat")

let i_6_1 = sphere(center: [-0.5, -0.6, -1.0], radius: 0.2, material: "ivory_pip_mat")
let i_6_2 = sphere(center: [-0.5,  0.0, -1.0], radius: 0.2, material: "ivory_pip_mat")
let i_6_3 = sphere(center: [-0.5,  0.6, -1.0], radius: 0.2, material: "ivory_pip_mat")
let i_6_4 = sphere(center: [ 0.5, -0.6, -1.0], radius: 0.2, material: "ivory_pip_mat")
let i_6_5 = sphere(center: [ 0.5,  0.0, -1.0], radius: 0.2, material: "ivory_pip_mat")
let i_6_6 = sphere(center: [ 0.5,  0.6, -1.0], radius: 0.2, material: "ivory_pip_mat")

let i_3_1 = sphere(center: [1.0, -0.6, -0.6], radius: 0.2, material: "ivory_pip_mat")
let i_3_2 = sphere(center: [1.0,  0.0,  0.0], radius: 0.2, material: "ivory_pip_mat")
let i_3_3 = sphere(center: [1.0,  0.6,  0.6], radius: 0.2, material: "ivory_pip_mat")

let i_4_1 = sphere(center: [-1.0, -0.5, -0.5], radius: 0.2, material: "ivory_pip_mat")
let i_4_2 = sphere(center: [-1.0,  0.5, -0.5], radius: 0.2, material: "ivory_pip_mat")
let i_4_3 = sphere(center: [-1.0, -0.5,  0.5], radius: 0.2, material: "ivory_pip_mat")
let i_4_4 = sphere(center: [-1.0,  0.5,  0.5], radius: 0.2, material: "ivory_pip_mat")

let i_5_1 = sphere(center: [-0.5, 1.0, -0.5], radius: 0.2, material: "ivory_pip_mat")
let i_5_2 = sphere(center: [ 0.5, 1.0, -0.5], radius: 0.2, material: "ivory_pip_mat")
let i_5_3 = sphere(center: [-0.5, 1.0,  0.5], radius: 0.2, material: "ivory_pip_mat")
let i_5_4 = sphere(center: [ 0.5, 1.0,  0.5], radius: 0.2, material: "ivory_pip_mat")
let i_5_5 = sphere(center: [ 0.0, 1.0,  0.0], radius: 0.2, material: "ivory_pip_mat")

let i_2_1 = sphere(center: [-0.5, -1.0, -0.5], radius: 0.2, material: "ivory_pip_mat")
let i_2_2 = sphere(center: [ 0.5, -1.0,  0.5], radius: 0.2, material: "ivory_pip_mat")

let i_all = i_1_1 | i_6_1 | i_6_2 | i_6_3 | i_6_4 | i_6_5 | i_6_6 | i_3_1 | i_3_2 | i_3_3 | i_4_1 | i_4_2 | i_4_3 | i_4_4 | i_5_1 | i_5_2 | i_5_3 | i_5_4 | i_5_5 | i_2_1 | i_2_2


let red_body = cube(min: [-1.0, -1.0, -1.0], max: [1.0, 1.0, 1.0], material: "body_mat")
let ivory_body = cube(min: [-1.0, -1.0, -1.0], max: [1.0, 1.0, 1.0], material: "ivory_body_mat")

let red_die = red_body - r_all
let ivory_die = ivory_body - i_all

let posed_red = translate(rotate(rotate(rotate(red_die, y: -65deg), x: -25deg), z: -10deg), x: -1.75, y: 1.53, z: 0.0)

let posed_ivory = translate(rotate(rotate(rotate(ivory_die, y: 65deg), x: -25deg), z: 15deg), x: 1.75, y: 1.54, z: -0.5)


scene {
  environment_map: "scenes/studio_env.png"
  camera { 
    origin: [0.0, 6.0, 12.0], 
    look_at: [0.0, 1.0, 0.0], 
    up: [0.0, 1.0, 0.0], 
    distance: 1.0, 
    fov_degrees: 45deg, 
    samples_per_pixel: 256,
    aperture: 0.35,
    focal_distance: 13.3
  }
  
  light { origin: [4.0, 6.0, 4.0], color: [1.2, 1.2, 1.2], radius: 1.0 }
  light { origin: [-5.0, 3.0, 2.0], color: [0.3, 0.4, 0.5], radius: 2.0 }
  light { origin: [0.0, 5.0, -5.0], color: [0.6, 0.5, 0.4], radius: 2.0 }
  
  object(posed_red)
  object(posed_ivory)
  object(plane(normal: [0.0, 1.0, 0.0], distance: 0.0), material: floor_mat)
}
