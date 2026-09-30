# Hypertrace website

Website and interactive viewer built from source commit [10651435cb511d68fdb1977bda261ca6af522f33](https://github.com/agerasev/hypertrace/tree/10651435cb511d68fdb1977bda261ca6af522f33).

- [Scene gallery](https://agerasev.github.io/hypertrace/)
- [Hyperbolic viewer](https://agerasev.github.io/hypertrace/viewer/?scene=hy)
- [Euclidean viewer](https://agerasev.github.io/hypertrace/viewer/?scene=eu)
- [Fog viewer](https://agerasev.github.io/hypertrace/viewer/?scene=eu-fog)
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
