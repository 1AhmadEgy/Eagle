#!/usr/bin/env python3
"""Deterministic Test Lab category runner for the current Eagle repository.

The runner reports only evidence that can be demonstrated from repository state.
It never upgrades missing product tests to PASS.
"""
from __future__ import annotations

import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / ".ci" / "testlab"


def run(command: list[str]) -> tuple[int, str]:
    proc = subprocess.run(
        command,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    return proc.returncode, proc.stdout


def main() -> int:
    OUT.mkdir(parents=True, exist_ok=True)
    evidence: dict[str, object] = {
        "schema_version": "1.0",
        "project": "Eagle",
        "timestamp_utc": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "categories": {},
        "gate": "BLOCKED",
    }

    # Security: this is a concrete, repository-local control check.
    rc, output = run([sys.executable, "scripts/ci/verify-security-policy.py"])
    (OUT / "security-policy.log").write_text(output, encoding="utf-8")
    evidence["categories"]["Security"] = {
        "status": "PASS" if rc == 0 else "FAIL",
        "command": "python3 scripts/ci/verify-security-policy.py",
        "evidence": ".ci/testlab/security-policy.log",
    }
    if rc != 0:
        evidence["verification"] = "FAIL"
        (OUT / "categories.json").write_text(
            json.dumps(evidence, indent=2) + "\n", encoding="utf-8"
        )
        return rc

    # Static analysis: compile every repository Python source/check script.
    python_files = [
        p for p in ROOT.rglob("*.py")
        if ".git" not in p.parts and ".ci/testlab" not in str(p)
    ]
    static_status = "PASS"
    static_command = "python3 -m py_compile <repository Python files>"
    for path in python_files:
        rc, output = run([sys.executable, "-m", "py_compile", str(path.relative_to(ROOT))])
        if rc != 0:
            static_status = "FAIL"
            (OUT / "static-analysis.log").write_text(output, encoding="utf-8")
            break
    else:
        (OUT / "static-analysis.log").write_text(
            f"Compiled {len(python_files)} Python files successfully.\n",
            encoding="utf-8",
        )
    evidence["categories"]["Static analysis"] = {
        "status": static_status,
        "command": static_command,
        "evidence": ".ci/testlab/static-analysis.log",
        "files_checked": len(python_files),
    }
    if static_status == "FAIL":
        evidence["verification"] = "FAIL"
        (OUT / "categories.json").write_text(
            json.dumps(evidence, indent=2) + "\n", encoding="utf-8"
        )
        return 1

    manifests = [
        "package.json", "pyproject.toml", "requirements.txt", "requirements-dev.txt",
        "go.mod", "Cargo.toml"
    ]
    dependency_files = [name for name in manifests if (ROOT / name).is_file()]
    evidence["categories"]["Dependency checks"] = {
        "status": "PENDING" if dependency_files else "NOT_APPLICABLE",
        "command": "repository manifest inventory",
        "evidence": ".ci/testlab/categories.json",
        "manifests": dependency_files,
        "reason": (
            "Dependency manifests exist; ecosystem-specific dependency audit is still required."
            if dependency_files
            else "No supported application dependency manifest exists in the current repository."
        ),
    }

    # Product categories cannot be inferred from infrastructure-only checks.
    for category in (
        "Build", "Unit", "Integration", "Cryptography", "Protocol",
        "Regression", "Fuzz/property",
    ):
        evidence["categories"][category] = {
            "status": "PENDING",
            "reason": "No authoritative product capability/test suite exists to execute this category yet.",
        }

    evidence["verification"] = "PASS"
    evidence["gate"] = "BLOCKED"
    evidence["gate_reason"] = (
        "Security and static-analysis evidence are concrete, but product-specific "
        "categories remain PENDING; no overall release eligibility is inferred."
    )
    (OUT / "categories.json").write_text(
        json.dumps(evidence, indent=2) + "\n", encoding="utf-8"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
