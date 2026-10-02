# ARCH-003 — Dependency Rules

**Status:** Proposed

Allowed dependency direction:
core→none implementation-specific; identity→core; crypto→core+identity contracts; protocol→core+crypto contracts; storage→core; mesh→core+protocol contracts; ui→core+application services; integration→wiring; security→read/test/audit; observability→sanitized contracts.

Denied: UI→Keystore, UI→crypto internals, UI→database internals, Mesh→plaintext, Mesh→database internals, Storage→UI, Identity→UI, Crypto↔Mesh cycles, Storage→Protocol.

Exact enforcement technology remains governed by ADR-0013.