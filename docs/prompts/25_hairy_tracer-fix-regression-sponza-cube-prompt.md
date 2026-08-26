# Prompt: Fix the camera regression (FOV magnitude + vertical flip)
## Context

After adding the FOV safeguard (systemic fix for the earlier vpwidth/distance mismatch bug class), **two problems appeared together, in the same round of changes, across two different scenes**:

1. **FOV magnitude regression**: the drilled-cube scene swung from too-wide (161°) to way too-narrow (telephoto/extreme close-up), and Sponza — previously fixed and correctly framed — is now also over-zoomed.
2. **Confirmed vertical (Y-axis) flip**: Sponza's render is upside-down. Manually flipping it back top-to-bottom produces a correctly-oriented image — the arch curves the right way (round part on top, not bulging downward), and the floor with potted plants appears at the bottom where it should. Left-right is unaffected — the colored curtains are on their correct sides in both versions. This wasn't present before the FOV safeguard change.

**Both scenes breaking identically, in the same change, in more than one way** points strongly to a shared bug in the camera/FOV code itself (whatever was added for the safeguard) — not independent per-scene authoring mistakes. Treat this as one regression with two symptoms, both needing to be confirmed fixed, not just the more obvious one.

## Debugging steps — isolate before fixing

1. **Log or print the actual computed FOV (or effective vpwidth/vpheight) for both scenes**, before and after the safeguard change, and compare against expected values.
2. **Check for a degrees/radians mixup** if a `fov_degrees` field was added — passing a raw degree value into a trig function expecting radians (or vice versa) would explain a consistently wrong FOV magnitude across every scene using the shared conversion path.
3. **If a sanity-check/clamp approach was used instead**, check whether it's clamping FOV down to a narrow default value rather than just warning.
4. **For the vertical flip specifically**: check whatever code constructs the vertical viewport basis vector, or whatever determines how pixel row index maps to that vertical coordinate, for a sign or ordering mistake — something like a flipped sign on the "up" component, or pixel rows now being iterated in the opposite order from before. This is likely a second, separate mistake introduced in the same change as the magnitude issue — confirm both are fixed independently, not just one.
5. **Confirm whether Sponza is actually included in whatever regression suite was run** before this was reported as complete. If Sponza (or any large/slow real-world scene) isn't part of the fast automated snapshot suite, that's the gap that let both of these ship unnoticed — worth adding some form of check for large "hero" scenes going forward, even if it's a manual re-render step rather than full CI automation.

## Testing

- Confirm both Sponza and the drilled-cube scene render with correct framing **and** correct vertical orientation — check the arch curves the right way and the floor is at the bottom, not just "an image renders at a reasonable zoom level"
- Re-run the full regression gate, explicitly including Sponza this time, not just the fast synthetic-scene suite
- Add a unit test on FOV computation itself: given known `distance`/`vpwidth` or `distance`/`fov_degrees` inputs, confirm the resulting FOV or ray directions match hand-computed values — tests the magnitude issue directly
- Add a unit test for vertical orientation: for a known simple camera setup, confirm the ray direction for the top-row-center pixel and the bottom-row-center pixel point in the expected up/down directions relative to the camera's "up" vector — tests the flip directly, rather than relying on eyeballing a rendered scene. Given how many camera-related bugs this project has now hit (Sponza's FOV twice, the drilled-cube's FOV, and now this flip), this test is worth keeping permanently in the suite, not just as a one-off check for this fix.

## Deliverable

- Root cause of both the magnitude regression and the vertical flip, identified and fixed
- Corrected renders of both Sponza and the drilled-cube scene, with both issues confirmed resolved
- Confirmation of what the regression gate actually covers now, specifically whether Sponza is included
- The new FOV-magnitude and vertical-orientation unit tests
