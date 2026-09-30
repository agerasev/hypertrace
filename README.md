# Hypertrace website

Website and interactive viewer built from source commit [ea37cfc5adb2a8d9e1dc325d02115f8dc7d468a3](https://github.com/agerasev/hypertrace/tree/ea37cfc5adb2a8d9e1dc325d02115f8dc7d468a3).

- [Scene gallery](https://agerasev.github.io/hypertrace/)
- [Hyperbolic viewer](https://agerasev.github.io/hypertrace/viewer/?scene=hyperbolic)
- [Euclidean viewer](https://agerasev.github.io/hypertrace/viewer/?scene=euclidean)
- [Spherical viewer](https://agerasev.github.io/hypertrace/viewer/?scene=spherical)
- [Fog viewer](https://agerasev.github.io/hypertrace/viewer/?scene=fog)
- [Build and run locally](https://agerasev.github.io/hypertrace/run.html)
- [Theory](https://agerasev.github.io/hypertrace/theory.html)

The `previews/` directory contains original-resolution PNGs, responsive WebP images,
and exact render settings, dimensions, image hashes and source revisions in JSON.
Each preview retains its own render source revision; older previews need not match
the current interactive examples. PNG inputs are preserved byte for byte, and
responsive variants are never upscaled. The `viewer/` directory is a release
WebAssembly build. Linear samples are averaged before display gamma 1/2.2.

To reproduce the site, check out the website source commit and follow `site/README.md`.
To reproduce an image, use its `source_commit` and render settings from `previews/`.
Configure GitHub Pages to **Deploy from a branch**, **gh-pages**, **/ (root)**.
The `.nojekyll` file makes this a static deployment; no build service is needed.
