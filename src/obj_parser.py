import os
import numpy

def load_obj(filepath):
    """
    Parses a Wavefront .OBJ file and returns a list of triangles.
    Each triangle is a tuple of 3 numpy arrays: (v0, v1, v2).
    Automatically triangulates polygons (N-gons) using a triangle fan approach.
    """
    vertices = []
    triangles = []
    
    with open(filepath, 'r') as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
                
            parts = line.split()
            if not parts:
                continue
                
            prefix = parts[0]
            
            if prefix == 'v':
                # Vertex definition: v x y z
                x, y, z = map(float, parts[1:4])
                vertices.append(numpy.array([x, y, z]))
                
            elif prefix == 'f':
                # Face definition: f v1/vt1/vn1 v2/vt2/vn2 ...
                face_indices = []
                for p in parts[1:]:
                    # Extract the first number (vertex index) before any slashes
                    v_idx_str = p.split('/')[0]
                    # OBJ indices are 1-based, so subtract 1 for 0-based Python arrays
                    v_idx = int(v_idx_str) - 1
                    
                    # Handle negative indices (relative to the end of the vertex list)
                    if v_idx < 0:
                        v_idx = len(vertices) + v_idx + 1
                        
                    face_indices.append(v_idx)
                    
                # Triangulate the face (triangle fan)
                if len(face_indices) >= 3:
                    v0 = face_indices[0]
                    for i in range(1, len(face_indices) - 1):
                        v1 = face_indices[i]
                        v2 = face_indices[i + 1]
                        triangles.append((vertices[v0], vertices[v1], vertices[v2]))
                        
    return triangles
