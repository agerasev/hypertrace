# Publishing the gallery and web viewer

This directory contains the gallery, theory page, and shared styles. Rendered
previews and Trunk output are generated artifacts, stored on `gh-pages` rather
than in the source checkout. The published site is designed for the repository
path `/hypertrace/`, with its interactive viewer under `/hypertrace/viewer/`.

Requirements: the native renderer and web build prerequisites in the root
README, Python 3, and ImageMagick (`magick` or `convert`).

## Preview inputs and provenance

The builder reads `eu`, `hy`, and `eu-fog` previews from one directory. Each needs
`<scene>.json` from the headless renderer and either `<scene>.png` or
`<scene>.ppm`. PNG takes priority and is copied byte for byte, so an approved
screenshot can be published without rerendering or re-encoding it.

Each JSON must also contain `source_commit`: the full Git revision of the code
that produced **that image**. Record it at render time, after committing its
source changes. Do not replace older images' revisions with the website's latest
revision. Existing `gh-pages/previews/*.png` and their matching JSON files can be
copied into the input directory unchanged; the recorded PNG hashes are checked.

For example, to capture fresh previews from a clean, committed checkout:

```sh
mkdir -p build/previews
cargo build --release -p hypertrace-gallery --bin headless
for scene in eu hy; do
  cargo run --release -p hypertrace-gallery --bin headless -- \
    --scene "$scene" --width 2560 --height 1920 --samples 4096 \
    --seed 3735928559 --output "build/previews/$scene"
done
cargo run --release -p hypertrace-gallery --bin headless -- \
  --scene eu-fog --width 640 --height 480 --samples 32768 \
  --seed 3735928559 --output build/previews/eu-fog
python3 - <<'PY'
import json
from pathlib import Path
import subprocess
commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
for scene in ('eu', 'hy', 'eu-fog'):
    path = Path('build/previews') / f'{scene}.json'
    metadata = json.loads(path.read_text())
    metadata['source_commit'] = commit
    path.write_text(json.dumps(metadata, indent=2) + '\n')
PY
```

These dimensions and sample counts are suggestions; gallery labels and responsive
image widths come from the supplied metadata and actual image dimensions. Render
one scene at a time. The native renderer bounds its GPU queue for long renders
and reports progress every 128 samples. Outputs include linear float pixels, a
display-ready PPM, and JSON metadata; only PNG/WebP previews and metadata are
published. Retain the linear captures separately when they are useful for analysis.

## Build and publish

Commit website and viewer changes before building, so their source links point
to an exact revision. Then build:

```sh
python3 tools/build_pages.py build/previews --output build/pages
```

The builder verifies preview dimensions and provenance, generates the release
viewer with the correct URL prefix, and records PNG and WebP SHA-256 hashes.
Responsive previews are at most 640 and 1280 pixels wide, keeping the original
aspect ratio and never enlarging a smaller source image. The gallery links to
each image's own source revision, which may differ from the current viewer.
The builder does not push branches or change GitHub settings.

Copy the contents of `build/pages/` into the root of the `gh-pages` branch,
including `.nojekyll`, and commit and push that branch. Keep the website source
commit and every preview's `source_commit` reachable in its history so source
links continue to work. Configure **Settings → Pages → Deploy from a branch →
gh-pages → / (root)**.

For a different repository path, pass `--public-url /your-repository/`.
The generated site is self-contained apart from ordinary links to GitHub,
the original article, and the YouTube video; it requires no CDN scripts or fonts.
