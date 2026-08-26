# Camera Saga Post-Mortem

## Was it the quaternion look-at, the FOV safeguard, or something else?

**Neither.** Both bugs were arithmetic errors in [`render.rs`](file:///mnt/SDA1/bing/src/hairy-tracer/hairy-tracer-core/src/render.rs), in the scanline loop — completely downstream of the camera orientation math. The `CameraOrientation::from_look_at` Gram-Schmidt code in [`camera.rs`](file:///mnt/SDA1/bing/src/hairy-tracer/hairy-tracer-core/src/camera.rs) was correct throughout, and the quaternion round-trip had passing unit tests to prove it.

---

## Bug 1: FOV Magnitude — Wrong Viewport Anchor

**File:** `render.rs`, scanline loop  
**Root cause:** The viewport center was placed at `look_at` instead of at `distance` units from the camera.

```rust
// BROKEN — viewport center physically sits at look_at
let target_point = look_at + x_scalar * u + y_scalar * v;

// FIXED — viewport center sits at 'distance' in front of the camera
let target_point = cam_origin - w * distance + x_scalar * u + y_scalar * v;
```

This made the effective FOV depend on the *accidental* ratio of `vpwidth` to the distance from camera to `look_at`, not `vpwidth` to `distance`. For Sponza (`vpwidth: 1000`, `distance: 1`, but `|origin - look_at| = 1000`), the two distances happened to cancel and the FOV looked right. Once `distance` defaulted to `1.0` after the refactor, the same `vpwidth: 1000` at `distance: 1` became a ~180° FOV.

**Is this a generalizable risk?** No. The fix is universal and unconditional — every camera angle is now correct by construction. There is no edge case.

---

## Bug 2: Vertical Flip — Wrong Y Scanline Mapping

**File:** `render.rs`, scanline loop  
**Root cause:** Pixel row `ytmp = 0` (the top row of a BMP file) was mapped to `y_scalar = -yright` (the *bottom* of the camera viewport).

```rust
// BROKEN — top of image maps to bottom of viewport
let y_scalar = -yright + ytmp * yd;

// FIXED — top of image maps to top of viewport
let y_scalar = yright - ytmp * yd;
```

This flipped every image vertically. It was invisible in symmetric test scenes (spheres look the same upside-down) but obvious in Sponza where the floor appeared at the top of the frame.

**Is this a generalizable risk?** No. The fix is a single sign flip in the inner loop — completely general. No camera angle can re-trigger it.

---

## What about the quaternion singularity question?

The one real degenerate case in `from_look_at` is when the camera direction (`origin - look_at`) is **exactly parallel to `up`** — e.g. looking straight down with `up = (0,1,0)`. In that case `up.cross(w) = 0` and the fallback kicks in:

```rust
let u = if u_norm == 0.0 {
    DVec3::new(1.0, 0.0, 0.0)  // arbitrary right vector
} else {
    u / u_norm
};
```

This is handled safely and doesn't panic, but the resulting orientation is arbitrary (no unique "right" direction exists when looking straight up/down). **This was not involved in any of the bugs seen during this session.** Both Sponza and the drilled cube cameras had well-conditioned orientations. However, if future scenes use a top-down or bottom-up camera, the rendered image will have an indeterminate roll angle — worth noting for future scene authors.

---

## The `fov_degrees` field

This was an **additive convenience feature**, not a bug fix. It allows scene JSON authors to specify the camera angle in degrees without doing manual trigonometry:

```
vpwidth = 2 * distance * tan(fov_degrees / 2)
```

Both scene JSONs were updated to use it after Bug 1 was fixed, because the old `vpwidth` values had been calibrated for the *broken* formula and were no longer valid.

---

## Summary

| | Cause | Scope | Fixed in |
|---|---|---|---|
| FOV magnitude | Wrong anchor in ray target calculation | Universal (all scenes) | `render.rs` |
| Vertical flip | Inverted y-scanline mapping | Universal (all scenes) | `render.rs` |
| Quaternion singularity | N/A — not involved | Future risk (straight up/down cameras) | Already handled with fallback |

**This camera saga is closed.**
