import re

with open('scenes/showcase_dice_ivory.dsl', 'r') as f:
    content = f.read()

# Replace body_mat
body_mat_new = """material body_mat {
  kDiffuse: [0.92, 0.89, 0.82]
  kAmbient: 0.1
  kSpecular: 0.5
  nS: 35.0
  roughness: 0.25
  metallic: 0.05
  use_fresnel: true
}"""
content = re.sub(r'material body_mat \{.*?\n\}', body_mat_new, content, flags=re.DOTALL)

# Replace pip_mat
pip_mat_new = """material pip_mat {
  kDiffuse: [0.02, 0.02, 0.02]
  kAmbient: 0.05
  kSpecular: 0.9
  nS: 80.0
  roughness: 0.05
  metallic: 0.0
}"""
content = re.sub(r'material pip_mat \{.*?\n\}', pip_mat_new, content, flags=re.DOTALL)

with open('scenes/showcase_dice_ivory.dsl', 'w') as f:
    f.write(content)
