import json
from pathlib import Path
import tempfile
import unittest

import compare_benchmarks as compare


def run(**changes):
    result = {
        "schema_version": 1, "backend": "wgpu", "device": "Example GPU",
        "device_type": "IntegratedGpu", "driver": "test driver", "backend_api": "Vulkan",
        "scene": "eu", "width": 100, "height": 50, "samples": 4, "bounces": 4,
        "seed": 123, "batch_size": 1, "sync_per_batch": True, "warmup_samples": 2,
        "trials": 3, "setup_ms": 2.0, "device_setup_ms": 1.0,
        "render_ms": [10.0, 30.0, 20.0],
        "readback_ms": [1.0, 3.0, 2.0], "checksum": [1.0, 2.0, 3.0],
    }
    result.update(changes)
    return result


class ComparisonTests(unittest.TestCase):
    def test_units_and_aggregation_use_all_individual_trials(self):
        summary = compare.summarize([
            ("first.json", run()),
            ("second.json", run(trials=1, render_ms=[100.0], readback_ms=[5.0],
                                setup_ms=4.0, device_setup_ms=3.0)),
        ])
        row = summary["groups"][0]["runs"][0]
        self.assertEqual(row["render_ms"], {"median": 25.0, "min": 10.0, "max": 100.0})
        self.assertEqual(row["readback_ms"]["median"], 2.5)
        self.assertEqual(row["setup_ms"]["median"], 3.0)
        self.assertEqual(row["device_setup_ms"], {"median": 2.0, "min": 1.0, "max": 3.0})
        self.assertEqual(row["ms_per_sample"], 6.25)
        self.assertEqual(row["megapixel_samples_per_second"], 0.8)
        self.assertEqual(row["trials"], 4)

    def test_different_workloads_are_not_compared_and_devices_are_labeled(self):
        cpu = run(device="Example CPU", device_type="Cpu")
        summary = compare.summarize([
            ("gpu.json", run()), ("cpu.json", cpu), ("different-seed.json", run(seed=124)),
        ])
        self.assertEqual(len(summary["groups"]), 2)
        comparisons = [c for g in summary["groups"] for c in g["comparisons"]]
        self.assertEqual(len(comparisons), 1)
        self.assertEqual(comparisons[0]["device_comparison"], "cross_device")
        report = compare.markdown(summary)
        self.assertIn("Example CPU", report)
        self.assertIn("Example GPU", report)
        self.assertIn("cross-device comparison", report)

    def test_incomplete_or_incompatible_measurements_are_rejected(self):
        for changes in [
            {"render_ms": [1.0]}, {"render_ms": [1.0, 0.0, 2.0]},
            {"setup_ms": float("nan")}, {"sync_per_batch": False}, {"width": True},
            {"device_setup_ms": -1.0}, {"device_setup_ms": float("inf")},
        ]:
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                compare.validate(run(**changes))

    def test_device_setup_is_required(self):
        measurement = run()
        del measurement["device_setup_ms"]
        with self.assertRaisesRegex(ValueError, "device_setup_ms"):
            compare.validate(measurement)

    def test_directory_cli_deduplicates_files_and_round_trips_reports(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "run.json"
            source.write_text(json.dumps(run()))
            (root / "frame.json").write_text(json.dumps({"backend": "wgpu", "scene": "eu"}))
            markdown = root / "report.md"
            summary = root / "summary.json"
            compare.main([str(root), str(source), "--output", str(markdown), "--json", str(summary)])
            self.assertEqual(json.loads(summary.read_text())["groups"][0]["runs"][0]["trials"], 3)
            self.assertIn("20.000 [10.000–30.000]", markdown.read_text())
            self.assertEqual(len(compare.read_runs([root])), 1)


if __name__ == "__main__":
    unittest.main()
