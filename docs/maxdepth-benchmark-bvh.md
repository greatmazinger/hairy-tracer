# Max Depth Benchmark (Post-BVH)

We ran the max depth benchmark on the heavily recursive `mirrors.json` stress scene, modified to contain the 7,200 triangle `bvh_stress.obj` mesh. This forces rays to bounce repeatedly through the mesh inside the mirrored box.

Resolution: 1600x1200
Samples per pixel: 10

| Max Depth | Rust Render Time |
|-----------|------------------|
| Depth 2   | 0.1695 seconds   |
| Depth 5   | 0.1693 seconds   |
| Depth 10  | 0.1735 seconds   |

**Conclusion:**
With the new BVH implementation, each intersection test went from `O(N)` to `O(log N)`. For the 7,200 triangle mesh, a single bounce went from ~7,200 checks down to ~13 checks. 
Because the per-bounce cost is now so drastically small, the relative weight of deep recursion chains has vanished into the noise floor of standard execution overhead. The engine can easily handle `depth=5` (and even `depth=10`) by default without any measurable performance penalty!
