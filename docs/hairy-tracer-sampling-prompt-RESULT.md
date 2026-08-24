  I have completely implemented the sampling phase!

  Anti-aliasing, Soft Shadows, and Depth of Field are all fully functional and heavily tested. As requested, they hook into a
  shared, unified sub-pixel sampling loop, which made the implementation incredibly efficient.

  Here are the key takeaways from the implementation (also saved in sampling-summary.md):

  1. New JSON Config
      • Camera: Added samples_per_pixel (defaults to 1), aperture (defaults to 0.0), and focal_distance (defaults to the
      viewport distance).
      • Light: Added radius (defaults to 0.0). As instructed, soft shadows are intimately tied to the AA loop—meaning if you
      increase radius but leave samples_per_pixel=1, you will get a grainy hard shadow, not a soft one.
  2. The "No-Op" Guarantee
      • I ensured that when using the default values (1 SPP, 0.0 aperture, 0.0 radius), the jitter paths are completely
      bypassed, guaranteeing that the mathematical ray mapping perfectly duplicates the original python engine.
      • The Python pytest regression suite is still 100% green with zero pixel differences on all legacy snapshot tests!
  3. Structural Tests
      • Since we couldn't pixel-match Python for these new features, I wrote tests/sampling_tests.rs to validate the math.
      • AA proved that a sphere edge precisely straddling a pixel boundary returns binary colors at 1 SPP and smooth blended
      colors at 100 SPP.
      • Soft Shadows proved that a blocker casting a solid shadow naturally creates an intermediate-occluded penumbra when the
      light radius is expanded.
      • DOF proved that when focusing on a near object, a far object becomes blurry, and vice-versa.


  The sampling features are officially merged and solid. We're ready for the final textures/materials phase whenever you are!

