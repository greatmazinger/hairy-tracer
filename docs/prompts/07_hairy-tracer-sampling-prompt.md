# Prompt: Sampling phase — anti-aliasing, soft shadows, depth of field
---

## Context

Foundation phase is complete: `hairy_tracer_core` now has a proper `Material` struct (ambient color explicitly overridable via JSON, with the old geometry-type quirk preserved only as a documented fallback), UV coordinates flowing into `Hit` for every primitive, and full end-to-end parser tests covering the fallback/override behavior. The E2E pixel-perfect tests against the Python reference are green.

This phase is different from every phase before it in one important way: **anti-aliasing, soft shadows, and depth of field don't exist in the original Python engine at all.** There's no Python behavior to read and match here — these are genuinely new features. That changes what "correct" means for this task: instead of matching an existing reference, the goal is that every new feature must default to being a no-op that exactly reproduces current behavior, so existing scenes and existing E2E snapshots keep passing unchanged. New visual behavior only appears when a scene opts in via new parameters.

Planned order within this phase (build and test each before starting the next — they layer on the same per-pixel sampling loop):

1. Anti-aliasing — the foundational piece; soft shadows and DOF both extend its sampling loop rather than building their own
2. Soft shadows
3. Depth of field

## Before writing any code

Inspect the current render loop, camera, and light representation before designing anything:
- Where does the per-pixel ray generation currently happen, and how does it interface with the `rayon` tile loop? This is the loop that needs to become "N samples per pixel" instead of "1 ray per pixel."
- What's the current `Light` representation — position, color/intensity, anything else? Confirm it's a point light with no size/radius concept yet.
- What's the current camera struct — eye/target/up, FOV, viewport — and where exactly is a primary ray constructed from (x, y)? This is where DOF will need to jitter the ray origin.
- Is there already an RNG dependency in the crate, or does one need adding (the `rand` crate is the standard choice)?

## Scope

**1. Anti-aliasing**
- Add a `samples_per_pixel` parameter (scene-level or render-call-level — pick whichever fits the existing config shape) defaulting to `1`
- At `samples_per_pixel = 1`, behavior must be bit-for-bit identical to the current single-sample render — no jitter, same ray as today. This is what keeps the existing E2E snapshots valid without modification.
- At `samples_per_pixel > 1`, jitter the sample position within the pixel (stratified or random, your choice) and average the resulting colors
- This introduces the shared sampling loop — structure it so soft shadows and DOF can hook into the same per-sample iteration rather than each adding their own separate loop

**2. Soft shadows**
- Extend `Light` with an optional radius (defaulting to `0.0` — a radius-0 light is a point light, identical to current behavior)
- When radius is `0.0`, shadow ray casting is unchanged from today
- When radius is `> 0.0`, sample multiple points across the light's area (using the per-sample loop from step 1 — one shadow-ray sample point per pixel-sample, not a separate inner loop) and average occlusion
- This means soft shadows only actually show softness when `samples_per_pixel > 1` — document that relationship clearly, since a radius-5 light at `samples_per_pixel = 1` would just pick one random point on the light per pixel, which is noisy rather than soft

**3. Depth of field**
- Extend the camera with `aperture` (lens radius, default `0.0`) and `focal_distance` (default matching whatever makes the pinhole-equivalent behavior fall out naturally — check the math rather than guessing)
- At `aperture = 0.0`, ray generation is unchanged — identical pinhole camera behavior
- At `aperture > 0.0`, jitter the ray origin across a disk of that radius (again, per-sample from step 1) and aim through the same point on the focal plane, so all samples for a pixel converge there

## Testing

- **Regression gate, same as Foundation**: with all new parameters at their defaults (`samples_per_pixel = 1`, light radius `0.0`, aperture `0.0`), every existing E2E snapshot test must still pass with zero pixel difference. This is the thing that proves the defaults are truly no-ops, not approximately so.
- New tests can't be pixel-diff against a Python reference (none exists for these features), so validate structurally instead:
  - AA: render a high-contrast edge at `samples_per_pixel = 1` vs. `> 1` and confirm the higher-sample version has intermediate colors at the edge (evidence of actual averaging, not just more noise)
  - Soft shadows: render a scene with a radius `> 0` light and confirm a penumbra region exists (partial occlusion values between fully lit and fully shadowed) where a point light would give a hard binary edge
  - DOF: render a scene with `aperture > 0` and confirm points at the focal distance stay sharp (low variance across samples) while points off the focal plane blur (higher variance)
- Run the full existing pytest suite to confirm nothing upstream regressed

## Deliverable

- `samples_per_pixel`, light radius, and camera aperture/focal_distance added with true no-op defaults
- The shared per-sample loop that AA, soft shadows, and DOF all hook into
- Structural tests per the above, plus confirmation the full E2E regression suite is still green at default parameters
- A short note on any scene JSON schema additions (new optional fields, all defaulting to current behavior) needed to expose these parameters
