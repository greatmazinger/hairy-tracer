## Context

I have a Python raytracer called "Hairy Tracer" with:
- Geometry: infinite planes, procedural checkered planes, spheres, triangles (Möller–Trumbore intersection, with automatic back-face normal flipping), and triangle meshes loaded from Wavefront `.obj` files (with triangle-fan auto-triangulation for quads/n-gons)
- An AABB acceleration structure wrapping each `.obj` mesh, using the slab method, to skip geometry when a ray misses the bounding box
- Phong shading, hard shadows, mirror reflection, and glass refraction (Snell's law + IOR) — **not in scope for this task**
- A `pytest` unit test suite covering vector math, reflection, refraction, and intersection boundaries, plus an E2E snapshot test comparing a rendered 10×10 image against a stored pixel array

I'm porting the performance-critical hot path to Rust, to be called from Python later via PyO3. This is step 1 of 3:

1. **This task** — intersection + BVH traversal in Rust, correct and unit-tested in isolation. No Python bindings, no threading/rayon yet.
2. (later) Wrap it in a `rayon`-based tile-parallel render loop.
3. (later) Expose the whole thing as one coarse PyO3-callable `render_image()` function.

Do not build steps 2 or 3 — just the foundation they'll sit on.

## Goal

A standalone Rust crate/module that reproduces the geometric behavior of the Python raytracer's intersection logic, structured so a `rayon`-parallel render loop can later call a single `trace_ray` (or similar) entry point per pixel without needing to know about threading itself.

## Before writing any code

Read the existing Python source for the exact current behavior — don't assume. In particular, check and match:
- The epsilon/tolerance used in Möller–Trumbore and slab-method intersection tests (avoid re-deriving different constants)
- The exact back-face normal-flipping rule for triangles
- How the checkered plane picks which of its two materials applies at a given point (the tiling/modulo logic)
- The AABB slab-method implementation details (how t_min/t_max are tracked and compared)
- The `.obj` triangle-fan auto-triangulation ordering (winding order matters for normals)

If any of this can't be found or is ambiguous in the Python code, ask me rather than guessing.

## Scope

**Geometry types**, each implementing a shared intersection trait:
- Sphere
- Infinite plane
- Checkered plane (two materials, tiled)
- Triangle (Möller–Trumbore, back-face normal flip)
- Triangle mesh (collection of triangles loaded conceptually from parsed OBJ data — assume the mesh is handed in as a flat vertex/index buffer; don't reimplement the OBJ parser itself in Rust yet)

**Acceleration structure:**
- AABB (slab method) wrapping a mesh's triangles
- Structure this so it can be swapped for a proper BVH later without changing the trait/call interface — a single top-level AABB per mesh (matching the current Python behavior) is fine for now, but note in a comment where a recursive BVH would slot in

**Ray & hit types:**
- A `Ray` struct (origin, direction)
- A `Hit` (or `Intersection`) struct: `t`, hit point, surface normal, and a reference/index to which object and material was hit — no shading math, just what a shader would need later
- A top-level scene intersection function: given a `Ray` and a list of scene objects, return the closest `Hit` (or `None`)

**Explicitly out of scope for this task:** Phong shading, shadow rays, reflection/refraction, camera/viewport code, rayon/threading, PyO3 bindings, JSON scene parsing.

## Implementation notes

- Use `glam` for vector/matrix math (fast, simple, common choice for graphics code) unless you have a reason to prefer `nalgebra` — either is fine, just be consistent and say which you picked.
- Design the intersection trait so it's `Send + Sync` from the start, even though nothing is threaded yet — this avoids rework when rayon is added in step 2.
- Keep the module free of any Python/FFI dependency (no `pyo3` imports) — it should build and run as a plain Rust crate on its own.

## Testing

Write Rust unit tests (not pytest) covering:
- Each primitive type: a ray that clearly hits, clearly misses, and grazes/edge-cases (tangent to a sphere, parallel to a plane, hitting a triangle edge/vertex)
- Back-face triangle hits produce a flipped normal
- AABB rejects rays that miss the box before doing any per-triangle work (test this is actually happening, not just that the final answer is correct)
- At least one test with a small hand-built mesh (a few triangles) checking that the closest hit among multiple candidates is returned, not just the first

Where a numeric expected value matters (hit point, normal, t), compute or state the expected value explicitly in the test rather than asserting against whatever the code happens to produce.

## Deliverable

- The Rust module/crate with the above
- A short summary of any design decisions you made where the Python source was ambiguous or you deviated from it, and why
