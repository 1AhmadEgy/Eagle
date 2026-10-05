from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from importlib.util import module_from_spec, spec_from_file_location

MODULE_PATH = Path(__file__).resolve().parents[2] / "scripts" / "ci" / "verify-relay-boundary.py"
SPEC = spec_from_file_location("relay_boundary", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
MODULE = module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class RelayInfrastructurePolicyTests(unittest.TestCase):
    def test_clean_runtime_source_passes(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "app").mkdir()
            source = root / "app" / "Network.kt"
            source.write_text("fun connect(peer: String) = peer", encoding="utf-8")
            self.assertEqual(MODULE.scan_repository(root), [])

    def test_relay_indicator_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "app").mkdir()
            source = root / "app" / "Network.kt"
            source.write_text("fun send() = relay(message)", encoding="utf-8")
            findings = MODULE.scan_repository(root)
            self.assertEqual(len(findings), 1)
            self.assertIn("forbidden relay", findings[0])

    def test_turn_and_websocket_endpoints_are_blocked(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "app").mkdir()
            source = root / "app" / "Transport.kt"
            source.write_text(
                """val a = "turns://example.invalid"
val b = "wss://example.invalid"""",
                encoding="utf-8",
            )
            findings = MODULE.scan_repository(root)
            self.assertEqual(len(findings), 2)

    def test_documentation_is_not_runtime_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            docs = root / "docs"
            docs.mkdir()
            source = docs / "transport.md"
            source.write_text("Relay is prohibited.", encoding="utf-8")
            self.assertEqual(MODULE.scan_repository(root), [])


if __name__ == "__main__":
    unittest.main()
