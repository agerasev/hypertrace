# Constant-curvature migration baseline

Captured on 2026-09-28 before local dependency substitution or geometry changes.
This records the phase-0 reference for [the migration roadmap](CURVATURE_MIGRATION.md).

## Revisions and environment

| Component | Baseline |
| --- | --- |
| Hypertrace executable source | `292dfc2103052d476412eea5a618a1b16f7e5939` |
| Hypertrace documentation checkpoint | `356cf2fe77e5b1b80d3451160edb8b115334bb12` |
| Registry `ccgeom` actually used | `0.1.0` |
| Registry `vecmat` actually used | `0.7.8` |
| WGPU actually used | `30.0.1` |
| Sibling `ccgeom` checkout, not yet used | `4d64cc01a272e1c3fc2704b580f742ca6114bd88` |
| Sibling `vecmat-rs` checkout, not yet used | `ad071fb952b268b4db1e04960bc95425be7d1b44` |
| Sibling `wgame` checkout | `42858dc961e1ac27a56a5cc84572e7489d036abc` |
| Rust | `rustc 1.98.1 (48a229cea 2026-09-01)`, LLVM `22.1.8` |
| Cargo | `cargo 1.98.1 (797e8a9bc 2026-08-05)` |
| Host | `x86_64-unknown-linux-gnu`, Linux `6.8.0-138-generic` |
| Backend | Vulkan, selected with `WGPU_BACKEND=vulkan` |
| Adapter | `llvmpipe (LLVM 15.0.7, 256 bits)`, device type `Cpu` |
| Driver | `llvmpipe Mesa 23.2.1-1ubuntu3.1~22.04.4 (LLVM 15.0.7)` |

The documentation checkpoint was committed while capture started; it changed
only the roadmap and TODO. Executable sources are identical at both Hypertrace
revisions. This baseline uses registry libraries, not the newer sibling sources.
The optional viewer dependency was not enabled for these headless checks.

## Artifacts

The local artifact directory is
`/tmp/hypertrace-curvature-baseline-20260928`. It contains:

- `source/`: copied workspace source, manifests, comparison tools and lockfile.
- `bin/`: copied release headless/benchmark programs and all 13 test executables.
- `environment.json`, `cargo-metadata.json`, `Cargo.lock`: exact environment,
  dependency graph and resolution used for this capture.
- `logs/`, `test-results-initial.json`, `gpu-test-results.json`, and
  `render-command-results.json`: commands, exit codes, and complete output.
- `eu` and `hy` images, each as `.rgba32f`, `.ppm`, `.png` and settings `.json`;
  `*-repeat` files and comparison reports establish repeatability.
- `camera-poses.json`, the copied tiling fixtures, four benchmark JSON reports,
  and `SHA256SUMS.json` covering the captured files.

The binary and source copies allow baseline runs after the working sources
change. `/tmp` artifacts are local and disposable; preserve this directory
elsewhere before clearing temporary files. The settings, source revisions and
commands below allow reconstruction. The lockfile is captured as evidence;
this does not change the repository policy of ignoring `Cargo.lock`.

## Validation results

`cargo test --workspace` passed: **19 CPU tests, including two doctests**, no
failures; the 19 GPU tests were ignored by that ordinary test run.

All **19 opt-in GPU tests passed**, with no failures. Tests were run from the
copied executables with `--ignored --test-threads=1 --nocapture`; executable
suites were also run sequentially, avoiding competing GPU test workloads.
This is equivalent coverage to:

```sh
WGPU_BACKEND=vulkan cargo test -p hypertrace-wgpu -- --ignored --test-threads=1
```

Coverage includes ABI/layout checks, generated composition and custom leaves,
failed-update recovery and resource limits, geometry math, presentation,
deterministic rendering/batching, and all 12 original hyperbolic tiling fixture
points against both pentastar and pentagonal materials. The exact fixture
coordinates and IDs remain in the captured
`source/renderer-wgpu/tests/tiling.rs`; the successful run is
`logs/gpu-tiling.log`.

No baseline test failures were observed. These are software Vulkan results;
hardware-driver and browser execution were not part of this baseline capture.

## Scene and image reference

Both images use the generic scene builders, default camera and `PointView`
field-of-view parameter `1.0`, 160 × 120 pixels, 32 samples per pixel, seed
`3735928559`, and headless batches of at most 16 samples. Euclidean uses four
bounces; hyperbolic uses three. Linear pixels are little-endian normalized f32
RGBA, top row first. Display previews use gamma `1/2.2`.

The Euclidean camera is the identity rotation with translation
`[0.0, 0.5, 2.0]`. The hyperbolic camera is the original upper-half-space
Möbius transform:

```text
rotate_z(5*pi/6).chain(rotate_x(2*pi/5)).chain(shift_z(2))
```

Composition is `outer.chain(inner) = outer(inner(point))`. Exact constructor
sources are in `camera-poses.json` and the copied scene modules. These two
default poses establish a reference, not exhaustive camera-path coverage.

Reproduce each image from the baseline checkout with:

```sh
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-wgpu --example headless -- \
  --scene eu --width 160 --height 120 --samples 32 --seed 3735928559 \
  --output /tmp/hypertrace-curvature-baseline-20260928/eu
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-wgpu --example headless -- \
  --scene hy --width 160 --height 120 --samples 32 --seed 3735928559 \
  --output /tmp/hypertrace-curvature-baseline-20260928/hy
```

The saved binaries accept the same arguments without rebuilding. Independent
reruns of each scene were **bit-identical**: RGB MAE, RMSE and maximum error all
zero, all 57,600 RGB channels identical, and alpha error zero. Both previews
were visually inspected. Their baseline linear-file SHA-256 hashes are:

| Scene | SHA-256 of `.rgba32f` |
| --- | --- |
| `eu` | `870357d36db741edabe82fe8dadae5dcbff25efed9169f50e4f8ea2e75dd7788` |
| `hy` | `e9bbb64706a44b6058abcb1185d8a86930cb224080691482a4dffa07819e82d9` |

For later comparisons, use `tools/compare_frames.py` on the baseline and new
prefixes with matching settings. Exact equality is established for repeated
baseline runs on this adapter; it is not a promised tolerance across new
coordinate representations. Select and record migration tolerances before
using comparison results as acceptance criteria.

## Release benchmark reference

All four runs used 320 × 240 pixels, 16 samples per trial, 16 warmup samples,
five trials, seed `3735928559`, and the same default camera/bounce settings as
the images. Configurations ran sequentially after the GPU tests and release
build completed. A trial waits for completion after every batch; reset/upload,
setup and readback are excluded from the render timings.

| Scene | Samples per batch | Median completed render, ms |
| --- | ---: | ---: |
| `eu` | 1 | 211.202 |
| `eu` | 16 | 148.293 |
| `hy` | 1 | 227.161 |
| `hy` | 16 | 148.918 |

These are software-renderer measurements on a shared CPU host, not hardware GPU
throughput claims. Individual trial times, adapter/driver, startup, warmup,
readback and checksums are recorded in `benchmark-SCENE-batchN.json`.

For each scene and batch size, the command was:

```sh
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-wgpu --example benchmark -- \
  --scene hy --width 320 --height 240 --samples 16 --warmup 16 --trials 5 \
  --batch 16 --seed 3735928559 \
  --output /tmp/hypertrace-curvature-baseline-20260928/benchmark-hy-batch16.json
```

Change `--scene` and `--batch` to the row being reproduced. No performance
acceptance threshold is inferred from these short baseline measurements.
