# Prompt: Diagnose stale render pipeline (die scene edits not appearing in output)
## Context

Three rounds of uploaded "new" renders of the showcase die have been pixel-identical (or visually indistinguishable) to the very first version — same exact camera framing, same pip shading, same everything. This means the requested fixes (wider framing, pip material tuning, DOF/reflection strength, ivory color) are not actually reaching the rendered output, regardless of whether the scene source was edited. This needs to be diagnosed as a pipeline problem, not treated as another round of aesthetic iteration.

## Diagnostic steps — do these before touching the scene again

1. **Check file modification timestamps** across the whole chain: the `.dsl` source file, the compiled `.json` output, and the final rendered image file. Confirm each is newer than the one before it (JSON newer than the `.dsl` source, image newer than the JSON). If the image file's timestamp predates the JSON, the render definitely did not run against current data — that alone would explain everything.

2. **Diff the actual compiled JSON** against a copy from before the fixes were made. Confirm the camera/material/light fields you changed actually appear differently in the new JSON. If the JSON itself didn't change, the DSL compile step isn't picking up the edited source — check you're compiling the file you think you're editing (right path, no duplicate/stale copy elsewhere in the repo).

3. **Canary test**: make one drastic, impossible-to-miss change — e.g., set the background/environment to solid pure green, or move the camera to an absurdly different position — recompile, re-render, and confirm that specific obvious change actually shows up in the output image. If even a deliberately extreme, unmissable change doesn't appear, that conclusively proves a stale-path/caching issue somewhere in the pipeline rather than "the real fix was just too subtle to see" — don't move past this step until the canary change is visibly confirmed.

## Once the pipeline issue is found and fixed

- Re-apply the original four fixes (framing, pip shading, DOF/reflection visibility, ivory color cast) from the prior prompt
- Re-render and confirm the output actually differs from the original version this time — a direct visual/pixel comparison against the very first render, not just a description of what should have changed

## Deliverable

- Root cause of the stale output (timestamp/path/caching issue — identify which)
- Confirmation via the canary test that the pipeline now actually reflects source changes
- The four original fixes correctly re-applied and rendered, with a genuine visual difference from the original
