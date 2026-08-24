hairy-tracer
============

A raytracer that was originally written in Python in 2010 for a Computer Graphics class. It used to be really slow, but it's not so slow anymore.

**Engine Status (August 2026):**
The core rendering engine has been completely ported to Rust (`hairy-tracer-core`) for massive performance gains and new features (Fresnel reflection, volumetric absorption, depth of field, environment maps, procedural/image texturing, anti-aliasing).
- **Rust is now the primary, actively developed engine.** It is used by default.
- The original pure-Python engine is retained under the `--pure-python` flag as an unmaintained historical reference only. It lacks the advanced physical accuracy and features of the Rust core.

Bear in mind the following:
1. While it's not cutting edge, it mostly works.
2. It's incredibly fast now thanks to Rust.
3. It's actively being improved again.

Here are some sample renders:

![Sample 1](example/sample01.jpg)
![Sample 2](example/sample02.jpg)
![Sample 3](example/sample03.jpg)
![Sample 4](example/sample04.jpg)
![Sample 5](example/sample05.jpg)

I put this online as a starting point to start working on it again. And just maybe, someone out there 
might be interested in working on it too.

- 9 October 2014
- Updated 22 August 2026
