# Publishing the gallery and web viewer

This directory contains the gallery, theory page, and shared styles. Rendered
previews and Trunk output are generated artifacts, stored on `gh-pages` rather
than in the source checkout. The published site is designed for the repository
path `/hypertrace/`, with its interactive viewer under `/hypertrace/viewer/`.

Requirements: the native renderer and web build prerequisites in the root
README, Python 3, and ImageMagick (`magick` or `convert`).

From the repository root, render both previews on a compute-capable GPU:

```sh
mkdir -p build/previews
cargo build --release -p hypertrace-renderer --example headless
for scene in eu hy; do
  cargo run --release -p hypertrace-renderer --example headless -- \
    --scene "$scene" --width 2560 --height 1920 --samples 4096 \
    --seed 3735928559 --output "build/previews/$scene"
done
```

The native renderer bounds its GPU queue for long renders and reports progress
every 128 samples. Render one scene at a time. The outputs include linear float
pixels, a display-ready PPM, and JSON metadata; only encoded PNG/WebP previews
and metadata are published.

Commit any source changes before building, so the site's source links point to
an exact, reproducible commit. Then build:

```sh
python3 tools/build_pages.py build/previews --output build/pages
```

The builder verifies that the settings match the gallery labels, generates the
release viewer with the correct URL prefix, encodes the previews, and records
their SHA-256 hashes. Full-resolution PNGs are accompanied by 1280 × 960 and
640 × 480 WebP previews derived from the same high-sample renders. The gallery
uses responsive image selection and includes direct links to the small previews.
The builder does not push branches or change GitHub settings.

Copy the contents of `build/pages/` into the root of the `gh-pages` branch,
including `.nojekyll`, and commit and push that branch. Keep the source commit
reachable in its history so the links to the implementation continue to work.
Configure **Settings → Pages → Deploy from a branch → gh-pages → / (root)**.

For a different repository path, pass `--public-url /your-repository/`.
The generated site is self-contained apart from ordinary links to GitHub,
the original article, and the YouTube video; it requires no CDN scripts or fonts.
