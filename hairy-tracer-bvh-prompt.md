# Prompt: Acceleration phase — recursive BVH
## Context

`hairy_tracer_core` currently accelerates mesh intersection with a single top-level AABB (slab method) per mesh — a ray that misses the box skips the whole mesh, but a ray that hits it falls through to a linear scan over every triangle. This phase replaces that with a real recursive BVH: a tree of nested bounding boxes that narrows down to a small candidate set of triangles instead of testing all of them.

Since the Python-parity retirement, correctness for this phase means something specific and stricter than usual: **a BVH is a pure acceleration structure — it must not change a single output pixel, ever.** Every existing scene in `tests/snapshots/` must render bit-for-bit identical to the frozen baseline after this change. This isn't a "defaults preserve old behavior" situation like Sampling or Materials — there's no opt-in flag here, because a BVH doesn't add new visual behavior, it just finds the same closest hit faster. Any pixel difference means the BVH is wrong, full stop.

**One thing to get right that the last two benchmarks got wrong**: the `maxdepth` benchmark from the previous phase came back nearly flat (0.39s vs 0.38s) most likely because none of the existing test scenes actually forced rays to recurse anywhere near the max depth — it measured the cheap case, not the expensive one. Same risk here: if none of the existing scenes have meshes with enough triangles to stress a linear scan, a before/after benchmark will look like noise regardless of whether the BVH is actually faster. Build a real stress-test scene as part of this task, not an afterthought.

## Before writing any code

- Read the current AABB/mesh intersection code — confirm exactly how the single-box-then-linear-scan currently works, since the BVH replaces this, not the primitive intersection tests themselves (sphere/plane/triangle math is untouched)
- Check whether the existing per-mesh AABB becomes redundant once a BVH's root node has its own bounding box (likely yes — the root node's box does the same job the standalone AABB did) and consolidate rather than keeping both

## Scope

**BVH structure:**
- A recursive node type: each node holds a bounding box (AABB, reusing the existing slab-method logic) and either two child nodes or a leaf containing a small list of triangle indices (a leaf threshold like ≤4 triangles is a reasonable starting point)
- Build top-down: at each node, choose a split axis (the longest axis of the current bounding box is the simplest reasonable heuristic), split triangles by the median of their centroids along that axis, and recurse until the leaf threshold or a max tree depth is reached
- A median-split heuristic is fine for this task; a full surface-area-heuristic (SAH) split is a legitimate future optimization but not required here — don't get pulled into it as scope creep
- Traversal: test the ray against a node's box; if it misses, prune that whole subtree; if it hits, recurse into children (testing the closer child first, using a running closest-hit distance to prune the other child early when possible) — until reaching a leaf, where triangles are tested individually as before

**Integration:**
- The mesh's `Intersectable` implementation should delegate to BVH traversal instead of the current linear scan
- No change to the `Intersectable` trait's external interface — this is purely an internal implementation swap

**Stress-test scene:**
- Build (or generate) a scene with a mesh containing enough triangles that a linear scan is measurably expensive — thousands of triangles, not the handful used in existing test scenes. A subdivided sphere or a denser `.obj` is a reasonable way to get there.
- This scene doesn't need to go in the permanent snapshot set necessarily, but it does need to exist and be used for the benchmark below

## Testing

- **Correctness, exact**: every scene in the current snapshot baseline must render bit-for-bit identical after the BVH replaces the linear scan — no tolerance, no "close enough." Run the full snapshot suite and confirm zero diff.
- **BVH-specific correctness tests**: verify every triangle in a mesh ends up in exactly one leaf (none lost, none duplicated) after construction; verify BVH traversal returns the same closest hit as a brute-force linear scan, on both the existing small test meshes and the new stress-test mesh — this is the direct proof the tree traversal logic is sound, independent of whether it's also faster
- **Benchmark**: render the stress-test scene with the old linear-scan path and the new BVH path, report actual timing for both — this is the number that should show a real difference, unlike the flat `maxdepth` result
- Run the full existing pytest suite to confirm nothing upstream regressed

## Deliverable

- Recursive BVH construction and traversal, replacing the per-mesh linear scan
- The stress-test scene, committed somewhere sensible (e.g. `scenes/stress/` or similar)
- Confirmation of exact (not tolerance-based) match against the full snapshot baseline
- Benchmark numbers: linear scan vs. BVH on the stress-test scene, and a brief note on the speedup observed
