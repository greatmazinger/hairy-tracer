with open('scenes/showcase_dice.dsl', 'r') as f:
    c = f.read()

import re

# Extract body_mat and pip_mat
body_mat = re.search(r'material body_mat \{.*?\n\}', c, re.DOTALL).group(0)
pip_mat = re.search(r'material pip_mat \{.*?\n\}', c, re.DOTALL).group(0)
floor_mat = re.search(r'material floor_mat \{.*?\n\}', c, re.DOTALL).group(0)

# Create ivory_body_mat
ivory_body_mat = body_mat.replace('body_mat', 'ivory_body_mat')
ivory_body_mat = re.sub(r'kDiffuse: \[.*?\]', 'kDiffuse: [0.94, 0.89, 0.80]', ivory_body_mat, count=1)

# Create ivory_pip_mat
ivory_pip_mat = """material ivory_pip_mat {
  kDiffuse: [0.05, 0.08, 0.25]
  kAmbient: 0.1
  kSpecular: 0.5
  nS: 40.0
  roughness: 0.1
  metallic: 0.0
}"""

pips_template = """
let {p}1_1 = sphere(center: [0.0, 0.0, 1.0], radius: 0.2, material: "{m}")

let {p}6_1 = sphere(center: [-0.5, -0.6, -1.0], radius: 0.2, material: "{m}")
let {p}6_2 = sphere(center: [-0.5,  0.0, -1.0], radius: 0.2, material: "{m}")
let {p}6_3 = sphere(center: [-0.5,  0.6, -1.0], radius: 0.2, material: "{m}")
let {p}6_4 = sphere(center: [ 0.5, -0.6, -1.0], radius: 0.2, material: "{m}")
let {p}6_5 = sphere(center: [ 0.5,  0.0, -1.0], radius: 0.2, material: "{m}")
let {p}6_6 = sphere(center: [ 0.5,  0.6, -1.0], radius: 0.2, material: "{m}")

let {p}3_1 = sphere(center: [1.0, -0.6, -0.6], radius: 0.2, material: "{m}")
let {p}3_2 = sphere(center: [1.0,  0.0,  0.0], radius: 0.2, material: "{m}")
let {p}3_3 = sphere(center: [1.0,  0.6,  0.6], radius: 0.2, material: "{m}")

let {p}4_1 = sphere(center: [-1.0, -0.5, -0.5], radius: 0.2, material: "{m}")
let {p}4_2 = sphere(center: [-1.0,  0.5, -0.5], radius: 0.2, material: "{m}")
let {p}4_3 = sphere(center: [-1.0, -0.5,  0.5], radius: 0.2, material: "{m}")
let {p}4_4 = sphere(center: [-1.0,  0.5,  0.5], radius: 0.2, material: "{m}")

let {p}5_1 = sphere(center: [-0.5, 1.0, -0.5], radius: 0.2, material: "{m}")
let {p}5_2 = sphere(center: [ 0.5, 1.0, -0.5], radius: 0.2, material: "{m}")
let {p}5_3 = sphere(center: [-0.5, 1.0,  0.5], radius: 0.2, material: "{m}")
let {p}5_4 = sphere(center: [ 0.5, 1.0,  0.5], radius: 0.2, material: "{m}")
let {p}5_5 = sphere(center: [ 0.0, 1.0,  0.0], radius: 0.2, material: "{m}")

let {p}2_1 = sphere(center: [-0.5, -1.0, -0.5], radius: 0.2, material: "{m}")
let {p}2_2 = sphere(center: [ 0.5, -1.0,  0.5], radius: 0.2, material: "{m}")

let {p}all = {p}1_1 | {p}6_1 | {p}6_2 | {p}6_3 | {p}6_4 | {p}6_5 | {p}6_6 | {p}3_1 | {p}3_2 | {p}3_3 | {p}4_1 | {p}4_2 | {p}4_3 | {p}4_4 | {p}5_1 | {p}5_2 | {p}5_3 | {p}5_4 | {p}5_5 | {p}2_1 | {p}2_2
"""

red_pips = pips_template.format(p="r_", m="pip_mat")
ivory_pips = pips_template.format(p="i_", m="ivory_pip_mat")

base_geom = """
let red_body = cube(min: [-1.0, -1.0, -1.0], max: [1.0, 1.0, 1.0], material: "body_mat")
let ivory_body = cube(min: [-1.0, -1.0, -1.0], max: [1.0, 1.0, 1.0], material: "ivory_body_mat")

let red_die = red_body - r_all
let ivory_die = ivory_body - i_all

let posed_red = translate(rotate(rotate(rotate(red_die, y: -65deg), x: -25deg), z: -10deg), x: -1.3, y: 1.2, z: 0.0)

let posed_ivory = translate(rotate(rotate(rotate(ivory_die, y: 65deg), x: -25deg), z: 15deg), x: 1.3, y: 1.2, z: -0.5)
"""

scene = """
scene {
  environment_map: "scenes/studio_env.png"
  camera { 
    origin: [0.0, 6.0, 12.0], 
    look_at: [0.0, 1.0, 0.0], 
    up: [0.0, 1.0, 0.0], 
    distance: 1.0, 
    fov_degrees: 45deg, 
    samples_per_pixel: 16,
    aperture: 0.35,
    focal_distance: 13.3
  }
  
  light { origin: [4.0, 6.0, 4.0], color: [1.2, 1.2, 1.2], radius: 1.0 }
  light { origin: [-5.0, 3.0, 2.0], color: [0.3, 0.4, 0.5], radius: 2.0 }
  light { origin: [0.0, 5.0, -5.0], color: [0.6, 0.5, 0.4], radius: 2.0 }
  
  object(posed_red)
  object(posed_ivory)
  object(plane(normal: [0.0, 1.0, 0.0], distance: 0.0), material: "floor_mat")
}
"""

dsl = body_mat + "\n\n" + pip_mat + "\n\n" + ivory_body_mat + "\n\n" + ivory_pip_mat + "\n\n" + floor_mat + "\n\n" + red_pips + "\n" + ivory_pips + "\n" + base_geom + "\n" + scene

with open('scenes/showcase_dice_craps.dsl', 'w') as f:
    f.write(dsl)
