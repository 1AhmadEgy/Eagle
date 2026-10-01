#!/usr/bin/env python3
"""Validate Eagle's generated provenance evidence without network access."""

from __future__ import annotations

import json
import re
import sys
from datetime import datetime
from pathlib import Path

SCHEMA = Path("docs/provenance/schema.json")
EVIDENCE = Path(".ci/documentation/provenance.json")


def fail(message: str) -> None:
    print(f"PROVENANCE_VALIDATION: FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


if not SCHEMA.is_file():
    fail("missing docs/provenance/schema.json")
if not EVIDENCE.is_file():
    fail("missing .ci/documentation/provenance.json")

try:
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    evidence = json.loads(EVIDENCE.read_text(encoding="utf-8"))
except (OSError, json.JSONDecodeError) as exc:
    fail(f"invalid JSON: {exc}")

if schema.get("title") != "Eagle Provenance Evidence":
    fail("unexpected provenance schema")

required = {
    "schema_version", "project", "commit", "ref", "event",
    "actor", "timestamp_utc", "changed_files", "ownership", "test_lab"
}
missing = required - evidence.keys()
if missing:
    fail(f"missing required fields: {sorted(missing)}")

if evidence["schema_version"] != "1.0":
    fail("unsupported schema_version")
if evidence["project"] != "Eagle":
    fail("project must be Eagle")
if not re.fullmatch(r"[0-9a-f]{40}", evidence["commit"]):
    fail("commit must be a 40-character lowercase SHA-1")
try:
    datetime.fromisoformat(evidence["timestamp_utc"].replace("Z", "+00:00"))
except ValueError:
    fail("timestamp_utc is not RFC3339-compatible")

ownership = evidence["ownership"]
if ownership != {
    "intent": "PRIVATE / ALL RIGHTS RESERVED",
    "copyright_holder": "UNCONFIRMED",
    "open_source_license_grant": "NONE",
}:
    fail("ownership record violates the private-ownership contract")

test_lab = evidence["test_lab"]
if test_lab.get("status") not in {"PASS", "FAIL", "PENDING", "NOT_APPLICABLE", "UNCONFIRMED"}:
    fail("invalid Test Lab status")
if not isinstance(test_lab.get("evidence_source"), str) or not test_lab["evidence_source"]:
    fail("missing Test Lab evidence source")

if not isinstance(evidence["changed_files"], list):
    fail("changed_files must be an array")

print("PROVENANCE_VALIDATION: PASS")
