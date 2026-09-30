# Publishing the website

This directory owns the landing page, four example pages, native run guide,
theory, and browser embedding code. Generated renders and Trunk output belong on
`gh-pages`, not in the source checkout. The published path is `/hypertrace/`, with
the optional viewer under `/hypertrace/viewer/`.

Keep the site factual. Native applications are the main route. Browser instances
start only on request, pause while hidden/offscreen, and report actual startup
failures beside a link to native instructions. Theory uses concise prose with
equations, diagrams, implementation links, and relevant primary references.

Requirements: native and web prerequisites in the root README, Python 3, and
ImageMagick (`magick` or `convert`).

## Capture inputs

The builder reads these filename stems from one directory:

- `hyperbolic`, `euclidean`, `spherical`, `fog`: overview captures.
- `hyperbolic-detail`, `euclidean-detail`, `spherical-detail`, `fog-detail`: separate
  camera views of the same scenes; their JSON `scene` remains the actual scene ID.
- `eu-fog`: the earlier approved capture, retaining its historical scene ID.

Each needs `<capture>.json` and `<capture>.png` or `<capture>.ppm`. PNG takes
priority and is copied byte for byte. Copy the historical PNG and JSON from the
published `gh-pages` checkout without altering their recorded provenance.

Each JSON must include `source_commit`: the full revision of the code that
produced **that image**, recorded at render time after committing the relevant
source changes. It is independent of the current website/viewer revision.
Existing PNG hashes and dimensions are checked. Do not relabel historical images
with newer scene IDs, settings, or source revisions.

For example, capture from a committed checkout:

```sh
mkdir -p /tmp/hypertrace-renders
cargo build --release -p hypertrace-gallery --bin headless
cargo run --release -p hypertrace-gallery --bin headless -- \
  --scene spherical --width 2560 --height 1920 --samples 4096 --batch 1 \
  --seed 3735928559 --output /tmp/hypertrace-renders/spherical
cargo run --release -p hypertrace-gallery --bin headless -- \
  --scene spherical --width 1600 --height 1200 --samples 4096 --batch 1 \
  --fov 0.8 --yaw 0.2 --pitch 0.1 \
  --seed 3735928559 --output /tmp/hypertrace-renders/spherical-detail
python3 - <<'PY'
import json
from pathlib import Path
import subprocess
commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
for capture in ('spherical', 'spherical-detail'):
    path = Path('/tmp/hypertrace-renders') / f'{capture}.json'
    metadata = json.loads(path.read_text())
    metadata['source_commit'] = commit
    path.write_text(json.dumps(metadata, indent=2) + '\n')
PY
```

Use the matching scene ID for the other examples. Choose detail camera angles by
inspecting low-resolution probes, then render fresh pixels at the final size.
The above camera angles are illustrative, not a required layout. `--fov` is the
tangent of half the vertical field of view; yaw/pitch are local radians relative
to the scene's initial camera. Camera and batch settings are written to JSON.

Suggested overview size: 2560×1920; detail size: 1600×1200. Use at least 4096
samples, more for fog/caustics when inspection shows excessive noise. Render one
scene at a time. `--batch 1` bounds dispatch duration; completing each batch also
bounds queued work. Keep the executable used for a capture available throughout
its run; do not clear its target directory during background rendering.

Only PNG/WebP images and metadata are published. Keep linear captures separately
for numerical comparisons. Preview labels derive from metadata, not these
suggested settings. Derivatives are at most 640 and 1280 pixels wide, retain the
aspect ratio, and never upscale a source.

## Validate and build

```sh
python3 -m unittest discover -s tools -p 'test_*.py'
node site/example.test.cjs
node examples/gallery/web/controls.test.cjs
cargo clippy -p hypertrace-gallery --all-targets -- -D warnings
cargo clippy -p hypertrace-gallery --target wasm32-unknown-unknown --features web --bin viewer -- -D warnings
```

Commit source changes before building so source links identify the actual build:

```sh
python3 tools/build_pages.py /tmp/hypertrace-renders --output /tmp/hypertrace-pages
```

The builder validates input provenance, bundles the release viewer, and records
PNG and WebP hashes in `previews/manifest.json`. For another repository path, use
`--public-url /your-repository/`.

Serve the output under its configured path. Check landing cards, screenshot
originals, the native guide, theory equations/diagrams, and both successful viewer
startup and unsupported/error messages. Check narrow screens and browser console
errors. A passing WASM build alone does not establish WebGPU execution.

## Publish

Copy the generated output into a clean `gh-pages` worktree, including `.nojekyll`,
then commit and push that branch when publication is authorized. Keep the website
source commit and every image's `source_commit` reachable in its history so
GitHub source links work. A merge using the `ours` strategy can retain source
history before replacing the worktree contents with the built site.

GitHub Pages uses **Deploy from a branch → gh-pages → / (root)**. The builder does
not push or change hosting settings. Verify the deployed source revision and
image hashes after publication. The site uses local styles, MathML, and SVG;
references are ordinary links. The historical video loads a YouTube privacy-enhanced
embed only after a click and also has a direct link. There are no CDN scripts or fonts.
