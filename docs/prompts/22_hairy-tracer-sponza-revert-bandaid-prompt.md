# Prompt: Revert band-aid fixes, tune exposure, final Sponza render
## Context

The FOV math bug is fixed and Sponza renders correctly now — recognizable structure, correct framing. Before this was diagnosed, two changes were made to try to reduce what was misdiagnosed as convergence noise: forcing all materials to `roughness = 1.0` (fully diffuse, killing any real specular response) and converting area lights to point lights (losing soft shadows). Neither was actually treating the real problem, since the real problem was the FOV. Both are worth reverting now.

## Scope

- Revert material roughness to the values from the actual `.mtl` conversion, rather than the forced `1.0` fallback
- Revert lights back to area lights (with real radius) where that was the original intent, now that the FOV fix means they're being tested against a correctly-framed scene rather than a single zoomed-in texel
- Look at the blown-out/overexposed floor and upper-wall regions in the last render — check whether light intensities need retuning now that the scene is rendering correctly (this may have been tuned somewhat blindly against the broken FOV render and be miscalibrated for the correct one)

## Testing

- Re-render at the same modest settings first (300×300, similar sample count) to confirm reverting the band-aids didn't reintroduce the original fireflies/penumbra noise now that the actual bug (FOV) is fixed — if it does reappear, that would mean roughness/area-lights weren't purely band-aids and need the NEE-firefly-clamping/roughness-threshold work applied properly instead of just reverted
- Once that's confirmed clean, do a proper higher-quality final render — larger resolution and higher sample count than the debugging renders — as the actual showcase image for this milestone

## Deliverable

- Materials and lights reverted to their real intended values
- Confirmation the fireflies/noise don't return once real roughness and area lights are back (or, if they do, a proper fix rather than re-flattening materials)
- Exposure/light intensity retuned if the blown-out regions needed it
- A final high-resolution, high-sample-count render of Sponza
