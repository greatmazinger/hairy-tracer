## Context

Repo layout — Python at root, Rust crate as a sibling directory:

```
    hairy-tracer/                          # repo root
    ├── src/                               # Python ray tracer engine
    │   ├── btrace.py                      # Main entry point, CLI, camera system (Look-At basis), multiprocessing render loop
    │   ├── scene_parser.py                # JSON scene → internal objects (camera, materials, lights, geometry)
    │   ├── obj_parser.py                  # Wavefront .OBJ loader with N-gon triangulation
    │   ├── rvmath/
    │   │   ├── __init__.py
    │   │   └── geometry/
    │   │       ├── __init__.py            # Exports: Plane, CheckeredPlane, Sphere, Triangle, Mesh
    │   │       ├── ray.py                 # Ray class (origin + direction)
    │   │       ├── sphere.py              # Sphere intersection (quadratic formula)
    │   │       ├── plane.py               # Plane + CheckeredPlane intersection
    │   │       ├── triangle.py            # Triangle intersection (Möller–Trumbore algorithm)
    │   │       ├── mesh.py                # Mesh container with AABB bounding box optimization
    │   │       ├── world.py               # World: object list, findIntersection, shadow rays
    │   │       └── utils.py               # Reflection/refraction vector helpers
    │   ├── rvlight/
    │   │   ├── __init__.py
    │   │   ├── light.py                   # Point light source
    │   │   └── illuminationmodel.py       # Phong illumination (ambient + diffuse + specular)
    │   ├── rvcolor/                       # Color utilities
    │   └── utils/                         # General utilities
    ├── scenes/                            # JSON scene definitions
    │   ├── spheres3.json                  # Classic 3-sphere scene
    │   ├── checkers.json                  # Checkerboard floor demo
    │   ├── triangle.json                  # Single triangle test
    │   ├── mesh_test.json                 # Pyramid mesh on checkerboard
    │   ├── torus_test.json                # 900-triangle torus (AABB benchmark)
    │   └── camera_test.json               # Look-At camera demo (diagonal aerial view)
    ├── models/                            # Wavefront .OBJ 3D models
    │   ├── pyramid.obj                    # 4-triangle pyramid
    │   ├── pyramid_rotated.obj            # Same pyramid, rotated 45° around Y
    │   └── torus.obj                      # Procedurally generated 900-triangle donut
    ├── tests/
    │   ├── conftest.py                    # pytest config (adds src/ to PYTHONPATH)
    │   ├── test_geometry.py               # Unit tests: Triangle intersection, normals
    │   ├── test_btrace.py                 # E2E snapshot test: renders 10x10 and compares pixel data
    │   └── snapshots/                     # JSON pixel snapshots for E2E regression tests
    ├── hairy-tracer-core/                 # Rust crate (intersection + BVH + rayon tiling)
    │   ├── Cargo.toml
    │   ├── Cargo.lock
    │   └── src/
    │       ├── lib.rs                     # Crate root
    │       ├── ray.rs                     # Ray struct
    │       ├── hit.rs                     # HitRecord struct
    │       ├── intersect.rs               # Intersectable trait
    │       ├── material.rs                # Material struct
    │       ├── sphere.rs                  # Sphere intersection
    │       ├── plane.rs                   # Plane intersection
    │       ├── checkered_plane.rs         # CheckeredPlane intersection
    │       ├── triangle.rs                # Triangle intersection (Möller–Trumbore)
    │       ├── mesh.rs                    # Mesh with AABB
    │       ├── aabb.rs                    # Axis-Aligned Bounding Box
    │       └── scene.rs                   # Scene definition
    ├── generate_torus.py                  # Script to procedurally generate torus.obj
    ├── render_all.sh                      # Batch render script
    ├── README.md
    ├── LICENSE
    └── .gitignore

```

`hairy_tracer_core` already implements ray/scene intersection, BVH traversal, and a `rayon`-parallel tile-based render loop, built and tested as a standalone Rust crate with no Python dependency so far. This task is the final integration step: add PyO3 bindings and wire the Python side to call into it.

This is step 3 of the plan:
1. ~~Intersection + BVH traversal in Rust, unit-tested in isolation~~ — done
2. ~~Wrap in a rayon-parallel tile render loop~~ — done
3. **This task** — expose it to Python via PyO3 and integrate

## Before writing any code

Don't assume the crate's shape from scratch — inspect what's actually there first:
- Read `hairy_tracer_core/src/lib.rs` (and any other source files) to find the current public entry point(s) — what function(s) exist for "render a scene," what types they take and return
- If a design-decisions summary was written when the crate was built, read it — it documents any deviations from the original Python behavior that matter here
- Read the existing Python render path (`raytracer.py` or equivalent) to see how a scene is currently loaded and how rendering is invoked, so the new call site fits the existing shape rather than inventing a new one
- Check whether the crate already accepts a scene as structured data (via `serde` deriving from the same shape as the JSON) or expects a raw JSON string it parses itself with `serde_json` — this determines how much scene-marshaling logic belongs on which side of the boundary. If neither exists yet, decide and implement one; note the choice in your summary.

If the current crate's entry point doesn't match what render_image would need (e.g., scene parsing was left out of scope during step 1/2 and isn't handled anywhere yet), raise it rather than silently reshaping the crate's internals — ask me if it's not obvious which side should own it.

## Goal

A single coarse-grained PyO3-exposed function — something like `render_image(scene_json: &str, width: usize, height: usize) -> Vec<u8>` — that Python can call once per render. No per-ray or per-tile crossing of the Python/Rust boundary.

## Scope

**Rust side:**
- Add `pyo3` as a dependency, configure `Cargo.toml` for a `cdylib` (`crate-type = ["cdylib"]`) alongside the existing `lib` target if the crate is still built as a plain library elsewhere
- Wrap the existing render entry point in a `#[pyfunction]`, exposed via a `#[pymodule]`
- Use `py.allow_threads(...)` around the actual rendering call so the rayon thread pool isn't serialized behind the GIL
- Don't modify the intersection/BVH/tiling internals unless the inspection step above turns up something that's actually missing (e.g., scene parsing) — this task is the boundary, not a rewrite of what's inside it

**Python side:**
- Add `maturin` as a dev dependency; confirm `pyproject.toml` is configured for it (`[build-system]` using `maturin`, `[tool.maturin]` pointing at the `hairy_tracer_core` crate)
- Add a new call site that imports the built extension and calls `render_image(...)`
- **Keep the existing pure-Python render path intact** — don't delete or replace it. Put the new path behind a flag, a separate function, or a config option (your choice, pick one and note it) so both remain callable side by side
- Existing scene-loading/JSON code on the Python side should stay put; only add what's needed to hand the scene off to the new function per whatever boundary shape was decided above

## Validation

- `maturin develop` to build the extension locally
- Run the existing `pytest` suite unchanged — it should still pass against the pure-Python path
- Add a comparison test: render the same scene through both the pure-Python path and the new Rust-backed path, and diff the resulting pixel arrays against each other (and against the existing E2E snapshot). They should match, or be documented if there's an acceptable numeric tolerance (e.g., floating-point ordering differences from parallel execution) — don't silently accept a mismatch
- Report actual timing difference between the two paths on at least one non-trivial scene, since that's the entire point of this work

## Deliverable

- PyO3 bindings and updated `Cargo.toml`/`pyproject.toml`
- The new Python call site, with the pure-Python path still present and working
- The comparison test
- A short note on: which side owns scene parsing and why, what flag/mechanism selects between the two render paths, and the measured speedup
