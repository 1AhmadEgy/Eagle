# MESH-003 — Peer Discovery
Status: SPECIFIED / IMPLEMENTATION-GATED

## Responsibilities
- Discover peer reachability metadata.
- Track freshness and expiry.
- Reject malformed/poisoned discovery records.
- Produce sanitized discovery events.

## Boundary
Discovery learns connectivity metadata only. It does not decrypt application data.

## Acceptance
Tests cover stale records, duplicate records, malformed records, poisoning indicators, and discovery timeout.
