# MESH-006 — Opaque Transport Contract
Status: SPECIFIED / IMPLEMENTATION-GATED

## Contract
MeshTransport operates on opaque/encrypted frames.

Required conceptual operations:
- discover
- gather_candidates
- connect
- send
- receive
- close
- state

## Invariants
- No application plaintext crosses the Mesh boundary.
- No private-key access.
- Bounded frame size.
- Typed transport errors.
- Unknown protocol versions fail safely.

## Acceptance
Contract tests prove opaque-frame round trip and plaintext rejection.
