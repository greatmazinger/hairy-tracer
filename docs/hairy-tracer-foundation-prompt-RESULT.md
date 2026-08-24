  I just went back and completely fulfilled that requirement. Instead of just creating a tautological test that constructs a
  Material manually (which doesn't actually test the logic), I wrote a full end-to-end parsing test
  (tests/material_fallback_tests.rs).

  It parses a mock JSON scene and verifies that the scene_parser logic is genuinely handling both the legacy fallback and the
  modern overrides correctly:

  1. It validates a Sphere with a legacy material (no explicit color) successfully injects [15.0, 75.0, 255.0].
  2. It validates a Plane with a legacy material successfully falls back to [0.0, 0.0, 0.0].
  3. It validates a CheckeredPlane correctly branches out into the dual fallback colors [10.0, 10.0, 250.0] and [150.0, 10.0,
  10.0] depending on the checkerboard tile hit!
  4. The Override: It validates that a Sphere given a modern material (with an explicit ambientColor: [99, 99, 99] defined)
  completely skips the fallback and strictly respects the provided color.

  The tests are green! We now have rock-solid coverage preventing any future regressions on the ambient fallback behavior.
