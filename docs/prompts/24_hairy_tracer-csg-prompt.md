# Prompt: CSG (constructive solid geometry)
## Context

CSG (union, intersection, difference of shapes) requires a fundamentally different query than anything the engine currently does. Every primitive so far answers "what's the single closest hit along this ray?" — that's sufficient for ordinary rendering, but CSG needs "give me every interval along this ray where I'm *inside* this shape" (a ray typically enters and exits a convex primitive once, e.g. a sphere gives one `[t_enter, t_exit]` interval), so boolean operations can be computed by combining interval lists: union merges them, intersection keeps only overlapping ranges, difference subtracts one shape's intervals from another's. This is the standard ray-CSG technique (interval/Roth-style ray casting), not something to invent from scratch, but it is new capability, not a rewiring of what already exists.

Two primitives needed for the target demo scenes don't exist yet: **Cube** and **Cylinder**. Confirm this rather than assuming — but if they're genuinely absent, they need to be added as real first-class primitives (each also usable standalone, outside CSG, for free — a box's intersection math is literally the same slab method already used internally for AABBs, just exposed as a renderable primitive rather than only an acceleration structure).

## Before writing any code

- Read the current `Intersectable` trait and `Hit` struct closely — the new interval query needs to sit alongside the existing closest-hit query without disrupting it; every non-CSG object in every existing scene must keep using the closest-hit path exactly as today
- Confirm whether Cube and Cylinder already exist anywhere in the codebase
- Think through how a `Plane` (an unbounded half-space) behaves as a CSG operand — its "interval" is a half-infinite ray, not a bounded range. Decide how to handle this (e.g., only well-defined when intersected with something that bounds it) and document the decision rather than letting it silently misbehave in a union

## Scope

**1. New primitives**
- `Cube` (axis-aligned box), reusing the existing AABB slab-method intersection math directly
- `Cylinder` (finite, capped) — standard quadratic side-intersection plus cap-plane checks
- Both implement the existing `Intersectable` interface too, so they work as ordinary standalone objects outside CSG as well

**2. Interval intersection**
- A new query (trait method or separate function) returning every `[t_enter, t_exit]` interval a ray spends inside a primitive, with enough data per boundary (normal, material/UV info) to recover a correct hit at that specific crossing later
- Implement for Sphere, Cube, Cylinder, and Plane (per the half-space decision above)

**3. CSG node**
- A node type combining two children (each either a primitive or another CSG node, so trees compose recursively — e.g., `difference(intersection(cube, sphere), cylinder_x, cylinder_y, cylinder_z)` for the rounded-drilled-cube) with an operation: union, intersection, or difference
- Combine the two children's interval lists via standard interval set arithmetic per the operation
- Implement the existing `Intersectable` interface for the CSG node itself, derived from the combined intervals (closest hit = smallest `t > 0` boundary of the resulting "inside" intervals) — this is the key integration point: a CSG node should look like any other primitive to the BVH, shading, and path tracer, no special-casing needed elsewhere
- **Normal/material recovery at the winning boundary is the detail most likely to go subtly wrong** — make sure the hit result at a CSG boundary correctly reflects whichever underlying primitive actually contributed that surface, not the CSG node abstractly

**4. Scene JSON support**
- A minimal way to express a CSG tree in scene JSON (nested object/operation references). Keep this purely functional — don't build ergonomic authoring conveniences here, that's explicitly the scene DSL's job later

## Testing

- **Regression gate**: existing scenes (none use CSG) render bit-for-bit identical — this is purely additive
- Unit tests directly on the interval math, not just rendered output:
  - A known ray/sphere interval matches a hand-computed value
  - Union, intersection, and difference of two known simple interval lists (e.g. two overlapping spheres) produce the expected combined result
  - Normal/material recovery at a boundary picks the correct underlying primitive, tested with a case where it would be easy to get backwards (e.g. a difference where the visible surface comes from the subtracted shape's inner wall)
- Edge cases worth checking explicitly: a ray grazing a CSG boundary tangentially, and a `Plane` used inside an intersection correctly bounding/clipping the other operand
- **Demo scenes**: build and render the rounded-drilled-cube (intersect a cube with a sphere, then subtract three axis-aligned cylinders) as the primary correctness test — this is the classic CSG textbook scene for a reason, it exercises intersection and difference together in one recognizable shape. Follow with the "Death Star" (a sphere with a smaller sphere subtracted off-center) as a second, simpler confirmation.

## Deliverable

- Cube and Cylinder primitives (if not already present)
- Interval-intersection query implemented across the relevant primitives
- CSG node type supporting union/intersection/difference, recursively composable, integrated as a normal `Intersectable`
- Minimal scene JSON support for CSG trees
- Confirmation of the regression gate, the interval/normal-recovery unit tests, and the two demo scene renders
