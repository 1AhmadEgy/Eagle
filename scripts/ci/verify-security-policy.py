#!/usr/bin/env python3
"""Static security-policy checks for Eagle CI configuration.

This verifier intentionally uses only Python's standard library so it can run
before project dependencies are installed.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW_DIR = ROOT / ".github" / "workflows"

REQUIRED_WORKFLOWS = {
    "ci.yml",
    "ai-fix-ci.yml",
    "documentation-agent.yml",
}
SHA_ACTION_RE = re.compile(
    r"^\s*(?:-\s*)?uses:\s*[^@\s]+@(?P<ref>[0-9a-f]{40})(?:\s+#.*)?$"
)

FORBIDDEN_REPAIR_PATHS = (
    ".github/workflows/**",
    ".env*",
    "**/*secret*",
    "**/*credential*",
)

def fail(message: str) -> None:
    print(f"SECURITY POLICY FAIL: {message}")
    raise SystemExit(1)

def check_required_workflows() -> None:
    missing = sorted(
        name for name in REQUIRED_WORKFLOWS
        if not (WORKFLOW_DIR / name).is_file()
    )
    if missing:
        fail(f"missing required workflows: {', '.join(missing)}")

def check_action_pinning() -> None:
    failures = []
    for path in sorted(WORKFLOW_DIR.glob("*.yml")):
        lines = path.read_text(encoding="utf-8").splitlines()
        for lineno, line in enumerate(lines, 1):
            if "uses:" not in line:
                continue
            if not SHA_ACTION_RE.match(line):
                failures.append(f"{path.relative_to(ROOT)}:{lineno}: {line.strip()}")
    if failures:
        fail("all workflow actions must use full 40-character commit SHAs:\n" + "\n".join(failures))

def check_permissions() -> None:
    for name in REQUIRED_WORKFLOWS:
        text = (WORKFLOW_DIR / name).read_text(encoding="utf-8")
        if not re.search(r"(?m)^permissions:\s*$", text):
            fail(f"{name}: top-level permissions block is required")

def check_repair_boundary() -> None:
    text = (WORKFLOW_DIR / "ai-fix-ci.yml").read_text(encoding="utf-8")
    required_fragments = (
        "github.event.workflow_run.event == 'push'",
        "github.event.workflow_run.head_branch == 'implementation/v1-foundation'",
        "Do not modify .github/workflows/*",
        "Do not merge or push to main.",
    )
    for fragment in required_fragments:
        if fragment not in text:
            fail(f"ai-fix-ci.yml: missing repair boundary: {fragment}")

    agent = (ROOT / ".opencode" / "agents" / "repair-agent.md").read_text(encoding="utf-8")
    for path in FORBIDDEN_REPAIR_PATHS:
        if path not in agent:
            fail(f"repair-agent.md: missing deny rule for {path}")
    if 'resource: "git push *"\n    effect: deny' not in agent:
        fail("repair-agent.md: git push must remain denied")

def main() -> int:
    check_required_workflows()
    check_action_pinning()
    check_permissions()
    check_repair_boundary()
    print("SECURITY POLICY PASS: workflow supply-chain and AI-repair boundaries verified.")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
