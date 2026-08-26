# Prompt: Fix drilled-cube scene (FOV + floor placement) and add an FOV safeguard
## Context

The rounded-drilled-cube render has two problems, both scene-authoring issues rather than CSG bugs:

**1. FOV is far too wide.** `distance: 1.0` with `vpwidth: 12.0` computes to `2·atan(6/1) ≈ 161°` — this is the same category of mistake as the Sponza FOV bug (vpwidth not scaled correctly relative to `distance`), just in the opposite direction: too wide here instead of too narrow there. Fix: set `vpwidth`/`vpheight` to `1.0` to match `distance: 1.0`, giving the same ~53° FOV used successfully in every earlier test scene — this comfortably frames the object (which spans roughly 4-5 units) from the camera's actual distance (~12.7 units away).

**2. The floor likely sits above the object instead of below it.** The checkered plane is `normal: [0,1,0]`, `distance: 2.0`. Confirm the actual sign convention (does `distance` place the plane at `y = +distance` or `y = -distance`?) — if it's placing the plane at `y = +2.0`, that's directly between the camera (`y = 5.0`) and the object (centered at the origin, spanning roughly `y = -2` to `+2`), acting as an obstruction rather than a floor beneath it. Fix the `distance` value (likely needs to be negative, or a larger magnitude, depending on the actual convention) so the floor sits below the object instead.

## Scope

**1. Fix the immediate scene**: correct `vpwidth`/`vpheight` and the floor's `distance` value, re-render, confirm the object is properly framed with a real floor beneath it (not above/through it)

**2. Add a systemic FOV safeguard**, since this specific mistake (vpwidth/distance mismatch) has now happened twice in two different directions:
- Either: a load-time check that computes the effective FOV from `distance`/`vpwidth`/`vpheight` and warns or errors if it falls outside a sane range (e.g., under ~15° or over ~150° is almost certainly a units mistake, not an intentional extreme lens)
- Or (preferred, since it prevents the mistake rather than just catching it): add a `fov_degrees` field as an alternative way to specify the camera's field of view directly, with the engine computing `vpwidth`/`vpheight` internally — removing the manual trig from scene authoring entirely. Keep the existing `vpwidth`/`vpheight`/`distance` fields working for backward compatibility with existing scenes.
- Pick whichever seems like better value for the effort; either is a reasonable choice, just make it a deliberate one

## Testing

- Regression gate as always: existing scenes unaffected by the safeguard addition (this is purely additive/validating, not changing existing camera behavior)
- Re-rendered drilled-cube scene showing correct framing and a proper floor beneath the object
- If a `fov_degrees` field was added, a quick test confirming it produces the same rays as an equivalent hand-computed `vpwidth`/`distance` pair

## Deliverable

- Fixed drilled-cube scene JSON and a corrected render
- The FOV safeguard (sanity check or `fov_degrees` field), whichever was chosen, with reasoning for the choice
- Confirmation of the regression gate
