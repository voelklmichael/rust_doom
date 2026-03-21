#!/usr/bin/env python3
"""
Insert `// Original: <c_name>` above each named struct/union field in src/*.rs.
Skips enum variants, _opaque/_placeholder/_private, fields that already have Original above them.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
SRC = REPO / "src"

FIELD_OVERRIDES: dict[tuple[str, str], str] = {
    ("NetModuleT", "init_client"): "InitClient",
    ("NetModuleT", "init_server"): "InitServer",
    ("NetModuleT", "send_packet"): "SendPacket",
    ("NetModuleT", "recv_packet"): "RecvPacket",
    ("NetModuleT", "addr_to_string"): "AddrToString",
    ("NetModuleT", "free_address"): "FreeAddress",
    ("NetModuleT", "resolve_address"): "ResolveAddress",
}

SKIP_FIELDS = frozenset(
    {"_opaque", "_placeholder", "_private", "_ph", "_scratch"}
)

STRUCT_START = re.compile(r"^(\s*)(pub\s+)?struct\s+([A-Za-z0-9_]+)\b")
UNION_START = re.compile(r"^(\s*)(pub\s+)?union\s+([A-Za-z0-9_]+)\b")
FIELD_LINE = re.compile(
    r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:\s*(?!:)"
)


def c_name_for_field(struct_name: str, rust_field: str) -> str:
    if rust_field == "type_":
        return "type"
    return FIELD_OVERRIDES.get((struct_name, rust_field), rust_field)


def should_skip_field(name: str) -> bool:
    return name in SKIP_FIELDS


def has_field_original(lines: list[str], field_idx: int) -> bool:
    j = field_idx - 1
    while j >= 0 and lines[j].strip().startswith("#["):
        j -= 1
    while j >= 0 and lines[j].strip() == "":
        j -= 1
    if j < 0:
        return False
    s = lines[j].strip()
    return s.startswith("// Original:") or s.startswith("/// Original:")


def insert_indent(field_line: str) -> str:
    m = re.match(r"^(\s*)", field_line)
    return m.group(1) if m else "    "


def is_unit_or_tuple_struct_without_body(line: str) -> bool:
    """True for `pub struct Foo;` / `struct Bar;` (no `{` on this line)."""
    st = line.strip()
    if "{" in st:
        return False
    if not st.endswith(";"):
        return False
    return "struct" in st or "union" in st


def find_struct_ranges(lines: list[str]) -> list[tuple[int, int, str]]:
    """(start_line, end_line_inclusive, struct_name) for each struct/union with a braced body."""
    ranges: list[tuple[int, int, str]] = []
    i = 0
    n = len(lines)
    while i < n:
        line = lines[i]
        sm = STRUCT_START.match(line) or UNION_START.match(line)
        if sm:
            struct_name = sm.group(3)
            if is_unit_or_tuple_struct_without_body(line):
                i += 1
                continue
            j = i
            depth = 0
            found_open = False
            while j < n:
                for ch in lines[j]:
                    if ch == "{":
                        depth += 1
                        found_open = True
                    elif ch == "}":
                        depth -= 1
                if found_open and depth == 0:
                    ranges.append((i, j, struct_name))
                    i = j
                    break
                j += 1
        i += 1
    return ranges


def process_struct_block(lines: list[str], struct_name: str) -> tuple[list[str], bool]:
    """
    lines: full struct item from struct/union line through closing }.
    Returns (new_lines, changed).
    """
    out = list(lines)
    changed = False
    depth = 0
    started = False
    i = 0
    while i < len(out):
        line = out[i]
        if not started:
            if "{" in line:
                started = True
                depth = line.count("{") - line.count("}")
            i += 1
            continue

        depth_before = depth
        if depth_before == 1:
            m = FIELD_LINE.match(line)
            if m and not line.strip().startswith("}"):
                fname = m.group(2)
                if not should_skip_field(fname) and not has_field_original(out, i):
                    cname = c_name_for_field(struct_name, fname)
                    ind = insert_indent(line)
                    orig = f"{ind}// Original: {cname}\n"
                    out.insert(i, orig)
                    changed = True
                    i += 2
                    depth += line.count("{") - line.count("}")
                    continue

        depth += line.count("{") - line.count("}")
        i += 1

    return out, changed


def process_file(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)
    ranges = find_struct_ranges(lines)
    if not ranges:
        return False

    changed_any = False
    # Bottom-up so line numbers stay valid
    for start, end, name in sorted(ranges, key=lambda t: t[0], reverse=True):
        block = lines[start : end + 1]
        new_block, ch = process_struct_block(block, name)
        if ch:
            lines[start : end + 1] = new_block
            changed_any = True

    if changed_any:
        path.write_text("".join(lines), encoding="utf-8")
    return changed_any


def main() -> int:
    for p in sorted(SRC.glob("*.rs")):
        process_file(p)
    return 0


if __name__ == "__main__":
    sys.exit(main())
