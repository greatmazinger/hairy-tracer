  I have fully implemented the materials phase, including Fresnel, absorption, textures, and environment maps! The E2E tests
  are perfectly green at default settings, keeping everything mathematically identical to the Python engine when the new
  options are toggled off.

  Here are the requested notes on the implementation details (also saved in material-quality-summary.md):

  1. Image Crate & HDR: I used the image crate (v0.25). It's the standard for Rust and perfectly handles JPEG/PNG/etc. I chose
  not to implement true HDR file support (.hdr/.exr) out-of-the-gate since we just needed something to test the UV and
  spherical lookups. However, the TextureImage struct natively stores its data as DVec3 f64 floats (not u8s), so swapping out
  image::open for image::hdr::HdrDecoder in the future will require exactly zero changes to the rendering pipeline itself!
  2. OBJ vt Parser: The .obj parser was extended to track vt lines, storing the u and v floats. When parsing faces (f v/vt/vn),
  it parses the split components and triangulates the vt indices exactly parallel to the geometric vertices. I added set_uvs
  to the Triangle struct, which scene_parser now calls to inject these UVs per-triangle. Inside Triangle::intersect, the
  Möller-Trumbore intersection now uses barycentric coordinates to interpolate the loaded vertex UVs instead of returning the
  raw barycentric coordinates.
  3. Tests: All structural tests you requested were implemented in a new materials_tests.rs file.

  Are there any specific final features or tweaks you'd like to work on for the core raytracer?

