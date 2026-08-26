# Prompt: CSG transform node + hand-authored gear scene
## Context

The gear scene (a cylindrical base with N teeth evenly spaced around its circumference) needs something the CSG system doesn't have yet: a way to rotate a primitive. The current `Cube` is axis-aligned min/max, and nothing in the CSG work so far supports arbitrary orientation. Placing gear teeth requires rotating the same tooth shape to N different angles around the gear's central axis before unioning each into the base — impossible without a transform node.

This scene is being built deliberately by hand, in raw JSON, as the "before" — the painful, repetitive version that will motivate building a scene DSL next. Don't add any authoring conveniences here (like a loop construct) — that's explicitly the next task's job. This task is: make the transform node exist, and use it the hard way, N times, by hand.

## Before writing any code

- Confirm the transform node genuinely doesn't exist yet — check the CSG node types and primitive definitions directly rather than assuming
- Decide scope: **translation + rotation only, no scaling**, for this task. This is a deliberate simplification — non-uniform scale would require inverse-transpose handling for normals, which isn't needed for placing gear teeth (rotation and translation alone are sufficient) and would be unnecessary complexity here

## Scope

**Transform node**
- A node wrapping a single child (a primitive or another CSG (sub)tree), carrying a rotation and a translation
- Intersection (and the interval query CSG needs) works by transforming the incoming ray into the child's local space using the *inverse* of the node's transform, running the normal intersection logic there, then transforming the resulting hit point and normal back into world space using the forward transform
- Since this is rotation + translation only (no scale), the normal transforms the same way the position does — no inverse-transpose complexity needed
- Must work correctly nested inside CSG union/intersection/difference — a transformed primitive should behave exactly like any other operand

**Gear scene, built by hand**
- Base: difference of two cylinders (outer radius minus a smaller inner bore/axle hole), matching the die/bowling-ball style of a simple hole-through-the-middle shape
- Teeth: simple rectangular box teeth (not tapered/involute — that's a nice-to-have polish item for later, not needed to prove out the transform node or motivate the DSL) placed around the base's circumference, each as a `Cube` wrapped in a `Transform` node with the appropriate rotation angle (`360° / N` apart) and translation (out to the base's outer radius), unioned together with the base
- Pick a reasonable tooth count (e.g., 24) — enough to look convincingly gear-like and enough repetition in the JSON to make the DSL's eventual value obvious

## Testing

- **Transform correctness, tested directly**: rotate a simple primitive (e.g., a box) by a known angle and confirm a ray that would hit the un-rotated box at a known point now hits the rotated version at the correctly rotated point — hand-computed expected values, not just "the render looks plausible"
- Confirm a transformed primitive still works correctly as an operand inside a CSG union/intersection/difference (not just standalone)
- **Regression gate**: existing scenes unaffected, purely additive
- Render the gear scene and visually confirm N evenly-spaced, correctly-oriented teeth around the base

## Deliverable

- The transform node (rotation + translation), integrated with both standalone rendering and CSG operations
- The hand-authored gear scene JSON and its render
- Confirmation of the transform-math unit test and the regression gate
- Roughly how large/repetitive the final gear JSON ended up being (line count, or however it's easiest to convey) — this is useful data for scoping the DSL next
