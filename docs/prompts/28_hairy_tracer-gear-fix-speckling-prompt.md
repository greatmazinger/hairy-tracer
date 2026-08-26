# Prompt: Fix coincident-surface speckling (tooth flush against disk rim)
## Context

With proper lighting now in place, a localized speckled/dithered black patch is visible through the gear's bore hole — roughly the size and shape of a single tooth. This is a different, more subtle bug than the transform-order issue already fixed (that one caused all teeth to collapse to one point; this one is present even with teeth correctly distributed).

**Likely root cause**: in `csg_gear.json`, each tooth is translated to `z: 2.0`, and the outer cylinder's radius is also exactly `2.0`. That means the tooth's inner face and the disk's outer rim are designed to sit at *exactly* the same position — flush, with zero gap and zero overlap. Exact coincidence between two surfaces is a well-known numerical instability in CSG/boolean geometry: the interval math has no reliable way to decide which surface is "in front" when they're at precisely the same distance, and floating-point rounding makes the answer flicker per-ray, which is exactly what a speckled/dithered pattern looks like. This condition technically applies to all 24 teeth, but is likely only visually obvious where the camera's viewing angle happens to graze that coincident boundary near-tangentially — which is the case for the view through the bore hole.

## Isolation step — confirm before fixing

Render just the base disk plus a single tooth (the unrotated one is fine, or any one tooth), with a camera angle chosen to view its base near-tangentially against the disk's rim (similar to the angle where the speckling is currently visible). If the same speckling reproduces in this minimal scene, that confirms the coincident-surface hypothesis directly.

## Fix

The standard, well-established fix for this class of bug is to **avoid exact coincidence by design** — give the tooth a small deliberate overlap into the disk rather than a flush fit. Concretely: reduce the tooth's translate distance slightly below the disk's outer radius (e.g., `1.9` instead of `2.0` for a radius-`2.0` disk), so the tooth's inner portion genuinely embeds into the disk and the union has real, unambiguous overlapping volume rather than two surfaces touching at a knife-edge. This is a standard CSG modeling practice, not a hack — deliberate small overlaps at union boundaries are how coincident-surface ambiguity is avoided in general, not just for this scene.

- Apply the fix to the gear scene (adjust the translate distance for all teeth, keeping them properly embedded in the rim)
- Consider whether this is worth documenting as general CSG-authoring guidance (e.g., in the scene DSL work later, or just as a comment/note for now) — coincident surfaces will bite any future CSG scene that fits parts flush against each other, not just this one

## Testing

- Confirm the isolation-test scene reproduces the speckling, then confirm the fix resolves it in that minimal scene before re-rendering the full gear
- Re-render the full gear scene and confirm the speckled patch through the bore hole is gone
- Regression gate as always: the rounded-drilled-cube and Death Star scenes don't have this coincident-surface pattern, so should be unaffected — confirm explicitly rather than assuming

## Deliverable

- Confirmation of the isolation test result (does the minimal scene reproduce the speckling)
- The fix applied to the gear scene, with the specific translate/radius values chosen and why
- A clean re-render of the full gear with no visible speckling
- A brief note on whether/where this gets documented as general CSG-authoring guidance
