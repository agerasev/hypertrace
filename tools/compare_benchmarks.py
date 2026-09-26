#!/usr/bin/env python3
"""Summarize synchronized Hypertrace benchmark JSON runs using only the stdlib."""

import argparse
from itertools import combinations
import json
import math
from pathlib import Path
import statistics
import sys


WORKLOAD_FIELDS = ("scene", "width", "height", "samples", "bounces", "seed")
RUN_FIELDS = (
    "backend", "device", "device_type", "driver", "backend_api", "batch_size",
    "sync_per_batch", "warmup_samples",
)
SUMMARY_KIND = "hypertrace_benchmark_summary"


def number(value, field, positive=False):
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ValueError(f"{field} must be a number")
    if not math.isfinite(value) or value < 0 or (positive and value == 0):
        raise ValueError(f"{field} must be finite and {'positive' if positive else 'nonnegative'}")
    return float(value)


def validate(run):
    if (not isinstance(run, dict) or type(run.get("schema_version")) is not int
            or run["schema_version"] != 1):
        raise ValueError("expected a benchmark object with schema_version 1")
    for field in ("backend", "scene", "device", "device_type", "driver", "backend_api"):
        if not isinstance(run.get(field), str) or not run[field]:
            raise ValueError(f"{field} must be a nonempty string")
    for field in ("width", "height", "samples", "bounces", "batch_size", "trials",
                  "seed", "warmup_samples"):
        value = run.get(field)
        minimum = 0 if field in ("seed", "warmup_samples") else 1
        if type(value) is not int or value < minimum:
            raise ValueError(f"{field} must be an integer >= {minimum}")
    if run.get("sync_per_batch") is not True:
        raise ValueError("sync_per_batch must be true for comparable render timings")
    for field in ("render_ms", "readback_ms"):
        values = run.get(field)
        if not isinstance(values, list) or len(values) != run["trials"]:
            raise ValueError(f"{field} length must equal trials")
        for value in values:
            number(value, field, positive=field == "render_ms")
    number(run.get("setup_ms"), "setup_ms")
    if "device_setup_ms" in run:
        number(run["device_setup_ms"], "device_setup_ms")
    checksum = run.get("checksum")
    if not isinstance(checksum, list) or len(checksum) != 3:
        raise ValueError("checksum must contain three finite RGB sums")
    for value in checksum:
        if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
            raise ValueError("checksum must contain three finite RGB sums")
    return run


def read_runs(inputs):
    runs = []
    seen = set()
    for item in inputs:
        item = Path(item)
        paths = sorted(item.glob("*.json")) if item.is_dir() else [item]
        for path in paths:
            path = path.resolve()
            if path in seen:
                continue
            seen.add(path)
            try:
                run = json.loads(path.read_text(encoding="utf-8"))
                # A results directory may also contain this tool's summaries
                # and machine inventory. Explicit file inputs remain strict.
                if item.is_dir() and isinstance(run, dict) and (
                    run.get("kind") == SUMMARY_KIND
                    or not ("render_ms" in run
                            or (run.get("schema_version") == 1 and "backend" in run))
                ):
                    continue
                runs.append((str(path), validate(run)))
            except (OSError, ValueError, TypeError) as error:
                raise ValueError(f"{path}: {error}") from error
    if not runs:
        raise ValueError("no benchmark runs found")
    return runs


def distribution(values):
    return {"median": statistics.median(values), "min": min(values), "max": max(values)}


def summarize(runs):
    groups = {}
    for path, run in runs:
        workload = tuple(run[field] for field in WORKLOAD_FIELDS)
        identity = tuple(run[field] for field in RUN_FIELDS)
        groups.setdefault(workload, {}).setdefault(identity, []).append((path, run))
    result = {"schema_version": 1, "kind": SUMMARY_KIND, "groups": []}
    for workload, implementations in sorted(groups.items()):
        group = {"workload": dict(zip(WORKLOAD_FIELDS, workload)), "runs": [], "comparisons": []}
        for index, (identity, measurements) in enumerate(sorted(implementations.items()), 1):
            row = dict(zip(RUN_FIELDS, identity))
            row["id"] = f"r{index}"
            row["input_files"] = [path for path, _ in measurements]
            row["invocations"] = len(measurements)
            row["trials"] = sum(run["trials"] for _, run in measurements)
            for field in ("render_ms", "readback_ms"):
                row[field] = distribution([value for _, run in measurements for value in run[field]])
            row["setup_ms"] = distribution([run["setup_ms"] for _, run in measurements])
            device_setup = [run["device_setup_ms"] for _, run in measurements if "device_setup_ms" in run]
            if device_setup:
                row["device_setup_ms"] = distribution(device_setup)
                row["device_setup_measurements"] = len(device_setup)
            row["checksums"] = [run["checksum"] for _, run in measurements]
            render_ms = row["render_ms"]["median"]
            work = group["workload"]
            row["ms_per_sample"] = render_ms / work["samples"]
            row["megapixel_samples_per_second"] = (
                work["width"] * work["height"] * work["samples"] / (render_ms * 1000)
            )
            group["runs"].append(row)
        for a, b in combinations(group["runs"], 2):
            same_device = (a["device"], a["device_type"]) == (b["device"], b["device_type"])
            group["comparisons"].append({
                "numerator": a["id"], "denominator": b["id"],
                "render_time_ratio": a["render_ms"]["median"] / b["render_ms"]["median"],
                "device_comparison": "same_reported_device" if same_device else "cross_device",
            })
        result["groups"].append(group)
    return result


