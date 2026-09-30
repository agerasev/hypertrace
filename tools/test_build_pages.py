"""Publication artifact checks, independent of demonstration scene layouts."""
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

from build_pages import (encode_preview, load_render, render_template, sha256,
                         template_values, validate_links)


ROOT = Path(__file__).resolve().parents[1]
CONVERT = shutil.which("magick") or shutil.which("convert")


@unittest.skipUnless(CONVERT, "ImageMagick is required")
class PreviewTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.renders = Path(self.temp.name) / "renders"
        self.previews = Path(self.temp.name) / "previews"
        self.renders.mkdir()
        self.previews.mkdir()
        self.commit = subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()

    def fixture(self, width=40, height=30):
        image = self.renders / "fixture.png"
        subprocess.run([CONVERT, "-size", f"{width}x{height}", "xc:red", str(image)],
                       check=True)
        settings = {"scene": "fixture", "width": width, "height": height,
                    "samples": 32, "bounces": 2, "display_gamma": 2.2,
                    "source_commit": self.commit, "png_sha256": sha256(image)}
        return image, settings

    def load(self, settings):
        (self.renders / "fixture.json").write_text(json.dumps(settings))
        return load_render(self.renders, "fixture", ROOT, CONVERT)

    def test_approved_png_preserves_bytes_and_does_not_upscale(self):
        image, settings = self.fixture()
        actual = encode_preview(*self.load(settings), self.previews, CONVERT)
        self.assertEqual(image.read_bytes(), (self.previews / "fixture.png").read_bytes())
        self.assertEqual(actual["source_commit"], self.commit)
        self.assertEqual([(p["width"], p["height"]) for p in actual["responsive"]], [(40, 30)])
        self.assertFalse((self.previews / "fixture.webp").exists())
        values = template_values({"fixture": actual}, "b" * 40)
        self.assertEqual(values["__FIXTURE_SRCSET__"], "previews/fixture-small.webp 40w")
        self.assertEqual(values["__FIXTURE_SETTINGS__"], "40 × 30 · 32 samples per pixel · 2 path events")
        self.assertEqual(values["__SOURCE_COMMIT__"], "b" * 40)
        self.assertEqual(values["__FIXTURE_SOURCE_COMMIT__"], self.commit)

    def test_large_preview_keeps_proportions_and_reports_real_dimensions(self):
        _, settings = self.fixture(1600, 800)
        actual = encode_preview(*self.load(settings), self.previews, CONVERT)
        self.assertEqual([(p["width"], p["height"]) for p in actual["responsive"]],
                         [(640, 320), (1280, 640)])
        for variant in actual["responsive"]:
            self.assertEqual(variant["sha256"], sha256(self.previews / variant["file"]))

    def test_rejects_image_metadata_mismatch(self):
        _, settings = self.fixture()
        settings["width"] += 1
        with self.assertRaisesRegex(ValueError, "dimensions"):
            self.load(settings)

    def test_rejects_changed_approved_png(self):
        _, settings = self.fixture()
        settings["png_sha256"] = "0" * 64
        with self.assertRaisesRegex(ValueError, "hash"):
            self.load(settings)

    def test_requires_explicit_render_provenance(self):
        _, settings = self.fixture()
        del settings["source_commit"]
        with self.assertRaisesRegex(ValueError, "source_commit"):
            self.load(settings)

    def test_rejects_invalid_render_counts(self):
        _, settings = self.fixture()
        for invalid in (0, -1, 1.5, True):
            with self.subTest(samples=invalid):
                with self.assertRaisesRegex(ValueError, "samples"):
                    self.load(dict(settings, samples=invalid))

    def test_detail_capture_retains_scene_identity_and_source(self):
        image, settings = self.fixture()
        shutil.copy2(image, self.renders / "fixture-detail.png")
        (self.renders / "fixture-detail.json").write_text(json.dumps(settings))
        actual = encode_preview(*load_render(self.renders, "fixture-detail", ROOT,
                                            CONVERT, "fixture"), self.previews, CONVERT)
        self.assertEqual(actual["scene"], "fixture")
        self.assertEqual(actual["source_commit"], self.commit)
        self.assertTrue((self.previews / "fixture-detail.png").is_file())
        values = template_values({"fixture-detail": actual}, "b" * 40)
        self.assertIn("__FIXTURE_DETAIL_SRCSET__", values)
        with self.assertRaisesRegex(ValueError, "different scene"):
            load_render(self.renders, "fixture-detail", ROOT, CONVERT, "wrong-scene")


class TemplateTests(unittest.TestCase):
    def test_escapes_values_and_rejects_missing_metadata(self):
        self.assertEqual(render_template('<img alt="__ALT__">', {"__ALT__": 'a "quote"'}),
                         '<img alt="a &quot;quote&quot;">')
        with self.assertRaisesRegex(ValueError, "unresolved"):
            render_template("__MISSING__", {})

    def test_publication_checks_prefixed_assets_relative_links_and_fragments(self):
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            (output / "image.webp").write_bytes(b"fixture")
            page = output / "index.html"
            page.write_text('<a href="theory.html#rays">Theory</a>'
                            '<img srcset="/project/image.webp 640w">')
            theory = output / "theory.html"
            theory.write_text('<h1 id="rays">Rays</h1><a href="./">Home</a>')
            validate_links(output, "/project/")
            theory.write_text('<h1 id="changed">Rays</h1>')
            with self.assertRaisesRegex(ValueError, "fragment"):
                validate_links(output, "/project/")
            page.write_text('<img src="missing.webp">')
            with self.assertRaisesRegex(ValueError, "missing local target"):
                validate_links(output, "/project/")


if __name__ == "__main__":
    unittest.main()
