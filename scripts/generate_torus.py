import math

def generate_torus(major_r=2.0, minor_r=0.8, u_segments=30, v_segments=15, filepath="models/torus.obj"):
    vertices = []
    
    # Generate vertices
    for i in range(u_segments):
        u = (i / u_segments) * 2 * math.pi
        for j in range(v_segments):
            v = (j / v_segments) * 2 * math.pi
            
            # Standard Torus equation (flat on XZ plane)
            x = (major_r + minor_r * math.cos(v)) * math.cos(u)
            y = minor_r * math.sin(v)
            z = (major_r + minor_r * math.cos(v)) * math.sin(u)
            
            # Let's rotate it 45 degrees around X so we can see the hole from the side
            angle = math.pi / 4
            y_rot = y * math.cos(angle) - z * math.sin(angle)
            z_rot = y * math.sin(angle) + z * math.cos(angle)
            
            # Let's rotate it 30 degrees around Y for an isometric look
            angle2 = math.pi / 6
            x_rot2 = x * math.cos(angle2) + z_rot * math.sin(angle2)
            z_rot2 = -x * math.sin(angle2) + z_rot * math.cos(angle2)
            
            vertices.append((x_rot2, y_rot, z_rot2))
            
    with open(filepath, "w") as f:
        f.write("# Procedural Torus\n")
        
        for v in vertices:
            f.write(f"v {v[0]:.4f} {v[1]:.4f} {v[2]:.4f}\n")
            
        # Generate Quad Faces
        for i in range(u_segments):
            for j in range(v_segments):
                # Calculate indices (1-based for OBJ)
                next_i = (i + 1) % u_segments
                next_j = (j + 1) % v_segments
                
                v1 = i * v_segments + j + 1
                v2 = next_i * v_segments + j + 1
                v3 = next_i * v_segments + next_j + 1
                v4 = i * v_segments + next_j + 1
                
                f.write(f"f {v1} {v2} {v3} {v4}\n")

if __name__ == "__main__":
    generate_torus()
    print("models/torus.obj generated successfully!")