def cell(value):
    return str(value).replace("|", "\\|").replace("\n", " ")


def timing(summary):
    return f'{summary["median"]:.3f} [{summary["min"]:.3f}–{summary["max"]:.3f}]'


def markdown(summary):
    lines = [
        "# Hypertrace benchmark comparison", "",
        "Times are milliseconds: median [minimum–maximum]. Render timing includes a completion "
        "wait after every batch and excludes readback. Setup excludes adapter/device creation; "
        "device setup is shown separately when recorded. Warmup samples are excluded from timing.", "",
        "One sample means one full-frame sample. Throughput counts width × height × samples. "
        "Only identical scene, dimensions, sample count, bounce limit, and seed share a group. "
        "Different device labels are marked as cross-device comparisons, not backend-only speedups.",
    ]
    for group in summary["groups"]:
        w = group["workload"]
        lines += [
            "", f'## {cell(w["scene"])} · {w["width"]}×{w["height"]} · {w["samples"]} samples', "",
            f'Bounces: {w["bounces"]}; seed: {w["seed"]}.', "",
            "| ID | Backend / API | Device | Batch | Trials | Render ms | Readback ms | Setup ms | ms/sample | MPixelSamples/s |",
            "| --- | --- | --- | ---: | ---: | --- | --- | --- | ---: | ---: |",
        ]
        for row in group["runs"]:
            lines.append(
                f'| {row["id"]} | {cell(row["backend"])} / {cell(row["backend_api"])} '
                f'| {cell(row["device"])} | {row["batch_size"]} | {row["trials"]} '
                f'| {timing(row["render_ms"])} | {timing(row["readback_ms"])} '
                f'| {timing(row["setup_ms"])} | {row["ms_per_sample"]:.3f} '
                f'| {row["megapixel_samples_per_second"]:.3f} |'
            )
        lines.append("")
        for row in group["runs"]:
            detail = (
                f'- **{row["id"]}**: {cell(row["device_type"])}; driver {cell(row["driver"])}; '
                f'{row["warmup_samples"]} warmup samples; {row["invocations"]} invocation(s).'
            )
            if "device_setup_ms" in row:
                detail += f' Device setup ms: {timing(row["device_setup_ms"])}.'
            lines.append(detail)
        if group["comparisons"]:
            lines += ["", "Render-time ratios use the table IDs (numerator ÷ denominator):", ""]
            for comparison in group["comparisons"]:
                label = ("cross-device comparison; hardware equivalence is unverified"
                         if comparison["device_comparison"] == "cross_device"
                         else "same reported device")
                lines.append(
                    f'- {comparison["numerator"]} ÷ {comparison["denominator"]}: '
                    f'{comparison["render_time_ratio"]:.3f}× ({label}).'
                )
    lines += ["", "Checksums are retained in JSON for diagnostics; this timing summary does not establish image parity.", ""]
    return "\n".join(lines)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("inputs", nargs="+", type=Path, help="benchmark JSON files or directories")
    parser.add_argument("--output", type=Path, help="write Markdown instead of standard output")
    parser.add_argument("--json", dest="json_output", type=Path, help="also write structured aggregate JSON")
    args = parser.parse_args(argv)
    try:
        summary = summarize(read_runs(args.inputs))
        report = markdown(summary)
        if args.output:
            args.output.write_text(report, encoding="utf-8")
        else:
            sys.stdout.write(report)
        if args.json_output:
            args.json_output.write_text(
                json.dumps(summary, indent=2, allow_nan=False) + "\n", encoding="utf-8"
            )
    except (OSError, ValueError) as error:
        parser.error(str(error))


if __name__ == "__main__":
    main()
