#!/usr/bin/env python3
"""Deterministic Eagle repository/documentation consistency gate."""
from __future__ import annotations
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
STATE_PATH = ROOT / "docs/03-architecture/REPOSITORY_STATE_V1.json"
PLATFORMS_PATH = ROOT / "docs/03-architecture/PLATFORMS.md"
ADR_INDEX_PATH = ROOT / "docs/03-architecture/ADR_INDEX.md"
WORKLIST_PATH = ROOT / "docs/03-architecture/adr/ADR-0011-0014-WORKLIST.md"
ROOT_BUILD_PATH = ROOT / "build.gradle.kts"
APP_BUILD_PATH = ROOT / "app/build.gradle.kts"

errors: list[str] = []
warnings: list[str] = []

def fail(message: str) -> None:
    errors.append(message)

def require_file(path: Path, description: str) -> None:
    if not path.is_file():
        fail(description + " missing: " + str(path.relative_to(ROOT)))

def main() -> int:
    for path, description in (
        (STATE_PATH, "Repository state registry"),
        (PLATFORMS_PATH, "Platform strategy"),
        (ADR_INDEX_PATH, "ADR index"),
        (WORKLIST_PATH, "ADR worklist"),
        (ROOT_BUILD_PATH, "Root Android build file"),
        (APP_BUILD_PATH, "Android app build file"),
    ):
        require_file(path, description)

    if errors:
        for item in errors:
            print("ERROR: " + item)
        return 1

    try:
        state = json.loads(STATE_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print("ERROR: cannot read repository state registry: " + str(exc))
        return 1

    platforms = PLATFORMS_PATH.read_text(encoding="utf-8")
    adr_index = ADR_INDEX_PATH.read_text(encoding="utf-8")
    worklist = WORKLIST_PATH.read_text(encoding="utf-8")
    root_build = ROOT_BUILD_PATH.read_text(encoding="utf-8")
    app_build = APP_BUILD_PATH.read_text(encoding="utf-8")

    actual_module = state["actual_android_module"]
    if not (ROOT / actual_module).is_dir():
        fail("actual Android module path does not exist: " + actual_module + "/")

    if "androidApp/" in platforms:
        fail("PLATFORMS.md still refers to stale androidApp/; actual module is app/")

    if actual_module + "/" not in platforms:
        fail("PLATFORMS.md does not document the actual Android module path app/")

    for name, rel in state["planned_platform_paths"].items():
        if (ROOT / rel).exists():
            warnings.append("planned platform path now exists (" + name + ": " + rel + "); review repository state")

    for adr_id, record in state["adr_state"].items():
        representation = ROOT / record["representation"]
        if not representation.is_file():
            fail(adr_id + " representation missing: " + str(representation.relative_to(ROOT)))
        if record["status"] == "proposed":
            dedicated = ROOT / ("docs/03-architecture/adr/" + adr_id + ".md")
            if not dedicated.is_file():
                fail(adr_id + " is marked proposed but its dedicated file is missing")

    for adr_id in ("ADR-0011", "ADR-0012", "ADR-0013", "ADR-0014"):
        if adr_id not in worklist:
            fail(adr_id + " missing from ADR worklist")

    expected = state["android_build"]
    checks = {
        "agp": 'id("com.android.application") version "' + re.escape(expected["agp"]) + '"',
        "compileSdk": "compileSdk = " + str(expected["compile_sdk"]),
        "targetSdk": "targetSdk = " + str(expected["target_sdk"]),
        "minSdk": "minSdk = " + str(expected["min_sdk"]),
        "junit": re.escape(expected["unit_test_framework"]),
    }
    for name, pattern in checks.items():
        haystack = root_build if name == "agp" else app_build
        if re.search(pattern, haystack) is None:
            fail("Android build baseline drift detected for " + name)

    for dep in ("libsignal", "openmls", "libp2p", "sqlcipher", "androidx.room"):
        if dep.lower() in app_build.lower():
            fail("current baseline unexpectedly contains external component dependency: " + dep)

    for key in (
        "real_cryptography",
        "device_identity",
        "approved_e2e_protocol",
        "rust_security_core",
        "kmp_shared_layer",
        "mesh_engine",
        "persistent_messaging_storage",
    ):
        if state["implementation_baseline"].get(key) is not False:
            fail("baseline maturity claim changed without state/evidence review: " + key)

    for adr_id, record in state["adr_state"].items():
        if record["status"].startswith("proposed") and adr_id not in adr_index:
            fail("ADR index/state mismatch for " + adr_id)

    active_roots = (
        ROOT / "docs/01-research",
        ROOT / "docs/03-architecture",
        ROOT / "scripts/ci",
    )
    for base in active_roots:
        if not base.exists():
            continue
        for path in base.rglob("*"):
            if not path.is_file() or path.suffix.lower() not in {".md", ".json", ".py", ".sh"}:
                continue
            if path in {STATE_PATH, path.parent / "REPOSITORY_CONSISTENCY_V1.md", path.parent / "verify-repository-consistency.py"}:
                continue
            try:
                content = path.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                continue
            if "androidApp/" in content:
                fail("stale androidApp/ token remains in active file: " + str(path.relative_to(ROOT)))

    if errors:
        print("Eagle repository consistency: FAIL")
        for item in errors:
            print("ERROR: " + item)
        for item in warnings:
            print("WARNING: " + item)
        return 1

    print("Eagle repository consistency: PASS")
    print("Android module: " + actual_module + "/")
    print("ADR-0011..0014: explicit pending worklist")
    print("Security implementation baseline: not yet integrated")
    for item in warnings:
        print("WARNING: " + item)
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
