# Prompt: Fix transform composition order (gear teeth collapsing to one point)
## Context

The gear scene shows two disconnected blobs instead of one unified gear, with speckled/jagged noise on both surfaces. The likely cause: the `transform` node's rotate and translate aren't composing in the order needed to arrange copies around a circle. For "N copies evenly spaced around a center," the correct composition is **translate first, then rotate** — push the shape out to its radius, then rotate that already-offset position around the center axis, so each copy's rotation sweeps it to a different angular position. If the code instead applies rotate-then-translate (rotate the shape in place, then apply a fixed world-space translation regardless of that rotation), every copy ends up at essentially the same position — which would explain both symptoms at once: a pile of ~24 near-identical, overlapping boxes reads as one disconnected clump away from the disk, and unioning that many nearly-coincident boxes produces exactly the kind of degenerate, speckled boundary noise visible in the render.

## Debugging step — confirm before fixing

Render a minimal test: two teeth only, at 0° and 180°. If they land on opposite sides of the disk, composition order is correct and the bug is elsewhere. If they land on top of each other (or very close), this confirms the composition-order diagnosis directly before touching the general fix.

## Scope (assuming the diagnosis holds)

- Fix the `transform` node to compose as translate-then-rotate consistently, **regardless of the order its fields happen to appear in the JSON** — the composition order shouldn't be implicitly tied to field order, since that's fragile and exactly how this happened
- Document the fixed composition order clearly in the transform code, so it can't quietly flip again
- Re-render the full gear scene and confirm teeth are now evenly spaced around the disk's circumference, forming one connected shape

## Testing

- The two-teeth isolation test above, as a permanent unit/integration test (not just a one-off manual check) — confirm two copies at known angles land at their expected, distinct positions
- Regression gate as always: existing CSG scenes (rounded-drilled-cube, Death Star) don't use `transform`, so should be unaffected — confirm this explicitly rather than assuming
- Re-rendered gear scene showing a single connected, evenly-toothed disk with clean edges (no speckled noise)

## Deliverable

- Confirmation of the two-teeth isolation test result
- Fixed transform composition order, with the order made explicit/documented
- The new transform-order test
- A clean, correct render of the full gear
