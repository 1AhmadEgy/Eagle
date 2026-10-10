#!/usr/bin/env python3
"""Fail-closed repository guard for Eagle's current P2P-only application-data policy."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

POLICY_VERSION = "2026-10-10-p2p-only-v2"

SCAN_ROOTS = {
    ".github/workflows",
    "app",
    "androidApp",
    "core",
    "desktopApp",
    "deploy",
    "deployment",
    "docker",
    "gradle",
    "helm",
    "infra",
    "infrastructure",
    "iosApp",
    "k8s",
    "kubernetes",
    "scripts",
    "shared",
    "src",
    "terraform",
}

EXCLUDED_PARTS = {
    ".git",
    ".gradle",
    ".opencode",
    ".ci",
    "archive",
    "build",
    "docs",
    "issues",
    "node_modules",
    "target",
    "tests",
}

EXCLUDED_TEST_DIRS = {"test", "tests", "androidTest", "androidtest"}

TEXT_SUFFIXES = {
    ".bash", ".c", ".cc", ".conf", ".cpp", ".env", ".gradle", ".h", ".hpp",
    ".hcl", ".ini", ".java", ".js", ".json", ".jsx", ".kt", ".kts", ".lock",
    ".m", ".mm", ".mod", ".properties", ".py", ".rs", ".sh", ".sum", ".swift",
    ".tf", ".tfvars", ".toml", ".ts", ".tsx", ".txt", ".xml", ".yaml", ".yml",
}

CONFIG_NAMES = {
    "Cargo.lock",
    "Cargo.toml",
    "Chart.yaml",
    "Dockerfile",
    "Gemfile",
    "Gemfile.lock",
    "go.mod",
    "go.sum",
    "gradle.lockfile",
    "gradle.properties",
    "libs.versions.toml",
    "package-lock.json",
    "package.json",
    "pnpm-lock.yaml",
    "pom.xml",
    "pyproject.toml",
    "requirements-dev.txt",
    "requirements.txt",
    "settings.gradle",
    "settings.gradle.kts",
    "build.gradle",
    "build.gradle.kts",
    "docker-compose.yml",
    "docker-compose.yaml",
    "compose.yml",
    "compose.yaml",
    "kustomization.yaml",
    "values.yaml",
}

SCANNER_RELATIVE_PATH = "scripts/ci/verify-relay-boundary.py"

FORBIDDEN_PATTERNS = (
    re.compile(r"\bturns?://", re.IGNORECASE),
    re.compile(r"\b(?:coturn|turnserver|turn[-_]?server)\b", re.IGNORECASE),
    re.compile(r"\biceTransportPolicy\s*[:=]\s*['\"]relay['\"]", re.IGNORECASE),
    re.compile(r"\brelay(?:client|server|endpoint|url|transport|fallback|path)\b", re.IGNORECASE),
    re.compile(r"\brelay[-_](?:client|server|endpoint|url|transport|fallback|path)\b", re.IGNORECASE),
    re.compile(r"\brelay\s*(?:\(|\.|:|=)", re.IGNORECASE),
    re.compile(r"\bwss?://", re.IGNORECASE),
    re.compile(r"\bwebsocket(?:client|server)?\b", re.IGNORECASE),
    re.compile(r"\bserver[-_ ]mediated\b", re.IGNORECASE),
    re.compile(r"\bcontent[-_ ]forward(?:ing)?\b", re.IGNORECASE),
    re.compile(r"\bproxy[-_ ]message\b", re.IGNORECASE),
)


def _excluded(relative_path: str) -> bool:
    parts = Path(relative_path).parts
    if relative_path == SCANNER_RELATIVE_PATH:
        return True
    if any(part in EXCLUDED_PARTS for part in parts):
        return True
    if any(part in EXCLUDED_TEST_DIRS for part in parts[:-1]):
        return True
    return False


def is_scanned(path: Path, root: Path) -> bool:
    rel = path.relative_to(root).as_posix()
    if _excluded(rel):
        return False

    parts = Path(rel).parts
    under_scan_root = (
        rel.startswith(".github/workflows/")
        or (parts and parts[0] in SCAN_ROOTS)
    )
    root_manifest = len(parts) == 1 and path.name in CONFIG_NAMES
    dependency_manifest = path.name in CONFIG_NAMES or path.name.startswith(".env")
    return (
        (under_scan_root or root_manifest or dependency_manifest)
        and (path.name in CONFIG_NAMES or path.name.startswith(".env")
             or path.suffix.lower() in TEXT_SUFFIXES)
    )


def scan_file(path: Path, root: Path) -> list[str]:
    if not is_scanned(path, root):
        return []

    relative = path.relative_to(root).as_posix()
    if path.is_symlink():
        return [f"{relative}: symlinked policy-relevant file (fail closed)"]

    try:
        content = path.read_text(encoding="utf-8")
    except (UnicodeDecodeError, OSError):
        return [f"{relative}: unreadable policy-relevant file (fail closed)"]

    findings: list[str] = []
    for line_no, line in enumerate(content.splitlines(), start=1):
        for pattern in FORBIDDEN_PATTERNS:
            if pattern.search(line):
                findings.append(
                    f"{relative}:{line_no}: forbidden relay/content-transport indicator"
                )
                break
    return findings


def scan_repository(root: Path) -> list[str]:
    findings: list[str] = []
    for path in root.rglob("*"):
        if path.is_file() or path.is_symlink():
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

    print("PASS: no forbidden relay/content-transport indicators found in scanned runtime/config paths.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
