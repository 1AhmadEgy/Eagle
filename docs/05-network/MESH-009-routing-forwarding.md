# MESH-009 — Routing and Forwarding
Status: SPECIFIED / IMPLEMENTATION-GATED

## Routing
Routes use bounded peer/transport metadata.

## Required protections
- hop limit
- loop detection
- duplicate suppression where required
- bounded route length
- deterministic route selection

## Boundary
Forwarding never needs application plaintext or private keys.

## Acceptance
Loop, excessive-hop, duplicate, and unreachable-route tests are deterministic.
