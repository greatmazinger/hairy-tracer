# Prompt: Material quality phase — Fresnel, absorption, textures, environment maps
---

## Context

Foundation and Sampling phases are both complete and merged:
- `Material` struct with explicit JSON overrides (ambient color quirk preserved only as documented fallback), plus a placeholder texture field left for this phase
- UV coordinates flowing into `Hit` for every primitive (sphere spherical mapping, plane planar mapping, triangle/mesh barycentric — though the custom `.obj` parser currently discards `vt` texture coordinates, noted as future work for this phase)
- A shared per-sample sampling loop from the Sampling phase (AA, soft shadows, DOF), all defaulting to no-op and pixel-identical to the original Python reference at default settings
- Full E2E regression suite green at all default parameter values

This phase is mixed: **Fresnel weighting modifies an existing code path** (the current reflection/refraction blend, ported from Python during the original shading/recursion work), while **absorption, textures, and environment maps are entirely new** (nothing in the original Python engine has any of these). Treat them differently:
- Fresnel: must be gated behind an opt-in flag defaulting to whatever preserves the current fixed reflect/refract blend exactly — this one actually risks changing existing pixel output if not gated carefully, unlike the purely-additive features below
- Absorption/textures/environment maps: same no-op-by-default discipline as the Sampling phase — new fields default to values that reproduce current behavior (clear glass, solid material color, current fixed background color) until a scene explicitly opts in

Build order (each builds on infrastructure from the one before):

1. Fresnel-Schlick weighting (modifies existing glass/mirror blend — do this first and confirm the regression gate holds before anything else)
2. Beer-Lambert absorption (extends glass now that Fresnel's reflect/refract split is in place)
3. Procedural textures (checker/noise patterns — consumes existing UV data, no image loading needed yet)
4. Image-mapped textures (needs an image-loading dependency and the `.obj` parser extended to actually capture `vt` coordinates)
5. Environment maps (reuses the image-loading infrastructure built for step 4, applied to the camera-miss/background case instead of a surface)

## Before writing any code

- Find the current reflection/refraction blend logic — is it a fixed ratio, or something else? This is what Fresnel replaces, gated behind the opt-in flag.
- Find the current miss/background handling — what happens today when a ray hits nothing? This is what environment maps override.
- Check what the Foundation-phase placeholder texture field on `Material` actually looks like — build on it rather than replacing it if it's already a reasonable shape.
- Confirm exactly what the `.obj` parser currently does with `vt` lines (discards entirely, or reads and drops them?) before extending it.
- Decide on an image-loading crate (the `image` crate is the standard choice for PNG/JPEG; note separately if you want HDR support for environment maps, since that typically needs a different loader).

## Scope

**1. Fresnel-Schlick weighting**
- Add a material-level flag (e.g. `use_fresnel`, default matching current behavior — off) gating this
- When off: identical to current fixed-ratio blend — this is the regression-critical case
- When on: Schlick's approximation determines the reflect/refract split based on the angle of incidence and the material's IOR, replacing the fixed ratio

**2. Beer-Lambert absorption**
- Add an absorption color/coefficient to `Material`, defaulting to `0.0` (fully clear, matching current glass behavior exactly)
- When non-zero, attenuate transmitted light exponentially with distance traveled through the medium, tinting by the absorption color

**3. Procedural textures**
- Extend the `Material` texture field to support at least one procedural pattern (checker is the simplest, reusing the tiling logic that likely already exists for `CheckeredPlane`)
- Sample using the UV coordinates already in `Hit` — no new geometry-side work needed
- Default: no texture assigned, material uses its solid diffuse/ambient color exactly as today

**4. Image-mapped textures**
- Add image loading and UV-based sampling (nearest or bilinear — bilinear is worth the small extra cost)
- Extend the `.obj` parser to actually capture and store `vt` coordinates instead of discarding them
- Default: no image assigned, same solid-color fallback as procedural textures

**5. Environment maps**
- Add an optional scene-level environment map, sampled by ray direction (equirectangular mapping is the standard approach) when a ray hits nothing
- Default: no environment map assigned, background is the current fixed color, unchanged

## Testing

- **Regression gate**: with `use_fresnel` off, absorption at `0.0`, no textures assigned, and no environment map, every existing E2E snapshot must still pass with zero pixel difference — same standard as every phase before this one
- Structural tests (no Python reference exists for any of these):
  - Fresnel: reflectance at grazing incidence should be near total, at normal incidence should match the material's base reflectivity — check against hand-computed Schlick values at a couple of known angles
  - Absorption: a thicker slab of colored glass should transmit measurably less light than a thin one, with the correct color tint
  - Procedural texture: sample points at known UV coordinates should return the expected checker cell color
  - Image texture: sample a known test image at known UV coordinates and confirm the returned color matches the source pixel
  - Environment map: a ray in a known direction should sample the expected pixel from a known test equirectangular image
- Run the full existing pytest suite to confirm nothing upstream regressed

## Deliverable

- Fresnel weighting (opt-in, gated), Beer-Lambert absorption, procedural textures, image textures, and environment maps, all implemented per the above
- Confirmation the E2E regression suite is still green at default settings
- A short note on: the image-loading crate chosen, whether HDR support was included for environment maps or deferred, and what the `.obj` parser extension for `vt` coordinates looked like
