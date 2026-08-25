# Prompt: Fix NEE fireflies on near-specular (low-roughness) materials
## Context

The gold bunny render shows the blocky triangle-facet bug is fully resolved (smooth shading confirmed) and real color bleeding is now visible — both genuine wins. But there's colored, per-pixel speckle concentrated specifically on the bunny (a near-specular/low-roughness metallic material), distinct from the milder, expected grayish Monte Carlo grain on the diffuse walls and floor.

This is the classic NEE-on-near-specular-materials failure mode: for a tight specular lobe, a shadow ray aimed at the light almost always finds the BRDF value near zero (the light essentially never sits inside a narrow lobe by chance), but on the rare pixel where it does align, the BRDF value or MIS weight spikes to compensate for the near-zero probability — producing exactly the colored fireflies seen here, concentrated on the shiny surface.

**First, confirm the diagnosis** before fixing anything: check whether speckle severity actually tracks with the material's roughness value — render the same scene with a rougher variant of the material and confirm the speckle is substantially reduced. If it doesn't correlate with roughness, this isn't the right diagnosis and needs rethinking before proceeding.

## Scope (assuming the diagnosis holds)

Two standard, complementary remedies — both are common in production path tracers, and combining them is reasonable:

**1. Roughness-based NEE skip**
- Below some small roughness threshold, skip NEE entirely for that material and rely purely on BSDF sampling for direct light contribution
- This is the standard practical fix: NEE's contribution becomes negligible for near-mirror surfaces anyway, while its variance cost is high — BSDF sampling naturally finds the light-aligned direction more efficiently for tight lobes
- Pick a threshold that's easy to tune (a constant is fine to start with, doesn't need to be adaptive)

**2. Firefly clamping**
- Cap the maximum radiance contribution any single sample can add during accumulation
- This is a soft, deliberate bias in exchange for much faster visual convergence — used pragmatically by nearly every production path tracer. Apply it carefully: too aggressive a clamp will clip legitimate bright highlights (the actual specular highlights this whole NEE-specular effort was trying to make sharp), so this should reduce extreme outliers, not flatten real highlights
- Worth checking whether energy conservation is still upheld in the *unclamped* math (this is a display/convergence aid, not a correctness fix for the underlying BRDF)

## Testing

- Re-render the gold bunny scene and confirm the colored speckle is substantially reduced, without visibly softening the legitimate specular highlights that NEE-specular was added to sharpen in the first place
- Render the same scene at a couple of roughness values and confirm the fix behaves sensibly across the range (doesn't harm convergence on rougher materials that weren't the problem to begin with)
- Regression gate as always: existing snapshot suite unaffected

## Deliverable

- Confirmation (or refutation) of the roughness-correlation diagnosis, with the comparison render
- Roughness-based NEE skip and firefly clamping implemented, with the threshold/clamp values chosen and a brief note on why
- A clean re-render of the gold bunny scene
- Confirmation of the regression gate
