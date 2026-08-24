# Prompt: Retire Python parity — Rust becomes the primary engine
## Context

Decision made: strict pixel-for-pixel agreement between the Rust engine and the original Python engine is no longer the correctness bar going forward. Rust now has capabilities Python never will (Fresnel, textures, environment maps, soft shadows, depth of field), and the `maxdepth` default increase (2 → 5, needed for environment maps to show correctly inside nested reflections/refractions) was the first change that couldn't cleanly satisfy both "match Python" and "do the new feature justice."

**What stays the same:** Python (`--pure-python`) remains in the codebase and fully runnable — it's kept as an unmaintained historical reference, not deleted.

**What changes:** Python is no longer part of the automated test suite's pass/fail criteria. No test should assert Rust output matches Python output going forward. The regression safety net that comparison used to provide needs replacing with something that doesn't depend on Python at all.

## Goal

1. Establish a **self-referential snapshot baseline**: freeze current Rust output as the new "known good" reference, and repurpose the E2E test suite to diff future changes against that baseline instead of against Python.
2. Decouple Python from test gating without removing it.
3. Make the `maxdepth` default an explicit, data-informed decision on its own merits (render-time cost vs. correctness), not an inherited side effect of the parity change.

## Before writing any code

**This is the one point where extra care matters more than usual**: freezing current output as the new ground truth means any existing bug in the current Rust renderer gets locked in as "correct" going forward, and would no longer be caught by comparison to anything. Before generating the new baseline snapshots:
- Do a final check of the current render output against the physics that were already independently validated in the structural tests (the Fresnel Schlick values, the Beer-Lambert single-ray transmittance, the exact texel/environment-map lookups) — if those checks are solid, the frozen baseline inherits that correctness rather than an unverified one
- Visually spot-check the `examples/materials/` renders from the earlier task against the corrected absorption and image-texture tests, since those exercised the same code paths this baseline will freeze

## Scope

**1. New baseline snapshots**
- Render the full existing test-scene set (the original E2E scenes plus the `examples/materials/` scenes) with the current Rust engine
- Save these as the new reference images, following whatever directory/naming convention the existing snapshot tests already use
- These become the new ground truth — future changes are correct if they match this baseline (or are an intentional, documented change to it), not if they match Python

**2. Repurpose the E2E test suite**
- Replace or update the tests that currently assert Rust output equals Python output — remove that specific assertion, don't remove the tests' value entirely
- New assertion: current Rust output matches the frozen baseline from step 1, within whatever tolerance the tests already use
- Explicitly document (a comment at the top of the test file is fine) that Python is no longer part of this comparison and why

**3. Decouple Python from gating, without deleting it**
- `--pure-python` stays functional and callable
- No test in the suite should fail based on Python's output going forward
- If there are existing tests whose *entire purpose* was the Rust-vs-Python comparison (not just an assertion within a broader test), it's fine to mark them explicitly skipped/manual with a clear comment explaining why, rather than deleting them outright — someone may want to run them by hand later out of curiosity

**4. Resolve `maxdepth` as a deliberate choice**
- Benchmark render time at `maxdepth = 2` vs `maxdepth = 5` across the existing scene set, so this is a measured tradeoff rather than an inherited assumption
- Set the default explicitly based on that data, and document the reasoning (e.g., "5 costs X% more render time on the benchmark set, but is required for correct environment-map reflections, which are now a supported feature — worth it")
- If the cost turns out to be significant for scenes that don't use environment maps, it's still worth considering scoping the higher depth to only scenes that actually define one, purely as a performance optimization now — not for parity reasons anymore

**5. Documentation**
- Update the project README or status doc: Rust is now the primary, actively developed engine; Python is retained as an unmaintained historical reference only

## Deliverable

- New baseline snapshot images, committed
- Updated E2E test suite comparing Rust against the new baseline, with Python-parity assertions removed and documented
- Benchmark numbers for `maxdepth 2` vs `5`, the default chosen, and the reasoning
- Updated documentation reflecting Rust as primary
- A short note confirming the pre-freeze sanity check (step "Before writing any code") was actually done, not skipped
