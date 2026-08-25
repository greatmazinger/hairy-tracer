# Prompt: Dynamic vertex normal generation for meshes without `vn` data
## Context

The blotching turned out not to be a bug: `bunny.obj` has no `vn` data, so the engine correctly falls back to flat per-face normals, and a highly specular gold material under flat shading in the path tracer produces a faceted "disco ball" effect — each triangle acts as its own small mirror, reflecting whichever part of the room (red wall, green wall) it happens to face.

Decision: implement **dynamic normal generation at load time** for meshes lacking `vn` data, rather than pre-baking a separate smoothed copy of this one file. This is a general capability — most external `.obj` files pulled from other sources won't include `vn`, so this fixes the gap for every future mesh, not just this one.

## Scope

- At OBJ load time, if a face has no `vn` indices (or the file has no `vn` data at all), generate smooth vertex normals instead of falling back to flat shading:
  - For each vertex, find all triangles that share it
  - Average their geometric face normals, **weighted by the angle each triangle subtends at that vertex** (not a plain unweighted average, and not area-weighted) — this is the standard, most correct approach and handles irregular tessellation (like this bunny's uneven 5,000-triangle mesh) meaningfully better than a naive average
  - Normalize the result
- Feed these generated normals into the same interpolation path already built for real `vn` data — no separate code path needed downstream of generation
- **Only kicks in when `vn` is genuinely absent.** If a mesh does have real `vn` data (including cases where an artist intentionally duplicated vertices to preserve hard edges), that data must be used as-is and never overridden by generated normals

## Testing

- Re-render the bunny scene: the faceted mirror-patch effect should be gone, replaced by specular highlights that sweep smoothly across the surface
- Unit test: build a small hand-constructed mesh with known triangle geometry (no `vn` data), compute the expected angle-weighted normal at a shared vertex by hand, and confirm the generated normal matches
- Regression gate as always: existing snapshot suite unaffected — none of the current test meshes should be affected by this change unless they already lacked `vn`, in which case confirm deliberately whether their appearance changes and whether that's expected

## Deliverable

- Angle-weighted normal generation for meshes without `vn` data, feeding into the existing interpolation path
- A clean re-render of the bunny showing smooth shading
- The unit test confirming the angle-weighted averaging math
- Confirmation of the regression gate
