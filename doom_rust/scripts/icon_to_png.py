#!/usr/bin/env python3
"""Convert icon.c RGB data to PNG. Reads doomgeneric/doomgeneric/icon.c."""
import re
from pathlib import Path

# Try Pillow first, fall back to png module
try:
    from PIL import Image
    HAS_PIL = True
except ImportError:
    HAS_PIL = False

def main():
    base = Path(__file__).resolve().parent.parent.parent
    icon_c = base / "doomgeneric" / "doomgeneric" / "icon.c"
    out_png = base / "doom_rust" / "asserts" / "icon.png"

    text = icon_c.read_text()
    # Extract all 0xXX hex values
    hex_vals = re.findall(r"0x([0-9a-fA-F]+)", text)
    bytes_data = bytes(int(h, 16) for h in hex_vals)

    w, h = 32, 32
    expected = w * h * 3
    if len(bytes_data) < expected:
        print(f"Warning: got {len(bytes_data)} bytes, expected {expected}")
        bytes_data = bytes_data.ljust(expected, b"\x00")
    elif len(bytes_data) > expected:
        bytes_data = bytes_data[:expected]

    # Build RGBA: black (0,0,0) -> transparent
    pixels = []
    for i in range(0, len(bytes_data), 3):
        r, g, b = bytes_data[i], bytes_data[i+1], bytes_data[i+2]
        if r == 0 and g == 0 and b == 0:
            pixels.append((0, 0, 0, 0))
        else:
            pixels.append((r, g, b, 255))

    if HAS_PIL:
        img = Image.new("RGBA", (w, h))
        img.putdata(pixels)
        img.save(out_png)
        print(f"Saved {out_png}")
    else:
        # Fallback: use raw PNG writing (minimal)
        import zlib
        import struct

        def png_chunk(chunk_type, data):
            return struct.pack(">I", len(data)) + chunk_type + data + struct.pack(">I", zlib.crc32(chunk_type + data) & 0xffffffff)

        raw = b""
        for y in range(h):
            raw += b"\x00"  # filter
            for x in range(w):
                p = pixels[y * w + x]
                raw += bytes(p)

        compressed = zlib.compress(raw, 9)
        signature = b"\x89PNG\r\n\x1a\n"
        ihdr = struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0)
        idat = png_chunk(b"IDAT", compressed)
        iend = png_chunk(b"IEND", b"")

        out_png.write_bytes(signature + png_chunk(b"IHDR", ihdr) + idat + iend)
        print(f"Saved {out_png} (no PIL)")

if __name__ == "__main__":
    main()
