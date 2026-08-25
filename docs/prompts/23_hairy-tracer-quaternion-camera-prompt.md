# Prompt: Quaternion camera orientation
## Context

The current camera is built from eye/look_at/up vectors via a look-at basis construction — this fully and correctly specifies a single static orientation, so there's no gimbal-lock problem to fix for a single frame. The real value of a quaternion representation here is forward-looking: it's the primitive that camera *animation* (a separate, future feature) will need for smooth, robust interpolation between orientations, and as a side benefit it makes composing rotations (orbiting, incremental yaw/pitch/roll) much easier to author than hand-computing raw look-at vectors — visible as exactly the kind of arithmetic scene authors currently have to do by hand (see the Sponza scene JSON's raw camera vectors).

**This task is scoped to the orientation representation and interpolation primitive only — not animation itself.** Building a frame-sequence rendering pipeline is separate future work; keep this task to the math.

## Before writing any code

- Read the current camera struct and look-at basis construction (right/up/forward from eye/target/up) — this is what needs to be reproducible from a quaternion representation
- Decide whether the quaternion becomes the camera's primary internal representation (with look-at vectors converted into it at load time) or a secondary representation constructed on demand — either is reasonable, but it should be a deliberate choice

## Scope

- A quaternion type supporting: construction from axis-angle, construction from the existing look-at vectors (for backward compatibility with all current scene JSON), multiplication/composition, and normalization
- Conversion from a quaternion back to the right/up/forward basis vectors the ray-generation code already consumes — this is the integration point, and it must reproduce the *exact* current basis for any camera converted from existing look-at vectors, since that's the regression-critical case
- Spherical linear interpolation (slerp) between two orientations — this is the actual payoff of this task, and the piece that camera animation will build on directly
- **No-op regression discipline, same as every feature so far**: existing scenes specify cameras via look-at vectors; converting those to a quaternion and back to basis vectors must produce bit-for-bit identical rays to the current implementation

## Explicitly out of scope for this task

- Animation / frame-sequence rendering (separate future feature — this task only builds the primitive it will need)
- Ergonomic scene-JSON authoring sugar (e.g. an "orbit by N degrees" convenience in scene files) — worth doing eventually, but keep this task to the underlying math so it stays focused

## Testing

- **Regression gate**: existing scenes render bit-for-bit identical, confirming quaternion-from-look-at-vectors round-trips correctly back to the same basis vectors
- Unit tests on the math directly, not just visual inspection (same lesson as the normal-interpolation and BVH correctness work):
  - Quaternion constructed from a known simple axis-angle rotation (e.g., 90° about the Y axis) produces the expected basis vectors
  - Composing two known rotations produces the expected combined rotation
  - Slerp at `t=0` and `t=1` returns exactly the two endpoint orientations
  - Slerp at `t=0.5` between two known simple orientations (e.g., 0° and 90° about a single shared axis) returns the expected 45° orientation
- Optional but worthwhile: render a handful of frames along a slerp path between two distinct camera orientations, just as a visual sanity check that interpolation is smooth and doesn't do anything unexpected — this previews what animation will look like later without building the animation feature itself

## Deliverable

- Quaternion type with construction, composition, and slerp
- Conversion to/from the existing look-at basis representation
- Confirmation of the regression gate
- The unit tests above, with actual results reported
- If done, the optional slerp-path preview frames
