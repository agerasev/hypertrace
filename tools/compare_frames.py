#!/usr/bin/env python3
"""Compare Hypertrace linear RGBA32F frames and their JSON sidecars.

Requires Python 3.8+ and only the standard library. Exit status is 0 for a valid
comparison within any supplied thresholds, 1 for exceeded thresholds, or 2 for
invalid input/settings. Run --help for the command-line interface.
"""

import argparse
from array import array
import json
import math
from pathlib import Path
import sys


LINEAR_FORMAT = "little-endian rgba32f, row-major, top row first"
SETTINGS = ("scene", "width", "height", "samples", "seed", "bounces", "linear_format")


def nonnegative(value):
    number = float(value)
    if not math.isfinite(number) or number < 0:
        raise argparse.ArgumentTypeError("expected a finite nonnegative number")
    return number


def frame_paths(value, metadata):
    path = Path(value)
    if path.suffix == ".rgba32f":
        default_metadata = path.with_suffix(".json")
    else:
        default_metadata = Path(str(path) + ".json")
        path = Path(str(path) + ".rgba32f")
    return path, Path(metadata) if metadata else default_metadata


def read_frame(path, metadata_path):
    with metadata_path.open(encoding="utf-8") as handle:
        metadata = json.load(handle)
    if not isinstance(metadata, dict):
        raise ValueError("{}: metadata must be a JSON object".format(metadata_path))
    for key in SETTINGS:
        if key not in metadata:
            raise ValueError("{}: missing metadata field {!r}".format(metadata_path, key))
    for key in ("width", "height", "samples", "bounces"):
        if type(metadata[key]) is not int or metadata[key] <= 0:
            raise ValueError("{}: {} must be a positive integer".format(metadata_path, key))
    if type(metadata["seed"]) is not int or not 0 <= metadata["seed"] <= 0xFFFFFFFF:
        raise ValueError("{}: seed must be a u32 integer".format(metadata_path))
    if not isinstance(metadata["scene"], str) or not metadata["scene"]:
        raise ValueError("{}: scene must be a nonempty string".format(metadata_path))
    if metadata["linear_format"] != LINEAR_FORMAT:
        raise ValueError("{}: unsupported linear_format {!r}".format(metadata_path, metadata["linear_format"]))
    expected = metadata["width"] * metadata["height"] * 16
    actual = path.stat().st_size
    if actual != expected:
        raise ValueError("{}: expected {} bytes from metadata, found {}".format(path, expected, actual))
    values = array("f")
    if values.itemsize != 4:
        raise ValueError("this Python platform does not provide 32-bit array('f')")
    with path.open("rb") as handle:
        values.fromfile(handle, expected // 4)
    if sys.byteorder != "little":
        values.byteswap()
    for index, value in enumerate(values):
        if not math.isfinite(value):
            pixel, channel = divmod(index, 4)
            raise ValueError("{}: non-finite {} at pixel ({}, {})".format(
                path, "RGBA"[channel], pixel % metadata["width"], pixel // metadata["width"]))
    return metadata, values


def compare(reference_metadata, reference, candidate_metadata, candidate):
    mismatches = ["{}: {!r} != {!r}".format(key, reference_metadata[key], candidate_metadata[key])
                  for key in SETTINGS if reference_metadata[key] != candidate_metadata[key]]
    # Camera/FOV fields are optional in the current producers. Compare them if
    # either producer supplies them; silently ignoring future camera data would
    # make an image comparison misleading.
    for key in ("camera", "fov"):
        if reference_metadata.get(key) != candidate_metadata.get(key):
            mismatches.append("{} differs or is missing from one sidecar".format(key))
    if mismatches:
        raise ValueError("incompatible render settings: " + "; ".join(mismatches))
    errors = array("d")
    alpha_error = 0.0
    for index, (left, right) in enumerate(zip(reference, candidate)):
        error = abs(left - right)
        if index % 4 == 3:
            alpha_error = max(alpha_error, error)
        else:
            errors.append(error)
    count = len(errors)
    return {
        "width": reference_metadata["width"],
        "height": reference_metadata["height"],
        "scene": reference_metadata["scene"],
        "samples": reference_metadata["samples"],
        "seed": reference_metadata["seed"],
        "bounces": reference_metadata["bounces"],
        "rgb_mae": math.fsum(errors) / count,
        "rgb_rmse": math.sqrt(math.fsum(value * value for value in errors) / count),
        "rgb_max_error": max(errors),
        "alpha_max_error": alpha_error,
        "identical_rgb_channels": sum(value == 0.0 for value in errors),
        "rgb_channels": count,
    }, errors


def write_difference(path, errors, width, height, scale):
    with Path(path).open("wb") as handle:
        handle.write("P6\n{} {}\n255\n".format(width, height).encode("ascii"))
        # Same display exponent as the reference output, after amplification.
        handle.write(bytes(int(255.0 * min(value * scale, 1.0) ** (1.0 / 2.2)) for value in errors))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("reference", help="reference .rgba32f path or output prefix")
    parser.add_argument("candidate", help="candidate .rgba32f path or output prefix")
    parser.add_argument("--reference-meta", help="override reference JSON sidecar path")
    parser.add_argument("--candidate-meta", help="override candidate JSON sidecar path")
    parser.add_argument("--max-mae", type=nonnegative, help="fail if RGB mean absolute error exceeds this value")
    parser.add_argument("--max-rmse", type=nonnegative, help="fail if RGB root mean square error exceeds this value")
    parser.add_argument("--max-error", type=nonnegative, help="fail if any absolute RGB channel error exceeds this value")
    parser.add_argument("--max-alpha-error", type=nonnegative, help="optional maximum absolute alpha error")
    parser.add_argument("--diff", help="write an RGB absolute-difference PPM")
    parser.add_argument("--diff-scale", type=nonnegative, default=1.0, help="multiply difference values before display gamma (default: 1)")
    parser.add_argument("--report", help="also write metrics and threshold results as JSON")
    args = parser.parse_args(argv)
    try:
        reference_path, reference_meta = frame_paths(args.reference, args.reference_meta)
        candidate_path, candidate_meta = frame_paths(args.candidate, args.candidate_meta)
        metadata_a, values_a = read_frame(reference_path, reference_meta)
        metadata_b, values_b = read_frame(candidate_path, candidate_meta)
        report, errors = compare(metadata_a, values_a, metadata_b, values_b)
        failures = []
        for metric, threshold in (("rgb_mae", args.max_mae), ("rgb_rmse", args.max_rmse),
                                  ("rgb_max_error", args.max_error), ("alpha_max_error", args.max_alpha_error)):
            if threshold is not None and report[metric] > threshold:
                failures.append("{} {:.9g} exceeds {:.9g}".format(metric, report[metric], threshold))
        report.update(reference=str(reference_path), candidate=str(candidate_path),
                      thresholds_passed=not failures, threshold_failures=failures)
        if args.diff:
            write_difference(args.diff, errors, report["width"], report["height"], args.diff_scale)
        if args.report:
            Path(args.report).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        print("{} {}x{}, {} samples, seed {}, {} bounces".format(
            report["scene"], report["width"], report["height"], report["samples"], report["seed"], report["bounces"]))
        print("RGB MAE={:.9g} RMSE={:.9g} max={:.9g}; alpha max={:.9g}".format(
            report["rgb_mae"], report["rgb_rmse"], report["rgb_max_error"], report["alpha_max_error"]))
        print("Identical RGB channels: {}/{}".format(report["identical_rgb_channels"], report["rgb_channels"]))
        for failure in failures:
            print("FAIL: " + failure, file=sys.stderr)
        return 1 if failures else 0
    except (OSError, ValueError, EOFError) as error:
        print("error: {}".format(error), file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
