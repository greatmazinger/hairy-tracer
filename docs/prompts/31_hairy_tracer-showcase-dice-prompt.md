# Prompt: Showcase die — flagship render for README/article
## Context

This is the flagship image — the one representing the project on the GitHub README and in the article. It should visibly demonstrate several capabilities at once, not just be "correct CSG": painted pips (exercising CSG material recovery, not just carving), path-traced GI, a studio environment map, shallow depth of field, and a glossy GGX material with a real Fresnel highlight.

Author this scene in the **scene DSL**, not raw JSON — both because it's the right tool now and because a clean, short DSL source alongside a beautiful render is good supporting material in itself.

## Design

**Geometry**: a cube (die body) with pip spheres subtracted from each face — shallow spherical caps, not deep gouges. The pip spheres should carry their own distinct material (see below) so the CSG boundary correctly exposes that material where subtracted, not the cube's.

**Pip layout** (standard, precise — getting this wrong would be immediately obvious to anyone who's seen a die):
- 1: center
- 2: two opposite corners (diagonal)
- 3: those two corners plus center
- 4: all four corners
- 5: four corners plus center
- 6: two columns of three

Opposite faces should sum to 7 (1↔6, 2↔5, 3↔4). Choose a camera angle showing three visually distinct faces (e.g. 1, 3, 6) rather than three similar-looking ones.

**Materials**: die body a saturated, glossy color (classic casino red or a deep ivory/cream) with moderate-low roughness GGX for a resin/plastic look; pips a strongly contrasting color (white on red, or black/red on ivory) — this is the detail that specifically shows off CSG material recovery, so make sure it's actually visible and correct, not just technically present.

**Lighting/environment**: a soft key light (real radius, not a hard point light), a dimmer fill, and either a rim light or reliance on the environment map for edge definition. A studio-style environment map (simple gradient is fine) for the glossy surface to reflect, plus a floor plane for a grounded contact shadow.

**Camera**: a 3/4 product-shot angle, with shallow depth of field — die in crisp focus, floor/background gently soft. Use the `fov_degrees` field, and sanity-check the resulting frame isn't a repeat of the earlier FOV mistakes (verify the object fills a reasonable portion of frame before committing to a full-quality render).

**Integrator**: path tracer, for real GI and glossy environment reflection.

## On the DSL's current limitations

The DSL doesn't yet support user-defined parametrized functions — each face's pip placement needs to be written out individually rather than expressed once and reused across faces. Author it this way for now; if it turns out to be genuinely painful (a lot of repeated per-face boilerplate), that's a legitimate, real-usage-motivated case for adding simple functions to the DSL as a follow-up — note whether this friction actually materializes, but don't build the function feature speculatively as part of this task.

## Testing

- Smoke-test at low resolution/samples first, same discipline as every other complex scene in this project — confirm pip patterns are geometrically correct (right count, right positions) and materials resolve correctly before committing to a full-quality render
- Confirm the painted-pip material recovery actually looks right (contrasting color visible and correctly bounded at the CSG boundary, no bleed or wrong-material patches)
- Full-quality final render once the smoke test checks out

## Deliverable

- The die scene authored in the DSL, plus the compiled JSON and the final high-quality render
- A note on whether the lack of DSL functions caused real friction worth addressing later
- Confirmation the pip layout is geometrically correct on all six faces
