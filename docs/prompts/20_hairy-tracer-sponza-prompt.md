# Prompt: Set up and render Sponza
## Context

Sponza (Crytek/McGuire archive version, `.obj` + `.mtl` + texture files) is the next real-world test — a large architectural scene, ~260K triangles, with real per-surface textured materials. This is a meaningfully bigger step than the Stanford Bunny test: Sponza ships with a `.mtl` file (Wavefront Material Template Library), and nothing in this codebase currently reads one — every scene so far has used hand-authored materials in this project's own JSON schema. This task needs to bridge that gap, not just drop the mesh in.

## Before writing any code

- Download the Sponza package and inspect its `.mtl` file: how many distinct materials, which reference texture maps, which (if any) use an alpha/opacity map for cutout transparency (foliage is the classic case)
- Confirm the current OBJ loading pipeline's exact relationship to `.mtl` — does it read it at all currently, or has every prior scene bypassed it entirely with hand-authored JSON materials? Establish this directly rather than assuming.
- Check whether the engine has any alpha-cutout/transparency-via-texture-alpha capability. It almost certainly doesn't (nothing in this project has needed it before) — if so, decide explicitly: render foliage as opaque geometry for this first attempt (a documented known limitation) or exclude foliage submeshes entirely. Either is fine; don't let it be a silent surprise in the render.

## Scope

**1. Material conversion**
- Write a one-time conversion script (Python or Rust, whichever's more convenient) translating Sponza's `.mtl` materials into this project's `MaterialJson` schema
- Map diffuse color/texture directly; estimate reasonable roughness/metallic values from the old Phong/specular data (this is a standard, inherently approximate "legacy material to PBR" conversion — doesn't need to be exact)
- Wire texture file paths correctly relative to wherever the Sponza asset folder lands (following the flat `models/` asset convention from the recent reorg — e.g. `models/external/sponza/`)

**2. Scene authoring**
- Camera: needs real positioning/framing for the atrium's actual scale — don't reuse a small test scene's camera settings as-is
- Lighting: the file has none. An environment map for outdoor light through the archways is a natural fit and reuses existing capability; a light source works too if simpler to get right first
- Integrator: this scene should live in the `path_trace/pbr` tier per the current taxonomy, since it's meant to exercise the full pipeline

**3. Smoke test before full render**
- Render first at low resolution and low `samples_per_pixel` — the goal is confirming the scene loads, materials/textures resolve (not falling back to a default gray material everywhere), camera framing is reasonable, and BVH construction completes without pathological slowness
- Report BVH construction time specifically, separate from render time — 260K triangles is meaningfully more than anything tested before, worth knowing if construction itself becomes a bottleneck

## Testing

- Review the smoke-test render for obvious problems (missing textures, wrong framing, broken lighting) before scaling up to a real render
- Once the smoke test looks right, render at a real resolution/sample count
- Document any known limitations discovered along the way (e.g., "foliage renders opaque, no alpha-cutout support") explicitly rather than leaving them unexplained in the output
- This is a new scene, not a change to existing ones — no regression risk to the current snapshot suite, no need to touch it

## Deliverable

- Sponza assets placed under `models/`, conversion script and generated materials
- Authored scene JSON with camera and lighting, filed under `scenes/path_trace/pbr/`
- Smoke-test render plus BVH construction timing
- Full-quality render once the smoke test checks out
- A short list of any known limitations or gaps discovered (alpha-cutout foliage or otherwise)
