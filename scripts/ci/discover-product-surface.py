#!/usr/bin/env python3
"""Deterministic product-surface discovery for Eagle.

This scanner inventories repository evidence without inventing product behavior.
It is intentionally dependency-free and safe to run in CI.
"""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / ".ci" / "testlab" / "product-surface.json"

IGNORED_DIRS = {".git", ".ci", "__pycache__"}
MANIFESTS = {
    "package.json", "pyproject.toml", "requirements.txt", "requirements-dev.txt",
    "go.mod", "Cargo.toml", "pom.xml", "build.gradle", "build.gradle.kts",
    "composer.json", "Gemfile", "mix.exs", "Package.swift",
}
TEST_DIR_NAMES = {"test", "tests", "spec", "__tests__"}
SOURCE_DIR_NAMES = {"src", "app", "lib", "server", "client", "backend", "frontend"}
ENTRYPOINT_NAMES = {
    "main.py", "app.py", "server.py", "main.go", "main.rs", "main.ts", "main.js",
    "index.ts", "index.js", "manage.py", "Program.cs", "Main.java",
}


def files() -> list[Path]:
    return [
        p for p in ROOT.rglob("*")
        if p.is_file() and not (set(p.parts) & IGNORED_DIRS)
    ]


def main() -> int:
    all_files = files()
    manifests = sorted(
        str(p.relative_to(ROOT))
        for p in all_files
        if p.name in MANIFESTS
    )
    source_dirs = sorted(
        str(p.relative_to(ROOT))
        for p in ROOT.rglob("*")
        if p.is_dir()
        and p.name in SOURCE_DIR_NAMES
        and not (set(p.parts) & IGNORED_DIRS)
    )
    test_dirs = sorted(
        str(p.relative_to(ROOT))
        for p in ROOT.rglob("*")
        if p.is_dir()
        and p.name in TEST_DIR_NAMES
        and not (set(p.parts) & IGNORED_DIRS)
    )
    entrypoints = sorted(
        str(p.relative_to(ROOT))
        for p in all_files
        if p.name in ENTRYPOINT_NAMES
    )
    implementation_files = [
        p for p in all_files
        if p.suffix.lower() in {
            ".py", ".js", ".jsx", ".ts", ".tsx", ".go", ".rs", ".java",
            ".kt", ".cs", ".cpp", ".cc", ".c", ".swift", ".rb", ".php",
        }
    ]

    product_surface = "DETECTED" if (manifests or source_dirs or implementation_files) else "NOT_DETECTED"
    test_surface = "DETECTED" if test_dirs else "NOT_DETECTED"

    evidence = {
        "schema_version": "1.0",
        "project": "Eagle",
        "product_surface": product_surface,
        "test_surface": test_surface,
        "repository_file_count": len(all_files),
        "implementation_file_count": len(implementation_files),
        "manifests": manifests,
        "source_directories": source_dirs,
        "test_directories": test_dirs,
        "entrypoints": entrypoints,
        "interpretation": (
            "Repository currently contains infrastructure/documentation evidence but no authoritative "
            "product capability surface from which product behavior or requirements can safely be inferred."
            if product_surface == "NOT_DETECTED"
            else
            "Product-related files are present. Requirements and test categories must be derived from their "
            "actual behavior before any category is promoted to PASS."
        ),
        "next_gate": (
            "REQUIREMENTS_AND_ARCHITECTURE_DISCOVERY"
            if product_surface == "NOT_DETECTED"
            else "DERIVE_REQUIREMENTS_AND_TEST_MATRIX"
        ),
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(evidence, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
