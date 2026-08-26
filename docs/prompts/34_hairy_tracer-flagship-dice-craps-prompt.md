# Prompt: Paired dice flagship — red showing 4, ivory showing 3
## Context

A third flagship image: both dice together in one composition, red showing 4 and ivory showing 3. Both individual die scenes are now correct and tuned (generous framing, strong DOF, corrected pip shading, neutral warm-cream ivory) — reuse that known-good geometry, materials, and pip layouts rather than re-deriving anything from scratch. This is a composition task, not a redesign.

## Scope

**Reuse, don't rebuild**: pull the existing cube-minus-pips geometry and material definitions for both the red and ivory dice directly from their respective DSL files. Do not re-derive the pip coordinate math — it's already correct and tested.

**Face selection**: using the existing opposite-faces-sum-to-7 convention already baked into each die's geometry, rotate each die as a whole rigid body so red's **4** face (four corners pattern) is the primary camera-facing/top face, and ivory's **3** face (diagonal three) is its primary camera-facing/top face. This is a rotation of the entire die object, not a change to its internal pip geometry.

**Positioning**: place the two dice near each other on the same floor plane — side by side or gently leaning together, not perfectly grid-aligned (a slight relative rotation between them reads as a more natural "tossed dice" composition than two parallel boxes). Explicitly confirm their bounding volumes don't interpenetrate — they're separate objects, not a CSG union, so any overlap will look like two solids poking through each other rather than a clean boolean result. Keep them at a similar distance from the camera (side by side rather than front-to-back) so both can reasonably stay in focus.

**Camera and DOF — re-tune, don't copy blindly**: the existing single-die camera framing was tuned for one subject and will not automatically frame two objects with the same generous margins. Widen the FOV or pull the camera back as needed, and re-check the DOF aperture/focal-distance settings — the aperture value tuned for a single die might be too aggressive now and could push one of the two dice outside the sharp focal region. Confirm both dice end up acceptably in focus (unless a deliberate one-sharp-one-soft artistic choice is preferred — if so, make that a stated decision, not an accident).

**Lighting/environment**: reuse the existing key/fill/environment-map recipe from the single-die scenes — no need to redesign lighting from scratch, just confirm it reads well with two objects casting shadows on each other and the floor rather than just one.

## Testing

- Smoke test at low resolution/samples first, same discipline as every complex scene in this project
- Confirm the visible face patterns are correct: exactly 4 pips in the four-corners pattern on red, exactly 3 pips in the diagonal pattern on ivory — this is the one detail that would be immediately obviously wrong to anyone glancing at the image
- Confirm no geometric interpenetration between the two dice
- Confirm both dice are acceptably in focus (or the soft/sharp choice is deliberate)
- Full-quality final render once the smoke test checks out

## Deliverable

- The new paired-dice DSL scene, compiled JSON, and final high-quality render
- Confirmation of correct pip counts/patterns on both visible faces
- A note on the final camera/DOF settings chosen and why, given they needed to differ from the single-die scenes
