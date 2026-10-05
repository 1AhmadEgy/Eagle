#!/usr/bin/env python3
"""Fail-closed repository guard for the current P2P-only application-data policy."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

POLICY_VERSION = "2026-10-05-p2p-only-v1"

EXCLUDED_PREFIXES = (
    ".git/",
    ".github/",
    ".opencode/",
    "archive/",
    "docs/",
    "issues/",
    "scripts/",
    "tests/",
    "infra/",
)

SOURCE_SUFFIXES = {
    ".c", ".cc", ".cpp", ".gradle", ".gradle.kts", ".java", ".js", ".jsx",
    ".kt", ".kts", ".m", ".mm", ".py", ".rs", ".swift", ".ts", ".tsx",
    ".xml",
}

CONFIG_NAMES = {
    "Cargo.toml",
    "build.gradle",
    "build.gradle.kts",
    "settings.gradle",
    "settings.gradle.kts",
    "package.json",
    "package-lock.json",
    "pyproject.toml",
    "requirements.txt",
    "requirements-dev.txt",
    "go.mod",
}

FORBIDDEN_PATTERNS = (
    re.compile(r"\brelay\b", re.IGNORECASE),
    re.compile(r"\bturn(?:s)?://", re.IGNORECASE),
    re.compile(r"\bturnserver\b", re.IGNORECASE),
    re.compile(r"\bwebsocket\b", re.IGNORECASE),
    re.compile(r"\bwss?://", re.IGNORECASE),
    re.compile(r"server[-_ ]mediated", re.IGNORECASE),
    re.compile(r"content[-_ ]forward(?:ing)?", re.IGNORECASE),
    re.compile(r"proxy[-_ ]message", re.IGNORECASE),
)

def is_scanned(path: Path, root: Path) -> bool:
    rel = path.relative_to(root).as_posix()
    if any(rel == prefix.rstrip("/") or rel.startswith(prefix) for prefix in EXCLUDED_PREFIXES):
        return False
    return path.name in CONFIG_NAMES or path.suffix.lower() in SOURCE_SUFFIXES


def scan_file(path: Path, root: Path) -> list[str]:
    if not is_scanned(path, root):
        return []
    try:
        content = path.read_text(encoding="utf-8")
    except (UnicodeDecodeError, OSError):
        return []
    findings: list[str] = []
    for line_no, line in enumerate(content.splitlines(), start=1):
        for pattern in FORBIDDEN_PATTERNS:
            if pattern.search(line):
                findings.append(f"{path.relative_to(root)}:{line_no}: forbidden relay/content-transport indicator")
                break
    return findings


def scan_repository(root: Path) -> list[str]:
    findings: list[str] = []
    for path in root.rglob("*"):
        if path.is_file():
            findings.extend(scan_file(path, root))
    return sorted(findings)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    args = parser.parse_args()
    root = Path(args.root).resolve()
    if not root.is_dir():
        raise SystemExit(f"repository root does not exist: {root}")

    print(f"Relay boundary policy: {POLICY_VERSION}")
    findings = scan_repository(root)
    if findings:
        print("FAIL: forbidden relay/content-transport indicators found:")
        for item in findings:
            print(f"  {item}")
        return 1

    print("PASS: no relay/content-transport runtime indicators found.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
