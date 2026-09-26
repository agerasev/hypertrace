# Hypertrace website

Static publication built from source commit [292dfc2103052d476412eea5a618a1b16f7e5939](https://github.com/agerasev/hypertrace/tree/292dfc2103052d476412eea5a618a1b16f7e5939).

- [Scene gallery](https://agerasev.github.io/hypertrace/)
- [Hyperbolic viewer](https://agerasev.github.io/hypertrace/viewer/?scene=hy)
- [Euclidean viewer](https://agerasev.github.io/hypertrace/viewer/?scene=eu)
- [Theory](https://agerasev.github.io/hypertrace/theory.html)

The `previews/` directory contains full-resolution PNGs, 1280 × 960 WebP images,
640 × 480 low-resolution previews,
and exact render settings. The `viewer/` directory is a release WebAssembly build.
Images retain the default scenes' cameras and bounce limits, at 2560 × 1920 and
4096 samples per pixel. Linear samples are averaged before display gamma 1/2.2.

To reproduce, check out the source commit and follow `site/README.md`.
Configure GitHub Pages to **Deploy from a branch**, **gh-pages**, **/ (root)**.
The `.nojekyll` file makes this a static deployment; no build service is needed.
