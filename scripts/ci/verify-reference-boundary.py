#!/usr/bin/env python3
"""Gate 5.7: reference implementations must not enter production dependency graphs."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


def main() -> int:
    root = Path(__file__).resolve().parents[2]
    proc = subprocess.run(
        ["cargo", "metadata", "--format-version", "1"],
        cwd=root,
        check=False,
        text=True,
        capture_output=True,
    )
    if proc.returncode != 0:
        print("Gate 5.7 FAIL: cargo metadata failed")
        print(proc.stderr)
        return proc.returncode

    metadata = json.loads(proc.stdout)
    packages = {pkg["id"]: pkg for pkg in metadata["packages"]}
    workspace_ids = set(metadata["workspace_members"])

    reference_ids = {
        pkg_id
        for pkg_id, pkg in packages.items()
        if "reference" in pkg["name"].lower()
        or "mock" in pkg["name"].lower()
        or "test" in pkg["name"].lower()
    }

    if not reference_ids:
        print("Gate 5.7 SKIP: reference crate not yet present")
        return 0

    production_ids = workspace_ids - reference_ids
    violations: list[str] = []

    resolve = metadata.get("resolve") or {}
    nodes = {node["id"]: node for node in resolve.get("nodes", [])}

    for root_id in sorted(production_ids):
        root_pkg = packages[root_id]
        node = nodes.get(root_id, {})
        for dep in node.get("deps", []):
            if dep["pkg"] not in reference_ids:
                continue
            kinds = {kind.get("kind") for kind in dep.get("dep_kinds", [])}
            if "normal" in kinds or "build" in kinds:
                target = packages[dep["pkg"]]["name"]
                violations.append(
                    f"{root_pkg['name']} -> {target} ({','.join(sorted(k for k in kinds if k))})"
                )

    if violations:
        print("Gate 5.7 FAIL: production dependency graph reaches reference implementations:")
        for violation in violations:
            print(f"  - {violation}")
        return 1

    print("Gate 5.7 PASS: no production dependency edge reaches reference/test/mock crates.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
