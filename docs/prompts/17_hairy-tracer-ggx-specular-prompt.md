# Prompt: GGX specular in next-event estimation (with MIS)
## Context

NEE currently only evaluates the diffuse material term when sampling direct light. Specular/GGX highlights on glossy or metallic materials only appear when an indirect BSDF-sampled bounce ray happens to hit a light by chance — which is slow to converge and is the source of the remaining stippling on the gold bunny. Adding GGX evaluation to the NEE loop is the right fix — it makes direct specular lighting resolve cleanly on the first pass instead of relying on luck.

**This introduces a double-counting risk that needs to be handled explicitly.** Once NEE can deliver a light's specular contribution directly, there are now two paths by which that same contribution can reach a pixel: the new NEE specular sample, and an indirect BSDF-sampled ray that happens to hit the light directly (the existing "lucky bounce" mechanism). Without combining these two estimators correctly, the light's specular contribution gets counted twice — not obviously wrong visually, just quietly too-bright specular highlights, in a way the existing per-BRDF energy-conservation tests wouldn't catch, since those test the BRDF in isolation rather than the full combined estimator.

## Scope

- Add GGX (and existing specular/Fresnel) evaluation to the NEE direct-lighting loop, alongside the existing diffuse evaluation
- **Implement multiple importance sampling (MIS)** — the power heuristic is the standard choice — to combine the NEE light-sampling estimator and the existing BSDF-sampling estimator, so a light's contribution via either path is weighted correctly rather than summed outright
- This means the indirect/BSDF-sampled path also needs its own PDF (with respect to solid angle) computed for whichever light it happens to hit, so MIS weights can be calculated on both sides

## Testing

- **Double-counting check**: construct a simple scene with a single light directly visible via a mirror or near-mirror surface, and confirm the resulting brightness matches the physically expected value (computable by hand for a simple case) rather than being systematically brighter than expected — this is the direct test for the double-counting risk, not just "does it look plausible"
- Re-render the gold bunny scene at the current `samples_per_pixel` setting and confirm the specular highlights are now clean and resolve without needing a sample count increase
- Regression gate as always: existing snapshot suite unaffected, since this only changes path-tracer NEE behavior, not Whitted
- Re-run the existing GGX energy-conservation tests to confirm they still hold — they should be unaffected by this change since they test the BRDF in isolation, but worth confirming explicitly given what this change touches

## Deliverable

- GGX specular evaluation added to NEE, combined with the existing BSDF-sampling path via MIS (power heuristic)
- The double-counting verification test and its result
- A re-rendered gold bunny scene showing resolved specular highlights
- Confirmation of the regression gate and existing energy-conservation tests
