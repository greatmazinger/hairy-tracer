# Prompt: Fix fireflies/color blotching from smooth-normal interpolation
## Context

The smooth-normal change introduced a real regression: re-rendering the Stanford Bunny scene with the path tracer now shows heavy red/green color blotching across the mesh surface, roughly following what look like individual triangle boundaries — not sparse pinpoint fireflies, solid wrong-colored patches. The background walls/floor are also noisier than the earlier flat-shaded render, which may just be a different `samples_per_pixel` setting between the two renders rather than part of the same bug — check the render settings match before treating that as related.

## Isolation step — do this first

Re-render the same scene with the **Whitted integrator** instead of the path tracer, with smooth normals still on. This determines whether the bug is in the normal computation itself (would show up under Whitted too) or specifically in how the path tracer's shading math consumes the interpolated normal (would only show up under path tracing). This directly narrows the search space before touching any code.

## Hypotheses to check, roughly in order of likelihood given this project's history

**1. `vn` index mismatch during face triangulation** — this project has hit this exact bug class twice already: the `vt` index mismatch fixed during the Materials phase, and the `original_index` hardcoded to `0` for every triangle found during the BVH work. Both were cases of correctly-parsed per-vertex data getting mis-wired during triangle-fan triangulation of `f v/vt/vn` faces. Check specifically: when a face is fanned into multiple triangles, does each generated triangle get its own three correct `vn` indices, or is an index being reused/misaligned across fan triangles the same way `vt` or `original_index` were before? The blotchy, triangle-boundary-aligned pattern in the render is consistent with specific individual triangles getting the wrong vertex normals, not with a systemic math error that would affect the whole mesh uniformly.

**2. Un-normalized interpolated normal** — barycentric interpolation of three unit vertex normals does not itself produce a unit vector (it's shorter than 1 everywhere except exactly at the vertices). If the interpolated normal isn't explicitly renormalized before being used in shading math (cosine terms, Fresnel, the BRDF denominator), magnitudes will be systematically wrong. In a path tracer, where results are divided by importance-sampling PDFs and errors compound across recursive bounces, even a small magnitude error can blow up into extreme single-sample outliers — which would explain fireflies specifically appearing under path tracing.

**3. Shading-normal vs. geometric-normal divergence** — interpolated (shading) normals can point meaningfully differently than the flat geometric face normal, especially near silhouettes or areas of high curvature relative to triangle size. If cosine terms or BRDF evaluation use the shading normal without any consistency check against the geometric one, rays can effectively sample into the wrong hemisphere or produce negative cosine terms that aren't properly clamped — a well-known raytracing footgun (sometimes called "the shading normal problem"), usually fixed by clamping or falling back toward the geometric normal when the two diverge too much.

## Testing

- Once fixed, confirm the specific render that showed blotching is now clean, under whichever integrator(s) were affected
- Add a direct unit test that the interpolated normal is unit-length (within a small epsilon) across a range of sampled barycentric coordinates on a test triangle with known, non-trivial vertex normals — this tests the mechanism directly rather than relying on a full render looking right, same lesson as the BVH correctness tests
- If it was a `vn` index issue, add a test analogous to the existing `vt` index tests confirming each triangulated face's normals map to the correct original vertices
- Regression gate as always: existing snapshot suite still bit-for-bit identical

## Deliverable

- Root cause identified (from the above or otherwise) and fixed
- Confirmation from the Whitted-vs-path-tracer isolation step of where the bug actually lived
- A clean re-render of the Stanford Bunny scene
- The new unit test(s) targeting the specific mechanism that was wrong
