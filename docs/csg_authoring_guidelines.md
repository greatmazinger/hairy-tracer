# CSG Authoring Guidelines

## Coincident Surfaces (Z-Fighting)
When unioning or subtracting primitives, avoid creating **exactly coincident surfaces**. Floating-point precision issues cause CSG interval math to become ambiguous when testing rays near surfaces that share the exact same boundary, resulting in speckled, noisy patterns (Z-fighting).

**The Fix:**
Always overlap your CSG parts intentionally by a small margin.
- If unioning a gear tooth onto a radius 2.0 disk, do not translate the tooth to 2.0 with a min bound of 0.0. Instead, translate it to 1.9, so it cleanly intersects and embeds into the disk by 0.1 units.
- Apply this safely to all axes (e.g., extend Y boundaries by 0.05 to avoid speckling on flat top/bottom surfaces of unions).
