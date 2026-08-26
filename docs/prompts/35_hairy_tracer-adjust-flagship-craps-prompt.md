# Prompt: Paired dice polish — hero-face rotation, navy ivory pips, DOF strength
## Context

Pip counts and patterns are correct (4 on red, 3 on ivory, right positions), but three things need fixing before this is genuinely showcase-quality:

1. **Wrong face is the "hero" face.** Both dice currently show a dead-on-camera "1" as their biggest, most prominent visible face, with the requested numbers (4 on red, 3 on ivory) relegated to the smaller, more oblique top face. The intent was for 4 and 3 to be the headline feature of the shot.
2. **Ivory pips read as plain silvery-gray**, not a deliberate contrasting color.
3. **DOF blur is much weaker than the single-die showcase shots** — background doesn't show the same clear soft-focus falloff.

## Scope

**1. Hero-face rotation**: re-rotate each die (as a whole rigid-body transform, not touching the internal pip geometry) so red's 4-face and ivory's 3-face become the primary front-facing/camera-dominant face, rather than the top face. This likely means adjusting each die's orientation relative to the camera more significantly than the current setup — check what face is currently dead-on to the camera (currently "1" on both) and rotate so 4/3 take that position instead, keeping the two dice's relative composition (positioning, camera framing) otherwise intact.

**2. Ivory pip color**: change the ivory die's pip material to a deep navy/indigo — something like `diffuse: [0.05, 0.08, 0.25]`, with low roughness for a slight lacquered look, replacing the current silvery-gray. This should read as a deliberate, elegant contrast against the warm cream body, not a near-neutral gray.

**3. DOF strength**: compare the current aperture/focal-distance values in the paired scene against the single-die showcase scenes (which had a clearly visible, strong background blur). Increase to match that established look, while confirming both dice still stay acceptably in focus given they're at slightly different distances from camera.

## Testing

- Smoke test at low resolution/samples first
- Confirm the new hero faces (4 red, 3 ivory) are correctly patterned and are now the dominant, camera-facing face — not just present somewhere on the die
- Confirm the navy pip color is visibly distinct from the red die's pip color and reads as intentional
- Confirm DOF blur strength now visually matches the single-die shots
- Full-quality final render once the smoke test checks out

## Deliverable

- Updated paired dice scene with all three fixes
- Final high-quality render
- Confirmation the hero faces are correctly both patterned and dominant in the composition
