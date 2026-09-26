#!/usr/bin/env python3
"""Bundle the viewer, rendered previews and explanatory pages for GitHub Pages."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess


def run(*args, **kwargs):
    subprocess.run(args, check=True, **kwargs)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("renders", type=Path, help="directory with eu/hy.ppm and eu/hy.json")
    parser.add_argument("--output", type=Path, default=Path("build/pages"))
    parser.add_argument("--public-url", default="/hypertrace/")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    renders = args.renders.resolve()
    output = args.output.resolve()
    if output == root or output in root.parents:
        parser.error("output must be a dedicated publication directory")
    if not args.public_url.startswith("/") or not args.public_url.endswith("/"):
        parser.error("public URL must be a path beginning and ending with /, e.g. /hypertrace/")
    convert = shutil.which("magick") or shutil.which("convert")
    if not convert:
        parser.error("ImageMagick is required to encode the previews")
    settings = {}
    for scene, bounces in (("eu", 4), ("hy", 3)):
        settings[scene] = json.loads((renders / f"{scene}.json").read_text())
        expected = {"scene": scene, "width": 2560, "height": 1920,
                    "samples": 4096, "bounces": bounces, "display_gamma": 2.2}
        for key, value in expected.items():
            if settings[scene].get(key) != value:
                parser.error(f"{scene}: expected {key}={value}, matching the gallery labels")
        if not (renders / f"{scene}.ppm").is_file():
            parser.error(f"missing {scene}.ppm")
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()
    if subprocess.check_output(["git", "status", "--porcelain"], cwd=root, text=True).strip():
        parser.error("commit source changes first so the published source links identify this build")
    output.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, NO_COLOR="true")
    run("trunk", "build", "--release", "--public-url", args.public_url + "viewer/",
        "--dist", str(output / "viewer"), cwd=root, env=env)
    for source in (root / "site").iterdir():
        if source.suffix in (".html", ".css"):
            (output / source.name).write_text(source.read_text().replace("__SOURCE_COMMIT__", commit))
    previews = output / "previews"
    previews.mkdir(exist_ok=True)
    for scene in settings:
        run(convert, str(renders / f"{scene}.ppm"), "-strip", "-define", "png:color-type=2",
            str(previews / f"{scene}.png"))
        run(convert, str(renders / f"{scene}.ppm"), "-resize", "1280x960", "-strip",
            "-quality", "90", str(previews / f"{scene}.webp"))
        run(convert, str(renders / f"{scene}.ppm"), "-resize", "640x480", "-strip",
            "-quality", "85", str(previews / f"{scene}-small.webp"))
        settings[scene]["png_sha256"] = hashlib.sha256((previews / f"{scene}.png").read_bytes()).hexdigest()
        settings[scene]["source_commit"] = commit
        (previews / f"{scene}.json").write_text(json.dumps(settings[scene], indent=2) + "\n")
    (previews / "manifest.json").write_text(json.dumps(settings, indent=2) + "\n")
    (output / ".nojekyll").touch()
    for name in ("LICENSE-MIT", "LICENSE-APACHE"):
        shutil.copy2(root / name, output / name)
    (output / "README.md").write_text(f"""# Hypertrace website

Static publication built from source commit [{commit}](https://github.com/agerasev/hypertrace/tree/{commit}).

- [Scene gallery](https://agerasev.github.io{args.public_url})
- [Hyperbolic viewer](https://agerasev.github.io{args.public_url}viewer/?scene=hy)
- [Euclidean viewer](https://agerasev.github.io{args.public_url}viewer/?scene=eu)
- [Theory](https://agerasev.github.io{args.public_url}theory.html)

The `previews/` directory contains full-resolution PNGs, 1280 × 960 WebP images,
640 × 480 low-resolution previews,
and exact render settings. The `viewer/` directory is a release WebAssembly build.
Images retain the default scenes' cameras and bounce limits, at 2560 × 1920 and
4096 samples per pixel. Linear samples are averaged before display gamma 1/2.2.

To reproduce, check out the source commit and follow `site/README.md`.
Configure GitHub Pages to **Deploy from a branch**, **gh-pages**, **/ (root)**.
The `.nojekyll` file makes this a static deployment; no build service is needed.
""")
    print(f"Built {output} from {commit}")


if __name__ == "__main__":
    main()
