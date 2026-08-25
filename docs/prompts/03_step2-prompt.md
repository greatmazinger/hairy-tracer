# Prompt: Shading, camera, recursion & rayon tiling for hairy_tracer_core
---

## Context

`hairy_tracer_core` currently contains only geometric intersection: primitives (Sphere, Plane, Checkered Plane, Triangle, Triangle Mesh), an AABB accelerator (slab method), `Ray`/`Hit`/`Intersectable`, and a `Scene::trace_ray` that returns the closest geometric hit. No color, lighting, shading, camera, recursion, threading, or file parsing exists yet.

Revised build order (shading/camera/recursion were missing from the original 3-step plan — this fills the gap before PyO3 binding, which comes after this task, not during it):

1. Shading — Point light + Phong illumination (ambient/diffuse/specular)
2. Recursion — shadow rays, mirror reflection, glass refraction (Snell's law + IOR)
3. Camera — look-at basis vectors, generating a `Ray` from (x, y) pixel coordinates
4. Rayon — tile-based parallel render loop across CPU cores
5. *(separate future task, not this one)* PyO3 binding exposing one coarse `render_image()` call

This task is steps 1–4. Do not add PyO3, `cdylib` config, or any Python-facing code in this task — that's the next prompt, after this is validated.

## Before writing any code

Read the existing Python source for the exact current behavior — don't approximate or re-derive from general Phong/Whitted-raytracer knowledge. Specifically:
- Ambient/diffuse/specular coefficients and how they combine, and the specular exponent handling
- The shadow-ray bias epsilon (the small offset from a hit point to avoid self-shadowing/"shadow acne") — get the actual constant, not a guessed one
- Max recursion depth for reflection and refraction, and what happens at the depth limit (return black? return the surface color unlit? something else?)
- IOR values used for glass and how Snell's law / Fresnel is currently computed
- The look-at camera math: up-vector convention, FOV-to-viewport-height conversion, and the "Dynamic Viewport Scaling" behavior mentioned in the original feature list (recalculating `vpheight` from output image dimensions)
- Background/miss color when a ray hits nothing

If any of these can't be found or are ambiguous, ask rather than guess — a mismatch here won't crash anything, it'll just silently produce different pixels, which defeats the pixel-diff validation planned for the PyO3 step.

## Scope

**Shading (step 1):**
- Point light struct: position, color/intensity
- Phong illumination combining ambient + diffuse + specular against the existing `Hit` normal/point data
- Support multiple lights in a scene (matching whatever the Python does — sum contributions, presumably)

**Recursion (step 2):**
- Shadow rays: cast from hit point toward each light, using `Scene::trace_ray` (or an equivalent occlusion-only variant if that's more efficient) to check for blockers; respect the bias epsilon found above
- Mirror reflection: reflect vector off the surface normal, recurse up to the max depth
- Glass refraction: Snell's law + IOR, recurse up to the max depth; this and reflection likely need to combine (Fresnel-weighted, or whatever mix the Python uses — check it, don't assume 50/50)

**Camera (step 3):**
- Look-at basis construction (eye, target, up → right/up/forward vectors)
- Pixel (x, y) + image width/height → world-space ray, including the dynamic viewport-height recalculation from the feature list

**Rayon tiling (step 4) — build and validate steps 1–3 serially first:**
- Only after shading/recursion/camera are correct single-threaded (see Testing below), wrap the per-pixel render in a `rayon`-parallel tile loop (32×32 tiles or horizontal strips — either is fine, strips are simpler)
- Scene data accessed read-only (`&Scene`) across threads; output framebuffer split via `chunks_mut` into disjoint per-thread regions — no locks needed if the split is truly disjoint

**Out of scope for this task:** PyO3, `cdylib`/`pyo3` in `Cargo.toml`, JSON scene parsing, OBJ parsing (mesh data can still be handed in as a flat vertex/index buffer as before).

## Testing

- Unit tests per addition: a lit vs. shadowed point gives different output; a mirror surface reflects a known ray correctly; refraction at normal incidence doesn't bend (sanity check); recursion actually terminates at max depth instead of stack-overflowing
- Before adding rayon: render at least one full small scene (reuse or port one of the existing Python test scenes) serially and manually compare against the Python output for that same scene — this is the "trusted serial baseline" the rayon layer gets checked against next, so it needs to be right first
- After adding rayon: render the same scene again and diff pixel-for-pixel against the serial-Rust output from the step above. They must match exactly (or within an explicitly justified floating-point tolerance) — a mismatch here means the tile split isn't actually disjoint, or there's a race, and that needs to be caught now rather than surfacing later as a mysterious pytest snapshot failure

## Deliverable

- Shading, recursion, camera, and rayon-tiled render loop added to `hairy_tracer_core`
- Unit tests per the above
- A short note confirming: serial-vs-Python comparison result, serial-vs-rayon comparison result, and any Python behavior that was ambiguous or had to be inferred rather than read directly from source
