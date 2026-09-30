import contextlib
import io
import json
from pathlib import Path
import struct
import tempfile
import unittest

import compare_frames as compare


def metadata(**changes):
    result = {
        "scene": "spherical", "width": 1, "height": 1, "samples": 4,
        "seed": 123, "bounces": 6, "linear_format": compare.LINEAR_FORMAT,
        "curvature_sign": 1, "curvature_radius": 2.0,
        "medium": {"extinction": 0.08, "albedo": [0.85, 0.9, 0.95]},
    }
    result.update(changes)
    return result


class ComparisonTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)

    def write_frame(self, name, settings):
        path = self.root / (name + ".rgba32f")
        sidecar = path.with_suffix(".json")
        path.write_bytes(struct.pack("<4f", 0.25, 0.5, 0.75, 1.0))
        sidecar.write_text(json.dumps(settings))
        return path, sidecar

    def test_current_metadata_round_trips_and_identical_frames_match(self):
        source = metadata()
        loaded, pixels = compare.read_frame(*self.write_frame("current", source))
        self.assertEqual(loaded, source)
        report, _ = compare.compare(loaded, pixels, loaded, pixels)
        self.assertEqual(report["rgb_max_error"], 0)
        self.assertEqual(report["identical_rgb_channels"], 3)

    def test_old_sidecars_without_physical_settings_are_rejected(self):
        for field in ("curvature_sign", "curvature_radius", "medium"):
            source = metadata()
            del source[field]
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, field):
                compare.read_frame(*self.write_frame("old", source))

    def test_different_physical_settings_are_not_compared(self):
        reference, pixels = compare.read_frame(*self.write_frame("reference", metadata()))
        for changes in (
            {"curvature_sign": -1},
            {"curvature_radius": 3.0},
            {"medium": {"extinction": 0.1, "albedo": [0.85, 0.9, 0.95]}},
            {"medium": {"extinction": 0.08, "albedo": [0.0, 0.9, 0.95]}},
        ):
            candidate, values = compare.read_frame(*self.write_frame("candidate", metadata(**changes)))
            with self.subTest(changes=changes), self.assertRaisesRegex(ValueError, next(iter(changes))):
                compare.compare(reference, pixels, candidate, values)

    def test_invalid_curvature_and_medium_values_are_rejected(self):
        for changes in (
            {"curvature_sign": True}, {"curvature_sign": 2},
            {"curvature_radius": 0}, {"curvature_radius": float("nan")},
            {"curvature_radius": True}, {"curvature_sign": 0},
            {"medium": None}, {"medium": {}},
            {"medium": {"extinction": -1.0, "albedo": [1.0, 1.0, 1.0]}},
            {"medium": {"extinction": float("inf"), "albedo": [1.0, 1.0, 1.0]}},
            {"medium": {"extinction": 0.0, "albedo": [1.0, 1.0]}},
            {"medium": {"extinction": 0.0, "albedo": [1.0, 1.0, 1.1]}},
            {"medium": {"extinction": 0.0, "albedo": [1.0, True, 1.0]}},
        ):
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                compare.read_frame(*self.write_frame("invalid", metadata(**changes)))

    def test_cli_reports_physical_mismatch_as_invalid_input(self):
        reference, _ = self.write_frame("reference", metadata())
        candidate, _ = self.write_frame("candidate", metadata(curvature_radius=3.0))
        errors = io.StringIO()
        with contextlib.redirect_stderr(errors):
            status = compare.main([str(reference), str(candidate)])
        self.assertEqual(status, 2)
        self.assertIn("curvature_radius", errors.getvalue())


if __name__ == "__main__":
    unittest.main()
