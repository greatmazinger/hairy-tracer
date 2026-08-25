# Prompt: Global Illumination phase — path tracing + PBR materials
## Context

Every phase so far — Foundation, Sampling, Material quality, Acceleration — has followed one discipline: new capability defaults to exactly reproducing existing behavior, opt-in only. That discipline can't apply the same way to full global illumination, because path-traced indirect lighting is a genuinely different image, not an additive toggle — a diffuse wall now scatters bounced light from its surroundings, which a Whitted-style renderer never computed at all. There's no "off" state that still looks like GI turned down.

**The fix that keeps the discipline intact**: move the opt-in up a level. Introduce an **integrator** abstraction — a swappable strategy for computing outgoing radiance at a hit point — and keep the current recursive Whitted-style shading (direct lighting, shadow rays, mirror/glass recursion) as the default integrator, untouched. Path tracing becomes a second integrator, selected explicitly per scene. Every existing scene keeps using Whitted by default, so the frozen baseline stays bit-for-bit exact without requiring GI itself to be non-disruptive.

## Before writing any code

- Read the current recursive shading/`trace_ray` implementation closely — this is what the Whitted integrator becomes, and where the integrator abstraction needs to slot in without changing its behavior
- Read the current `Light` struct (position, color, radius — from the Sampling phase's soft-shadow work) to determine how direct light sampling works inside the path tracer
- Read the current `Material` struct, especially the Fresnel-Schlick work from the Material quality phase — GGX/PBR reuses that Fresnel term rather than reimplementing it
- Decide explicitly (and document the reasoning) whether PBR/GGX materials are meaningful under the existing Whitted integrator too, or scoped as path-tracer-only — either is defensible, but it needs to be a stated decision, not an implicit one

## Scope

**1. Integrator abstraction**
- Extract the current recursive shading logic behind an integrator interface/trait, with the existing behavior as the `Whitted` (or similarly named) implementation — this step alone should be a pure refactor with zero pixel change, same regression standard as every phase before it
- Add a scene-level or render-level setting selecting the integrator, defaulting to `Whitted`

**2. Path tracing integrator**
- Cosine-weighted hemisphere sampling for diffuse bounce direction (importance sampling matched to a Lambertian BRDF)
- For mirror/glass materials, it's reasonable to reuse the existing deterministic reflect/refract directions rather than requiring full BSDF importance sampling machinery for every material type — a hybrid path tracer. Don't build a general BSDF-sampling framework unless the material system genuinely needs it; that's scope creep for this task.
- Direct light sampling (next-event estimation) at each bounce against the existing `Light` list — reuse the existing light representation rather than requiring emissive geometry, which would be a much larger scope
- Russian roulette termination based on path throughput, so recursion is unbiased rather than hard-capped in a way that introduces bias
- This reuses the `samples_per_pixel` infrastructure from the Sampling phase — note explicitly that GI needs meaningfully more samples than AA alone to converge to something visually clean; that's an expected cost, not a bug to chase

**3. GGX / Cook-Torrance PBR materials**
- Extend `Material` with roughness and metallic parameters
- Implement the GGX normal distribution function, Smith geometry term, and reuse the existing Schlick Fresnel term to assemble a microfacet BRDF
- Gate this behind an explicit material setting (e.g., a material "type" or a roughness/metallic pair only consulted when present) — existing materials without these fields render exactly as before under Phong

**Explicitly out of scope for this task** (real future work, not this phase): bidirectional path tracing, photon mapping or dedicated caustics beyond whatever naturally falls out of existing glass refraction plus GI, spectral rendering, denoising. Keep this task to unidirectional path tracing plus GGX materials.

## Testing

Given the BVH-phase lesson — a bug hid for a long time because correctness was only checked indirectly through a full rendered image — test the underlying math directly wherever possible, not just the final picture:

- **Regression gate**: with the integrator defaulting to `Whitted` and no material using roughness/metallic, every existing snapshot must remain bit-for-bit identical, same as always
- **Refactor-only correctness**: step 1 (extracting the integrator abstraction) must produce zero pixel change on its own, checked before path tracing is added on top — same "prove the foundation is solid before building on it" pattern as every earlier phase
- **PDF correctness**: verify the cosine-weighted sample PDF matches the analytical `cos(theta)/pi` directly, not just "the render looks diffuse"
- **Energy conservation**: numerically integrate the GGX BRDF over the hemisphere at a few roughness values and confirm it stays at or below 1 — a material shouldn't be able to reflect more light than it receives
- **Color bleeding (the classic GI proof)**: build a Cornell-box-style test scene (a box with one red wall, one green wall, white walls/floor/ceiling, a light on the ceiling) and render it with the path tracer — confirm visible color tinting on the white surfaces near the colored walls, which a Whitted-only renderer cannot produce. This is the single clearest piece of evidence indirect lighting is actually happening, not just that samples are being taken.
- **Convergence**: render the same GI scene at increasing `samples_per_pixel` and confirm variance between renders decreases — evidence the estimator is actually converging, not just producing different noise each time
- Run the full existing pytest suite to confirm nothing upstream regressed

## Deliverable

- Integrator abstraction with `Whitted` as the untouched default and a new path-tracing integrator
- GGX/Cook-Torrance PBR material support, reusing the existing Fresnel implementation
- Confirmation of the regression gate at default settings, and confirmation the integrator-extraction refactor alone was zero-pixel-change before path tracing was added
- The Cornell-box color-bleed scene and render, plus the PDF-correctness, energy-conservation, and convergence test results (actual numbers, not just pass/fail — same standard as the Material quality phase reporting)
- A render-time benchmark for the Cornell-box scene at a reasonable sample count, and a note on what sample count seems to give acceptable visual quality for a hobby renderer (not a production noise-floor target)
