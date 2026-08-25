# Prompt: Reorganize scenes by capability tier
## Context

Scene files are currently scattered without a consistent organization scheme (`scenes/`, `scenes/pt/`, `scenes/stress/`, `examples/materials/`) reflecting the order features were built in, not what each scene actually demonstrates. Now that the full capability range exists — legacy Whitted, Whitted with opt-in Sampling/Material features, path-traced diffuse GI, and full GGX-PBR path tracing — it's worth organizing scenes by which capability tier they exercise.

**Proposed structure:**

```
scenes/
  whitted/
    legacy/     — zero opt-in features; must still match the original frozen baseline exactly
    featured/   — Whitted + Sampling/Material-quality opt-ins (AA, soft shadows, DOF, Fresnel, absorption, textures, envmaps)
  path_trace/
    diffuse/    — path-traced for GI/color bleeding, no GGX/PBR materials
    pbr/        — full-featured: GGX-in-NEE, MIS, near-specular materials
  stress/       — BVH/perf benchmark scenes (kept separate — their purpose is timing, not capability demonstration)
```

`models/` (raw OBJ mesh assets) is explicitly **not** part of this reorg — meshes are integrator-agnostic and shared across tiers, so they stay flat. Decide explicitly what happens to `examples/materials/` (fold into `scenes/whitted/featured/`, or keep separate as a documentation gallery since it's referenced from the README) — either is fine, just make it a deliberate, documented choice rather than something that falls through the cracks.

## Before making any changes

This is a repo reorganization, so the discipline is the same as every structural change so far: **inventory before moving, prove zero behavior change after.**

- Enumerate every current scene file across all existing scene directories
- Grep the entire codebase (test files, benchmark scripts, snapshot-generation scripts, `btrace.py`, the README, anything else) for every reference to a scene path — a missed reference is a silent breakage, not a loud one, since a path that resolves to nothing may just fail to find/run a test rather than erroring clearly
- For each scene, confirm its actual `integrator` setting (and feature usage) actually matches the tier you're about to file it under — if anything's inconsistent (e.g., a scene sitting in `path_trace/` territory that's actually configured for Whitted), flag it rather than silently sorting by assumption

## Scope

- Move scene files into the new structure per the taxonomy above
- Update every script/test/doc reference found during the inventory step to the new paths
- Update the snapshot baseline mapping (path → snapshot) to match new scene locations — the snapshots themselves shouldn't change, only what path they're keyed to
- Fill genuine gaps: after sorting, if any tier is thin or has no clean representative scene (for instance, if the only path-traced scene is the full PBR gold bunny with nothing demonstrating plain diffuse GI on its own), create one minimal new scene for that tier so the taxonomy is meaningfully filled out, not just structurally correct

## Testing

- **Zero behavior change is the bar, same as every prior structural reorg**: run the full test suite before and after the move and confirm identical results — paths changed, nothing else did
- Confirm every scene renders successfully from its new location (catches any missed reference the grep step didn't find)
- If any new gap-filling scenes were added, confirm they render correctly and land in the frozen baseline as new, intentional additions — not compared against anything old

## Deliverable

- The new directory structure, populated
- Confirmation every script/test reference was found and updated (list what was found, not just "everything updated")
- The before/after test suite comparison confirming zero behavior change
- Any new gap-filling scenes created, and which tier they filled
- The explicit decision on `examples/materials/`
