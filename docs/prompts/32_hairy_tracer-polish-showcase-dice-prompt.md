# Prompt: Polish the showcase die (framing, pip shading, DOF/reflection visibility, ivory color cast)
## Context

Both die renders are technically correct (pip counts and patterns verified right), but four aesthetic issues need fixing before this is genuinely flagship-quality:

1. **Framing is too tight** — the die spills out of frame on the top and left edges. Needs a wider view with real margin on all sides.
2. **Pips read as blown-out/glowing rather than lit** — especially on the red variant, they're nearly flat pure-white with little shading gradient, more like embedded light sources than a lit material.
3. **DOF and environment reflection aren't visually apparent** — two of the capabilities this scene was specifically built to showcase. The background doesn't show clear out-of-focus softness, and the die's surface doesn't show visible sheen/reflection variation suggesting the environment map is contributing to the material's reflection term rather than just serving as backdrop.
4. **The ivory variant reads cool blue-gray instead of warm cream** — likely picking up ambient/environment tint more visibly than the saturated red does, since light near-white surfaces are far more susceptible to visible color cast than saturated ones.

## Scope

**1. Framing**: widen the camera's field of view or pull the camera back so the full die sits comfortably within frame with margin on every side — no edges clipped.

**2. Pip material**: reduce the pip material's brightness/specular intensity so shading gradient (highlight-to-shadow falloff across each spherical cap) is visible, rather than flat blown-out white. The pips should look like a lit white material, not a light source.

**3. DOF and environment reflection — confirm actually active, not just technically present**:
- Check the current aperture/focal-distance values are large enough to produce a visually obvious softness gradient between the in-focus die and the background at this camera distance — increase if the current blur is too subtle to read
- Check whether the die material's reflection term is actually sampling the environment map, or whether the environment map is only being used for the camera-miss/background case. If reflection isn't picking it up, this needs an actual fix, not just a lighting tweak — confirm which case it is before assuming it's just a subtlety issue
- Once both are confirmed active, tune strength so they're clearly visible in the final render — this is the flagship image, subtlety isn't the goal here

**4. Ivory color cast**: either warm the ivory material's base diffuse color slightly to compensate for ambient tint pickup, or shift the fill light/environment map toward a more neutral-warm tone rather than cool gray. Try the material-color fix first since it's more contained and won't affect the red variant's look.

## Testing

- Low-res/low-sample smoke test after each change before committing to full-quality renders, same discipline as every complex scene in this project
- Confirm pip counts/patterns are still correct after any geometry-adjacent changes (framing/camera changes shouldn't touch geometry, but worth a quick visual re-check regardless)
- Final high-quality renders of both red and ivory variants

## Deliverable

- Corrected renders of both variants with all four issues addressed
- Confirmation of whether DOF and environment reflection were already active but too subtle, or were genuinely not being applied — this matters for understanding whether anything else in the scene might have the same "technically present but not visible" issue
