# CSG Authoring Guidelines

## Coincident Surfaces (Z-Fighting)
When unioning or subtracting primitives, avoid creating **exactly coincident surfaces**. Floating-point precision issues cause CSG interval math to become ambiguous when testing rays near surfaces that share the exact same boundary, resulting in speckled, noisy patterns (Z-fighting).

**The Fix:**
Always overlap your CSG parts intentionally by a small margin.
- If unioning a gear tooth onto a radius 2.0 disk, do not translate the tooth to 2.0 with a min bound of 0.0. Instead, translate it to 1.9, so it cleanly intersects and embeds into the disk by 0.1 units.
- Apply this safely to all axes (e.g., extend Y boundaries by 0.05 to avoid speckling on flat top/bottom surfaces of unions).

## Enclosed Cavities and Whitted Lighting
When rendering deep cavities or bore holes (like the center of a CSG gear) with the standard Whitted integrator, you may see flat, pitch-black strips on the interior walls. This is often mistaken for a geometry error (like a leftover flat cap).

**Root Cause:**
This is an expected lighting limitation. In a single-bounce Whitted integrator without Global Illumination, areas that do not have direct line-of-sight to a light source (or are beyond the self-shadowing terminator where `N dot L = 0`) are purely black. The self-shadowing terminator on a cylindrical wall forms a perfectly straight vertical line, creating a "flat-edged" rectangular shadow that looks unnervingly like a flat geometric face.

**The Fix:**
- No geometry fix is needed.
- Render the scene with the Path Tracing integrator to allow light to bounce into the cavity, revealing the true smooth curved geometry.
- Alternatively, if using Whitted, place a small "fill light" inside or directly above the cavity to artificially light the interior.
