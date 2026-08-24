# Prompt: Foundation phase — material system + UV coordinates
---

## Context

`hairy_tracer_core` is now a complete, working, pixel-perfect Rust replacement for the Python raytracer's rendering pipeline: geometric intersection, BVH-less AABB acceleration, Phong shading, recursive shadow/reflection/refraction, look-at camera, rayon tile parallelism, JSON/OBJ parsing, and PyO3 bindings wired into `btrace.py` (with the pure-Python path kept as a `--pure-python` fallback). The E2E snapshot tests confirm exact pixel match between both paths, and the full pytest suite is green.

This is the first of several planned follow-on phases, chosen to go first because several later phases depend on it:

1. **This task — Foundation**: refactor the material representation and add UV coordinates
2. *(later)* Sampling — anti-aliasing, soft shadows, depth of field
3. *(later)* Material quality — Fresnel weighting, textures, environment maps (needs this task's UV support)
4. *(later)* Acceleration — real recursive BVH
5. *(later)* Global illumination — path tracing, PBR materials

**This task is purely structural.** No new visual features, no texture sampling, no Fresnel — just cleaning up what materials are represented as, and adding UV data that later phases will consume. Do not implement anything from phase 3 while doing this task, even if it seems like "just one more small addition."

## Before writing any code

Read the current material handling in both the Rust crate and the original Python source — don't assume a design, derive it from what's actually there:
- How is a material currently represented in `hairy_tracer_core`? Is it a struct per object, fields inline on each primitive, or something else?
- Find the "geometry-specific hardcoded ambient lighting" behavior mentioned when the crate was first ported (the report specifically called this out as a quirk of the original Python that was replicated, not fixed) — locate exactly where it lives in both the Python and Rust code, and understand why it's geometry-specific rather than material-specific before deciding how to generalize it
- What fields does the Python material representation actually have (ambient/diffuse/specular coefficients, reflectivity, transparency, IOR, color) and what are their current defaults/ranges?
- Does the existing `.obj` parser (custom reader, per the integration report) already read UV coordinates from the file and discard them, or does it not parse them at all? This determines whether mesh UV support is "wire up existing data" or "extend the parser."

If the hardcoded-ambient quirk turns out to be load-bearing for matching Python's current pixel output (i.e., it's not actually a bug, just unusual), preserve its *effect* exactly when generalizing it into the material system — this task must not change a single output pixel.

## Scope

**This task touches `hairy_tracer_core` only — no Python changes.** Python's material handling (including the `--pure-python` fallback) stays exactly as-is; it's kept purely as a legacy comparison reference for the E2E pixel-diff tests, not as a path being developed further. Rust already has to match Python's output regardless of how either side is internally structured, so cleaning up Rust's material representation doesn't require touching Python's.

**Material struct/trait (Rust):**
- Design a `Material` type holding: ambient/diffuse/specular coefficients, specular exponent, reflectivity, transparency, and IOR — whatever set of fields the Python source actually uses, matched exactly
- Replace all hardcoded per-geometry material logic (including the ambient quirk) with material-driven values, preserving current behavior exactly — this is a refactor, not a behavior change
- Leave room for future fields (texture reference, roughness/metallic for later PBR work) but don't implement them now — an empty `Option<TextureRef>` or similar placeholder is fine, a working texture system is not in scope
- Attach a `Material` to each primitive (Sphere, Plane, CheckeredPlane, Triangle, mesh triangles) in whatever way fits the existing `Intersectable` trait design

**UV coordinates:**
- Extend the `Hit` struct with a UV coordinate field (u, v)
- Compute UV for each primitive's intersection:
  - Sphere: standard spherical mapping (longitude/latitude from the hit point relative to center)
  - Plane / checkered plane: planar mapping matching whatever tiling logic the checkered plane already uses for material selection — check whether UV can reuse that existing calculation
  - Triangle: barycentric interpolation of per-vertex UVs (if the mesh/OBJ data has them) or a reasonable default (e.g., barycentric coordinates themselves) if not
  - Mesh triangles: pull from parsed OBJ UV data if the parser already captures it; extend the parser if it currently discards UV data
- Nothing needs to *consume* UV yet — no texture sampling, no visual change. This phase is plumbing: correct UV values reaching the `Hit` struct, verified by unit tests, not by any pixel output changing

## Testing

- **Primary regression gate**: the existing E2E pixel-perfect tests (`test_btrace_rust.py` and the original snapshot tests) must still pass with zero pixel difference, on both the Rust and `--pure-python` paths. This is non-negotiable for this task — if the material refactor changes even one pixel, something was generalized incorrectly, not "close enough"
- New unit tests for the `Material` struct: default values match the previous hardcoded constants exactly
- New unit tests for UV: known ray/primitive combinations with hand-computed expected UV values — sphere at a pole and at the equator, a plane tile boundary, a triangle vertex and its centroid
- Run the full existing pytest suite, not just the new tests — confirm nothing upstream regressed

## Deliverable

- `Material` struct/type replacing the hardcoded per-geometry logic, with the ambient quirk's *effect* preserved exactly
- UV coordinates flowing into `Hit` for every primitive type
- A short note: confirmation E2E tests are still pixel-identical, what the ambient quirk turned out to be and how it was generalized, and whether the OBJ parser needed extending for UV data or already had it
