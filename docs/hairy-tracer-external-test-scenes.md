# External test models for Hairy Tracer

Reference list of freely available `.obj` models, organized by what each is good for testing. Sources chosen for permissive licensing and native OBJ format.

## Sources

- **[McGuire Computer Graphics Archive](https://casual-effects.com/data/)** — the standard clearinghouse for free, research-use 3D models. Packaged as OBJ + PNG specifically, so texture files are ready to use as-is. Hosts most of the classics below (Bunny, Dragon, Buddha, Sponza, Teapot).
- **[common-3d-test-models (GitHub)](https://github.com/alecjacobson/common-3d-test-models)** — bare `.obj` files, no zip/texture bundling. Good for quick single-file tests.

## By purpose

### Quick sanity check
- **Suzanne** (Blender monkey) — ~968 triangles. Fast to render, good first check that an OBJ imports and shades correctly before moving to anything bigger.

### BVH / mesh stress testing
- **Stanford Bunny** — ~70K triangles. The classic mid-size stress case for acceleration structures.
- **Stanford Dragon** / **Happy Buddha** — considerably higher triangle counts, for pushing well past the ~7,200-triangle synthetic stress mesh already in the test suite.

### Texture testing
- Any McGuire archive model with an accompanying `.mtl` + PNG. This is genuinely new ground for the engine — the OBJ parser only started capturing `vt` coordinates during the Materials phase, so a real textured model is the first proper test of that path (beyond the synthetic 2×2 test buffer in the unit tests).

### Global illumination
- **Crytek Sponza** (McGuire archive) — the de facto standard scene for GI testing. An enclosed atrium with columns and multiple rooms, showing off indirect bounce lighting and color bleed far more dramatically than a single Cornell box. Best tried after the Cornell-box color-bleed test has already confirmed correctness — Sponza is a much bigger scene, better suited as a "does this hold up at scale" follow-up than a first check.

## Compatibility note

Worth checking whether the engine currently uses per-vertex normals (`vn`) from the file for smooth shading, or always computes flat face normals. Most of the models above ship with smooth normal data — if `vn` is being ignored, they'll render noticeably faceted, which a synthetic test triangle wouldn't reveal but a real model will.
