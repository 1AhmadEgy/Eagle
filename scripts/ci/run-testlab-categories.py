#!/usr/bin/env python3
"""Deterministic Test Lab category runner for the current Eagle repository.

Only executable evidence is promoted to PASS.
Missing product-level coverage remains PENDING.
"""
from __future__ import annotations

import json
import shutil
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


def record(evidence: dict[str, object], category: str, status: str, **extra: object) -> None:
    payload = {"status": status}
    payload.update(extra)
    evidence["categories"][category] = payload


def main() -> int:
    OUT.mkdir(parents=True, exist_ok=True)
    evidence: dict[str, object] = {
        "schema_version": "1.1",
        "project": "Eagle",
        "timestamp_utc": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "categories": {},
        "gate": "BLOCKED",
    }

    rc, output = run([sys.executable, "scripts/ci/verify-security-policy.py"])
    (OUT / "security-policy.log").write_text(output, encoding="utf-8")
    record(
        evidence,
        "Security",
        "PASS" if rc == 0 else "FAIL",
        command="python3 scripts/ci/verify-security-policy.py",
        evidence=".ci/testlab/security-policy.log",
    )
    if rc != 0:
        evidence["verification"] = "FAIL"
        (OUT / "categories.json").write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf-8")
        return rc

    python_files = [
        p for p in ROOT.rglob("*.py")
        if ".git" not in p.parts and ".ci/testlab" not in str(p)
    ]
    static_status = "PASS"
    static_output: list[str] = []
    for path in python_files:
        rc, output = run([sys.executable, "-m", "py_compile", str(path.relative_to(ROOT))])
        static_output.append(output)
        if rc != 0:
            static_status = "FAIL"
            break
    (OUT / "static-analysis.log").write_text(
        "".join(static_output)
        if static_status == "FAIL"
        else f"Compiled {len(python_files)} Python files successfully.\n",
        encoding="utf-8",
    )
    record(
        evidence,
        "Static analysis",
        static_status,
        command="python3 -m py_compile <repository Python files>",
        evidence=".ci/testlab/static-analysis.log",
        files_checked=len(python_files),
    )
    if static_status == "FAIL":
        evidence["verification"] = "FAIL"
        (OUT / "categories.json").write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf-8")
        return 1

    manifests = [
        "package.json",
        "pyproject.toml",
        "requirements.txt",
        "requirements-dev.txt",
        "go.mod",
        "Cargo.toml",
    ]
    dependency_files = [name for name in manifests if (ROOT / name).is_file()]
    record(
        evidence,
        "Dependency checks",
        "PENDING" if dependency_files else "NOT_APPLICABLE",
        command="repository manifest inventory",
        evidence=".ci/testlab/categories.json",
        manifests=dependency_files,
        reason=(
            "Dependency manifests exist; ecosystem-specific vulnerability/SCA review "
            "remains a separate release gate."
            if dependency_files
            else "No supported application dependency manifest exists."
        ),
    )

    relay_rc, relay_output = run(
        [sys.executable, "scripts/ci/verify-relay-boundary.py"]
    )
    (OUT / "p2p-boundary.log").write_text(relay_output, encoding="utf-8")
    record(
        evidence,
        "P2P boundary",
        "PASS" if relay_rc == 0 else "FAIL",
        command="python3 scripts/ci/verify-relay-boundary.py",
        evidence=".ci/testlab/p2p-boundary.log",
    )
    if relay_rc != 0:
        evidence["verification"] = "FAIL"
        (OUT / "categories.json").write_text(
            json.dumps(evidence, indent=2) + "\n", encoding="utf-8"
        )
        return 1

    security_tests = sorted((ROOT / "tests" / "security").glob("test_*.py")) if (ROOT / "tests" / "security").is_dir() else []
    if security_tests:
        rc, output = run(
            [sys.executable, "-m", "unittest", "discover", "-s", "tests/security", "-p", "test_*.py"]
        )
        (OUT / "security-tests.log").write_text(output, encoding="utf-8")
        if rc == 0:
            # Security PASS now requires executable repository security tests
            # in addition to the static policy gate.
            record(
                evidence,
                "Security",
                "PASS",
                command="python3 -m unittest discover -s tests/security -p test_*.py",
                evidence=".ci/testlab/security-tests.log",
                tests=len(security_tests),
            )
        else:
            record(
                evidence,
                "Security",
                "FAIL",
                command="python3 -m unittest discover -s tests/security -p test_*.py",
                evidence=".ci/testlab/security-tests.log",
                tests=len(security_tests),
            )
            evidence["verification"] = "FAIL"
            (OUT / "categories.json").write_text(
                json.dumps(evidence, indent=2) + "\n", encoding="utf-8"
            )
            return 1

    rust_test_passed = False
    if (ROOT / "Cargo.toml").is_file() and shutil.which("cargo"):
        rc, output = run(["cargo", "test", "--workspace", "--locked"])
        (OUT / "rust-test.log").write_text(output, encoding="utf-8")
        rust_test_passed = rc == 0
        record(
            evidence,
            "Build",
            "PASS" if rust_test_passed else "FAIL",
            command="cargo test --workspace --locked",
            evidence=".ci/testlab/rust-test.log",
            scope="Rust workspace build + tests",
        )
        record(
            evidence,
            "Unit",
            "PASS" if rust_test_passed else "FAIL",
            command="cargo test --workspace --locked",
            evidence=".ci/testlab/rust-test.log",
            scope="Rust unit tests",
        )
        record(
            evidence,
            "Integration",
            "PASS" if rust_test_passed and (ROOT / "core" / "tests").exists() else "PENDING",
            command="cargo test --workspace --locked",
            evidence=".ci/testlab/rust-test.log",
            scope="Rust integration test targets",
        )
        record(
            evidence,
            "Protocol",
            "PASS"
            if rust_test_passed
            and (ROOT / "core" / "src" / "protocol.rs").exists()
            and (ROOT / "core" / "src" / "replay.rs").exists()
            else "PENDING",
            command="cargo test --workspace --locked",
            evidence=".ci/testlab/rust-test.log",
            scope="bounded protocol/replay/freshness tests",
        )
    else:
        record(evidence, "Build", "PENDING", reason="Rust workspace/toolchain not available in this execution environment.")
        record(evidence, "Unit", "PENDING", reason="Rust toolchain not available in this execution environment.")
        record(evidence, "Integration", "PENDING", reason="Executable integration environment not available.")
        record(evidence, "Protocol", "PENDING", reason="Executable protocol test environment not available.")

    # Full application-level categories remain gated until their independent suites exist.
    record(
        evidence,
        "Cryptography",
        "PENDING",
        reason="No production cryptographic provider/interoperability suite is authorized by the current ADR gate.",
    )
    record(
        evidence,
        "Regression",
        "PENDING",
        reason="Full cross-platform/device lifecycle regression evidence is not yet available.",
    )
    record(
        evidence,
        "Fuzz/property",
        "PENDING",
        reason="Dedicated fuzz/property execution suite is not yet evidenced.",
    )

    evidence["verification"] = "PASS" if rust_test_passed or not (ROOT / "Cargo.toml").is_file() else "FAIL"
    evidence["gate"] = "BLOCKED"
    evidence["gate_reason"] = (
        "Executable Rust evidence is collected where available, but release-critical "
        "cryptography, interoperability, cross-platform, regression, fuzz/property and human-review gates remain unresolved."
    )

    (OUT / "categories.json").write_text(
        json.dumps(evidence, indent=2) + "\n",
        encoding="utf-8",
    )
    return 0 if evidence["verification"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
