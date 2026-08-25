# Prompt: Example scenes for the new Material capabilities

## Context

The Material quality phase is complete: `use_fresnel` (opt-in Fresnel-Schlick blending), absorption color/coefficient for glass, procedural and image-mapped textures, and environment maps are all implemented, tested, and pixel-identical to the Python reference at default settings.

This task has two parts: first, report the specific numbers from the structural tests already written, and second, build a small set of example scenes that visually demonstrate each capability.

## Part 1 — Report the actual structural test results

For each of the five structural tests in `materials_tests.rs`, report the actual computed values, not just pass/fail:

- **Fresnel**: the computed reflectance at grazing incidence and at normal incidence, alongside the hand-computed Schlick values they were checked against — how close were they?
- **Absorption**: the actual transmitted-light values at each glass thickness tested — does the falloff look exponential (each doubling of thickness roughly squaring the transmittance, or whatever the correct relationship is), or just "less light at more thickness" without a clear curve?
- **Procedural texture**: the checker cell colors returned at the tested UV coordinates — did they match the expected cell exactly?
- **Image texture**: the sampled color at the tested UV coordinate versus the actual source pixel value it should correspond to
- **Environment map**: the sampled pixel for the tested ray direction versus the expected pixel in the source equirectangular image

## Part 2 — Example scenes

Build and render a small set of scenes, one capability isolated per scene (not one kitchen-sink scene where effects are hard to attribute) — save them somewhere like `examples/materials/` alongside their rendered output, since these are worth keeping as both documentation and a visual regression reference going forward:

1. **Fresnel on vs. off** — the same glass or mirror object, rendered twice (`use_fresnel: false` and `true`), so the difference is directly comparable
2. **Absorption thickness comparison** — two or three glass objects of different thickness or absorption coefficient side by side in one scene, so the falloff is visible in a single render
3. **Procedural texture** — an object (sphere or plane is fine) with a checker or other procedural pattern applied, chosen so the UV tiling is clearly visible
4. **Image texture** — an object with a real loaded image texture; reuse whatever test image already exists from the image-texture structural test if one's available, rather than sourcing a new one
5. **Environment map** — a scene with at least one reflective and one refractive object against an environment-mapped background, so reflections/refraction of the environment are visible

For each: the scene JSON, and the rendered PNG output.

## Deliverable

- Part 1's numbers, reported plainly (not just "tests passed")
- Five scene JSON files and their rendered PNG outputs, organized under `examples/materials/`
- A one-line caption per example noting what to look for in the image
