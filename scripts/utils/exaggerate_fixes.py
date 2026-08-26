import re

for filename in ['scenes/showcase_dice.dsl', 'scenes/showcase_dice_ivory.dsl']:
    with open(filename, 'r') as f:
        c = f.read()
    
    # 1. Pip shading: make it much darker so it's obvious
    c = re.sub(r'kDiffuse: \[0.7, 0.7, 0.7\]', 'kDiffuse: [0.3, 0.3, 0.3]', c)
    
    # 2. Framing: Move camera WAY back and widen FOV
    c = re.sub(r'origin: \[0.0, 4.2, 7.0\]', 'origin: [0.0, 6.0, 10.0]', c)
    c = re.sub(r'fov_degrees: 35deg', 'fov_degrees: 45deg', c)
    
    # 3. DOF: Massive aperture
    c = re.sub(r'aperture: 0.15', 'aperture: 0.5', c)
    c = re.sub(r'focal_distance: 7.7', 'focal_distance: 11.66', c) # sqrt(6^2 + 10^2)
    
    # 4. Ivory color: make it extremely warm/yellow
    if 'ivory' in filename:
        c = re.sub(r'kDiffuse: \[0.98, 0.90, 0.78\]', 'kDiffuse: [1.0, 0.8, 0.4]', c)
        
    with open(filename, 'w') as f:
        f.write(c)

