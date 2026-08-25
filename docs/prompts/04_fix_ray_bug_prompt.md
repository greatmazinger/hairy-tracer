# Fix the Ray unitvector (viewvec) bug in Python

We are porting the hairy-tracer engine to Rust and doing 1-to-1 pixel snapshot comparisons. During the port, we discovered a mathematical bug in the Python codebase that distorts specular highlights. We need you to fix this bug in the Python source so we can update our snapshots.

## The Bug

In `src/rvmath/geometry/ray.py`, the `Ray` class `__init__` contains the following logic:

```python
self.direction = numpy.copy( dir )
self.direction = self.direction / numpy.linalg.norm( dir )
try:
    uv = dir - orig
except:
    # fallback logic
    pass
self.unitvector = uv / numpy.linalg.norm( uv )
```

The bug is that `dir` is usually *already* a direction vector (e.g., in `btrace.py`, rays are created with `dir = target_point - cam_origin`). Because `Ray` computes `uv = dir - orig`, it is effectively subtracting the origin *twice* (`target_point - cam_origin - cam_origin`). 

This broken `self.unitvector` is later retrieved via `getUnitVector()` and used as the `viewvec` for specular highlights (e.g., in `sphere.py`), causing the shiny spots on materials to appear in the wrong physical locations.

## The Fix

Please update `src/rvmath/geometry/ray.py` so that `getUnitVector()` simply returns the correctly normalized `direction` of the ray. 

1. Remove the `uv = dir - orig` logic and the `try/except` block entirely.
2. `self.unitvector` is completely redundant. You can either remove it and make `getUnitVector()` return `self.direction`, or just assign `self.unitvector = self.direction`.
3. After applying this fix, re-run the end-to-end tests in `tests/test_btrace.py`. The test will fail because the output pixels have changed. You will need to delete `tests/snapshots/spheres3_10x10.json` and run the test again to generate the corrected snapshot.

Once this is fixed in Python, the Rust team will mirror the fix in the Rust crate so the two implementations remain perfectly synced.
