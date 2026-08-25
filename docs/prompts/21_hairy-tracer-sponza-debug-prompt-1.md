# Prompt: Diagnose Sponza's lack of spatial coherence (not noise)

## Context

The Sponza render (`sponza_fixed.bmp`, 300×300, 64 SPP) was reported as "noise-free," but the actual image shows **zero spatial structure** — no edges, no floor line, no column or arch silhouettes, just uniformly speckled color across the entire frame with no coherence between neighboring pixels. This is not Monte Carlo noise: real photon noise, even badly under-sampled, still shows a recognizable underlying image. The speckle colors do look like genuine Sponza texture tones (warm browns/tans), which means meshes and textures are loading — the problem is that pixel position isn't correlating with scene content at all.

**Do not attempt further noise-reduction fixes (more samples, different light types, roughness changes) until this is diagnosed** — the previous round of fixes (point lights, forcing roughness to 1.0) targeted convergence noise, which isn't what this image actually shows, and re-rendering with more samples will not fix a lack of spatial coherence.

## Isolation steps — do these in order, don't skip ahead to fixing

**1. Render with the Whitted integrator instead of the path tracer**, same scene, same camera. Whitted has none of the Monte Carlo machinery, so if it also produces incoherent static, that rules out anything path-tracer-specific (importance sampling, MIS, etc.) and points at ray generation, mesh loading, or BVH/object resolution instead.

**2. Render a drastically reduced version of the scene** — just `sponza_floor.obj` and one column mesh, with a single simple point light, same camera position. If this small scene renders as a recognizable (if simple) image while the full 25-mesh scene doesn't, that implicates something specific to handling many discrete mesh objects in one scene — this is the first scene in the project with more than one or two mesh objects, each carrying its own material, so that configuration itself has never been exercised before. If even this minimal scene shows the same incoherent static, the problem is more fundamental — likely camera/ray generation or something scale-related (Sponza's coordinate scale, ~2000+ units, is far larger than any prior test scene).

**3. Log or dump primary ray directions for a small handful of pixels** (e.g., the four corners and the center) and manually verify each one is a sensible direction given the camera's `origin`/`look_at`/`up` — confirm ray direction is actually varying correctly and continuously across the image plane as (x, y) changes, rather than being constant, randomized, or otherwise decorrelated from pixel position.

## Based on what the isolation steps show

- If Whitted also fails: the bug is in ray generation, mesh loading, or BVH traversal — not the path tracer specifically. Focus on camera ray generation math and whether it behaves correctly at this scene's coordinate scale.
- If Whitted works but path tracer doesn't: something in the path tracer's handling of many separate mesh objects (material/texture resolution per hit, most likely) is the target.
- If the reduced scene works but the full scene doesn't: the bug is specifically triggered by having many mesh objects in one scene — check whether material/texture lookup is correctly scoped per-object rather than using some shared/global state that leaks across objects.
- If even the reduced scene fails: it's more fundamental — check ray generation and camera math directly against this scene's scale and parameters.

## Deliverable

- Results from all three isolation steps, reported plainly (which showed structure, which didn't)
- The actual root cause, once narrowed down by the above — not another attempt at noise reduction
- A corrected render showing recognizable Sponza geometry (columns, floor, arches visible), even if still grainy from a low sample count — grain is fine at this stage, incoherence is not
