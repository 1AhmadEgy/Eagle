#!/usr/bin/env python3
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[2]
violations = []

# Verified Android module.
settings = ROOT / "settings.gradle.kts"
if settings.exists() and 'include(":app")' not in settings.read_text(encoding="utf-8"):
    violations.append("Android module contract: expected include(\":app\")")

# High-risk names in UI/application source. This is intentionally conservative.
ui_roots = [ROOT / "app" / "src/main/java/com/eagle/app", ROOT / "shared" / "src", ROOT / "androidApp"]
key_patterns = re.compile(r"(?i)\b(private[_ -]?key|secret[_ -]?key|secretkey|privatekey)\b")
for root in ui_roots:
    if not root.exists():
        continue
    for p in root.rglob("*"):
        if "security" in p.parts:
            continue
        if not p.is_file() or p.suffix not in {".kt", ".kts", ".java", ".swift"}:
            continue
        try:
            text = p.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        if key_patterns.search(text):
            violations.append(f"possible private-key handling in platform/application source: {p.relative_to(ROOT)}")

# Transport/mesh must not expose obvious plaintext application APIs.
mesh_roots = [ROOT / "mesh", ROOT / "transport", ROOT / "core" / "src" / "transport"]
plaintext_patterns = re.compile(r"(?i)\b(plaintext|messagebody|message_body|rawmessage|raw_message)\b")
for root in mesh_roots:
    if not root.exists():
        continue
    for p in root.rglob("*"):
        if not p.is_file() or p.suffix not in {".rs", ".kt", ".java", ".swift", ".kts"}:
            continue
        try:
            text = p.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        if plaintext_patterns.search(text):
            violations.append(f"possible plaintext exposure in transport/mesh source: {p.relative_to(ROOT)}")

gate = ROOT / "docs" / "03-architecture" / "ARCHITECTURE_RELEASE_GATE_2026-10-05.md"
if not gate.exists():
    violations.append("architecture release gate document is missing")

if violations:
    print("ARCHITECTURE GATE: FAIL")
    for item in violations:
        print(f"- {item}")
    sys.exit(1)

print("ARCHITECTURE GATE: PASS")
