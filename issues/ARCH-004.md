# ARCH-004 — Architecture Tests

**Status:** Proposed

Automated tests must cover dependency isolation, cycle detection, plaintext boundaries, private-key exposure, contract compliance, secrets, and logging safety.

Acceptance: tests run in CI; intentional violations fail; baseline boundaries are covered; evidence is retained.

Requires: ARCH-001, ADR-0007, ARCH-002, ARCH-003.