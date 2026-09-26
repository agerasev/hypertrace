# OpenCL reference frames

The `reference` example renders the same scene builders and initial cameras as
the `eu` and `hy` interactive examples, without opening an SDL window. `hearth`
is outside the migration reference set.

```sh
POCL_CACHE_DIR=/tmp/hypertrace-pocl-cache cargo run -p hypertrace --example reference -- \
  --scene hy --width 320 --height 240 --samples 64 --seed 3735928559 \
  --bounces 3 --output /tmp/hy-reference
```

Use `--scene eu` for the Euclidean scene. The default bounce limit is 4 for `eu`
and 3 for `hy`; `--bounces` accepts 1 through 8. `--list` lists OpenCL devices;
the optional positional platform and device numbers select one. Resolution and
sample count must be positive. Camera transforms are constructed in f64 and
serialized to f32, as in the interactive examples.

If PoCL aborts while loading a cached program, add `POCL_KERNEL_CACHE=0` to the
command to rebuild kernels without using its persistent cache.

Each run writes:

- `PREFIX.rgba32f`: headerless, little-endian f32 RGBA, row-major with the top row
  first. RGB is linear light averaged over the sample count; alpha is 1.
- `PREFIX.ppm`: binary P6 RGB, gamma 1/2.2, clamped to [0,1] and truncated to bytes.
- `PREFIX.json`: scene, dimensions, sample count, seed, and bounce limit.

Compare the linear files when evaluating numerical parity. The gamma image is
for visual inspection. Different devices may still differ through floating-point
rounding and stochastic branches; identical seeds do not imply bitwise parity
between OpenCL and WGPU.

The standard-library Python [comparison tool](../../tools/compare_frames.py)
checks matching settings, file lengths, and finite RGBA values before reporting
linear RGB mean absolute error, root mean square error, and maximum error:

```sh
python3 tools/compare_frames.py /tmp/hy-reference /tmp/hy-wgpu \
  --diff /tmp/hy-difference.ppm --diff-scale 4 --report /tmp/hy-comparison.json
```

Pass output prefixes or `.rgba32f` paths; matching `.json` sidecars are required.
`--max-mae`, `--max-rmse`, and `--max-error` optionally set numeric limits in
linear light. For deterministic reruns, `--max-error 0` requires identical RGB.
Exit status is 0 for a valid comparison within the supplied limits, 1 for exceeded
limits, and 2 for invalid or incompatible inputs. Difference images show absolute
RGB errors multiplied by `--diff-scale`, then gamma 1/2.2 and clamping; they do not
change the reported metrics. There is no universal stochastic image tolerance:
choose thresholds appropriate to the scene and sample count.

For pixel index `i = x + width*y`, the initial u32 seed is computed with wrapping
integer arithmetic:

```text
v = seed XOR ((i + 1) * 0x9e3779b9)
v = (v XOR (v >> 16)) * 0x7feb352d
v = (v XOR (v >> 15)) * 0x846ca68b
v = v XOR (v >> 16)
```

Each random draw then uses `state = 1103515245*state + 12345`, converted to f32
and divided by 2^32, following the existing kernel. The runner explicitly
initializes every pixel state instead of relying on `SmallRng` implementation
details. Repeated runs on the same device use the same camera and random sequence.

The numerical regression test uses an actual OpenCL device and does not require
googletest:

```sh
POCL_CACHE_DIR=/tmp/hypertrace-pocl-cache cargo test -p hypertrace-kernel \
  --test hyperbolic_opencl -- --ignored
```

It checks distances against independent f64 formulas, including close points and
very small/large coordinate scales, and horosphere intersections for vertical,
nearly vertical, grazing, and repeated rays. A missing OpenCL device fails this
explicitly requested test.
