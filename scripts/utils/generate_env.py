import numpy as np
import imageio.v3 as iio

width = 2048
height = 1024
img = np.zeros((height, width, 3), dtype=np.float32)

for y in range(height):
    v = y / height
    intensity = max(0, 1.0 - v * 1.5) * 0.1
    img[y, :, :] = [intensity * 1.0, intensity * 0.95, intensity * 0.9]

for y in range(height):
    for x in range(width):
        u = x / width
        v = y / height
        if 0.4 < u < 0.6 and 0.1 < v < 0.35:
            edge_u = min(u - 0.4, 0.6 - u) / 0.1
            edge_v = min(v - 0.1, 0.35 - v) / 0.125
            intensity = min(edge_u, edge_v) * 0.9 # Cap at < 1.0 to fit in LDR
            img[y, x] += np.array([intensity, intensity, intensity])

img_u8 = np.clip(img * 255, 0, 255).astype(np.uint8)
iio.imwrite('scenes/studio_env.png', img_u8)
