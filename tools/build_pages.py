#!/usr/bin/env python3
"""Bundle the viewer, rendered previews and explanatory pages for GitHub Pages."""
import argparse
import hashlib
import html
import json
import os
from pathlib import Path
import re
import shutil
import subprocess


SCENES = ("euclidean", "hyperbolic", "fog")


def run(*args, **kwargs):
    subprocess.run(args, check=True, **kwargs)


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_render(renders, scene, root, convert):
    """Require explicit image provenance, independently of the viewer's revision."""
    settings = json.loads((renders / f"{scene}.json").read_text())
    if settings.get("scene") != scene:
        raise ValueError(f"{scene}: metadata identifies a different scene")
    for key in ("width", "height", "samples", "bounces"):
        value = settings.get(key)
        if type(value) is not int or value <= 0:
            raise ValueError(f"{scene}: {key} must be a positive integer")
    if settings.get("display_gamma") != 2.2:
        raise ValueError(f"{scene}: expected display_gamma=2.2")
    commit = settings.get("source_commit", "")
    if not isinstance(commit, str) or not re.fullmatch(r"[0-9a-f]{40}", commit):
        raise ValueError(f"{scene}: source_commit must identify the exact render source revision")
    subprocess.run(["git", "cat-file", "-e", f"{commit}^{{commit}}"],
                   cwd=root, check=True, capture_output=True)
    image = renders / f"{scene}.png"
    if image.is_file():
        if settings.get("png_sha256") not in (None, sha256(image)):
            raise ValueError(f"{scene}: PNG does not match its recorded hash")
    else:
        image = renders / f"{scene}.ppm"
        if not image.is_file():
            raise ValueError(f"{scene}: missing PNG or PPM")
    dimensions = subprocess.check_output(
        [convert, str(image), "-format", "%w %h", "info:"], text=True).split()
    if dimensions != [str(settings["width"]), str(settings["height"])]:
        raise ValueError(f"{scene}: image dimensions do not match its metadata")
    return image, settings


def encode_preview(image, settings, previews, convert):
    """Preserve approved PNGs byte for byte; derive WebP sizes without upscaling."""
    scene = settings["scene"]
    png = previews / f"{scene}.png"
    if image.suffix == ".png":
        shutil.copy2(image, png)
    else:
        run(convert, str(image), "-strip", "-define", "png:color-type=2", str(png))
    result = dict(settings, png_sha256=sha256(png))
    responsive = []
    for limit, suffix, quality in ((640, "-small", "85"), (1280, "", "90")):
        width = min(limit, settings["width"])
        if responsive and responsive[-1]["width"] == width:
            # A previously larger render may have left this unused derivative.
            (previews / f"{scene}{suffix}.webp").unlink(missing_ok=True)
            continue
        target = previews / f"{scene}{suffix}.webp"
        run(convert, str(png), "-resize", f"{limit}x>", "-strip", "-quality", quality,
            str(target))
        actual = subprocess.check_output(
            [convert, str(target), "-format", "%w %h", "info:"], text=True).split()
        responsive.append({"file": target.name, "width": int(actual[0]),
                           "height": int(actual[1]), "sha256": sha256(target)})
    result["responsive"] = responsive
    (previews / f"{scene}.json").write_text(json.dumps(result, indent=2) + "\n")
    return result


def template_values(settings, commit):
    values = {"__SOURCE_COMMIT__": commit}
    for scene, render in settings.items():
        prefix = scene.upper().replace("-", "_")
        variants = render["responsive"]
        values.update({
            f"__{prefix}_SRCSET__": ", ".join(
                f"previews/{entry['file']} {entry['width']}w" for entry in variants),
            f"__{prefix}_WIDTH__": str(render["width"]),
            f"__{prefix}_HEIGHT__": str(render["height"]),
            f"__{prefix}_SETTINGS__": (
                f"{render['width']} × {render['height']} · {render['samples']} samples per pixel"
                f" · {render['bounces']} bounces"),
            f"__{prefix}_SOURCE_COMMIT__": render["source_commit"],
        })
    return values


def render_template(source, values):
    for key, value in values.items():
        source = source.replace(key, html.escape(value, quote=True))
    if re.search(r"__[A-Z0-9_]+__", source):
        raise ValueError("unresolved gallery template placeholder")
    return source


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("renders", type=Path,
                        help="directory with euclidean, hyperbolic and fog PNG/PPM and JSON metadata")
    parser.add_argument("--output", type=Path, default=Path("build/pages"))
    parser.add_argument("--public-url", default="/hypertrace/")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    renders = args.renders.resolve()
    output = args.output.resolve()
    if output == root or output in root.parents or output == renders or output in renders.parents:
        parser.error("output must be a dedicated publication directory outside the render inputs")
    if not args.public_url.startswith("/") or not args.public_url.endswith("/"):
        parser.error("public URL must be a path beginning and ending with /, e.g. /hypertrace/")
    convert = shutil.which("magick") or shutil.which("convert")
    if not convert:
        parser.error("ImageMagick is required to encode the previews")
    try:
        inputs = {scene: load_render(renders, scene, root, convert) for scene in SCENES}
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()
    if subprocess.check_output(["git", "status", "--porcelain"], cwd=root, text=True).strip():
        parser.error("commit source changes first so the published source links identify this build")
    output.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, NO_COLOR="true")
    run("trunk", "build", "--release", "--public-url", args.public_url + "viewer/",
        "--dist", str(output / "viewer"), cwd=root, env=env)
    previews = output / "previews"
    previews.mkdir(exist_ok=True)
    settings = {scene: encode_preview(image, metadata, previews, convert)
                for scene, (image, metadata) in inputs.items()}
    values = template_values(settings, commit)
    for source in (root / "site").iterdir():
        if source.suffix in (".html", ".css"):
            (output / source.name).write_text(render_template(source.read_text(), values))
    (previews / "manifest.json").write_text(json.dumps(settings, indent=2) + "\n")
    (output / ".nojekyll").touch()
    for name in ("LICENSE-MIT", "LICENSE-APACHE"):
        shutil.copy2(root / name, output / name)
    (output / "README.md").write_text(f"""# Hypertrace website

Website and interactive viewer built from source commit [{commit}](https://github.com/agerasev/hypertrace/tree/{commit}).

- [Scene gallery](https://agerasev.github.io{args.public_url})
- [Hyperbolic viewer](https://agerasev.github.io{args.public_url}viewer/?scene=hyperbolic)
- [Euclidean viewer](https://agerasev.github.io{args.public_url}viewer/?scene=euclidean)
- [Fog viewer](https://agerasev.github.io{args.public_url}viewer/?scene=fog)
- [Theory](https://agerasev.github.io{args.public_url}theory.html)

The `previews/` directory contains original-resolution PNGs, responsive WebP images,
and exact render settings, dimensions, image hashes and source revisions in JSON.
Each preview retains its own render source revision; older previews need not match
the current interactive examples. PNG inputs are preserved byte for byte, and
responsive variants are never upscaled. The `viewer/` directory is a release
WebAssembly build. Linear samples are averaged before display gamma 1/2.2.

To reproduce the site, check out the website source commit and follow `site/README.md`.
To reproduce an image, use its `source_commit` and render settings from `previews/`.
Configure GitHub Pages to **Deploy from a branch**, **gh-pages**, **/ (root)**.
The `.nojekyll` file makes this a static deployment; no build service is needed.
""")
    print(f"Built {output} from {commit}")


if __name__ == "__main__":
    main()
