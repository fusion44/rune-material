#!/usr/bin/env python3
"""Vendor the Material Symbols Rounded subset used by rune-material."""

from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from urllib.request import urlopen
import xml.etree.ElementTree as ET


REVISION = "10ecc14790612ca8fd164e3f4ffe9c16fc50d5b7"
BASE = f"https://raw.githubusercontent.com/google/material-design-icons/{REVISION}"
DEST = Path(__file__).resolve().parents[1] / "assets" / "material-symbols"
SYMBOLS = (
    "add", "archive", "arrow_drop_down", "check", "close", "code", "description",
    "draft", "folder", "folder_open", "image", "lock", "play_circle", "remove", "search",
)
FILLED = {"folder", "folder_open"}


def download(name):
    variant = "_fill1" if name in FILLED else ""
    url = f"{BASE}/symbols/web/{name}/materialsymbolsrounded/{name}{variant}_24px.svg"
    with urlopen(url, timeout=30) as response:
        source = response.read().decode("utf-8").strip()
    root = ET.fromstring(source)
    if root.attrib.get("viewBox") != "0 -960 960 960":
        raise ValueError(f"Unexpected viewBox for {name}")
    # Rune's Icon accepts SVG bodies but does not apply SVG viewBox offsets.
    # Preserve Google's paths verbatim; map the source coordinates to 24x24.
    body = source.split(">", 1)[1].rsplit("</svg>", 1)[0]
    normalized = (
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">'
        f'<g transform="translate(0 24) scale(0.025)">{body}</g></svg>\n'
    )
    return name, normalized


def main():
    # Fetch everything before writing, so a missing symbol leaves files intact.
    with ThreadPoolExecutor(max_workers=6) as pool:
        symbols = list(pool.map(download, SYMBOLS))
    with urlopen(f"{BASE}/LICENSE", timeout=30) as response:
        license_text = response.read()
    DEST.mkdir(parents=True, exist_ok=True)
    for name, svg in symbols:
        (DEST / f"{name}.svg").write_text(svg, encoding="utf-8")
    (DEST / "LICENSE").write_bytes(license_text)
    print(f"Vendored {len(symbols)} Material Symbols Rounded icons at {REVISION}")


if __name__ == "__main__":
    main()
