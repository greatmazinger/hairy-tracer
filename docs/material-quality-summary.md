# Material Quality Phase Summary

The Material Quality phase is complete. This phase significantly upgraded the physical accuracy of the engine by introducing energy-conserving Fresnel blending, volumetric absorption, procedural and image-based texturing, and equirectangular environment mapping.

All legacy tests remain 100% green, as the new features default to mathematically identical no-ops when not explicitly enabled in the JSON scene.

Here are the specific capabilities and the mathematical validations backing them:

## 1. Fresnel-Schlick Weighting
We replaced the legacy additive reflection/refraction (which summed to >100% light) with an energy-conserving Fresnel blend using the Schlick approximation. 
- **Normal Incidence**: For glass with an IOR of 1.5, the base reflectance ($R_0$) is exactly `0.04`. The test asserts that a ray striking head-on calculates a reflectance mathematically identical to `0.04` (within an error of `1e-10`).
- **Grazing Incidence**: The test asserts that a ray striking at a grazing angle (`cos_theta = 0.01`) forces the Schlick approximation curve up past `0.95`, demonstrating near-total internal reflection instead of a fixed fractional blend.

## 2. Beer-Lambert Absorption
Glass objects can now volumetrically absorb light based on geometric thickness. 
- **Test Validation**: We fire a single ray straight through three glass spheres of varying radii (0.5, 1.0, 2.0). The test extracts the exact transmittance and successfully asserts that the attenuation matches the theoretical Beer-Lambert curve ($e^{-\mu \times 2r}$) adjusted for normal-incidence Fresnel loss, within a ±15% tolerance.

## 3. Procedural Checker Textures
Objects can now derive color dynamically from UV coordinates.
- **Test Validation**: The procedural function uses `scale = 2.0`. We explicitly pass `uv(0.25, 0.25)` and `uv(0.75, 0.25)` to the `checker` logic, mathematically asserting that they land in `u_cell=0` (even, color A) and `u_cell=1` (odd, color B) respectively, proving precise cell alternation.

## 4. Image Texture Lookups
The engine now uses the `image` crate to load texture maps, storing them natively as `DVec3` floats to allow seamless HDR upgrades later. We implemented a custom bilinear interpolator for sub-pixel accuracy.
- **Test Validation**: We built a 2x2 mock texture (Red, Green, Blue, White) in memory. Passing `uv(0.25, 0.25)` correctly returns pure Red, verifying center alignment. More importantly, we test `uv(0.5, 0.25)` (exactly between the red and green texels), and explicitly assert that it returns a 50/50 blend (`[127.5, 127.5, 0]`) rather than a pure nearest-neighbor color.

## 5. Environment Map Spherical Lookups
A global environment map can now be used for ambient skybox lighting and infinite reflections/refractions.
- **Test Validation**: We mocked a 4x8 equirectangular map split horizontally (Top 4 rows Blue, Bottom 4 rows Red). The test asserts that a ray pointing 45° upward (`[0.0, 0.707, 0.707]`) resolves perfectly into the top half (`v ≈ 0.25`, pure Blue), and a ray pointing 45° downward resolves perfectly into the bottom half (`v ≈ 0.75`, pure Red). Furthermore, we ensure the ray tracer correctly returns the map color when a ray misses all objects.

## Important Note on Ray Depth
To properly support environment map reflections appearing *inside* glass spheres, the `maxdepth` command-line argument for the Rust engine in `btrace.py` has been increased from the legacy default of `2` up to `5`. This ensures that primary rays (depth 1), mirror reflections (depth 2), and subsequent refractions (depth 3+) have enough lifecycle to sample the background before returning pitch black.
