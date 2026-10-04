# MESH-001 — Peer Identity Reference
Status: SPECIFIED / IMPLEMENTATION-GATED

## Purpose
Provide PrivateMesh with an opaque peer reference without transferring identity secrets into the networking layer.

## Contract
PeerId is stable, opaque, comparable, serializable, and non-secret.

## Rules
- Mesh may store and compare PeerId.
- Mesh never receives private keys.
- Identity verification remains outside Mesh.
- PeerId must not embed credentials, tokens, or plaintext user data.

## Acceptance
- PeerId equality is deterministic.
- Invalid encodings are rejected.
- No private-key API exists in the Mesh contract.
- Identity mapping is covered by a boundary test.
