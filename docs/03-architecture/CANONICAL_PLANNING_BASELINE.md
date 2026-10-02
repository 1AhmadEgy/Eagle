# Canonical Planning Baseline v1

**Project:** Eagle  
**Version:** 1.0-proposed  
**Date:** 2026-10-02  
**Status:** Proposed — not yet canonical/approved  
**Governing ADR:** ADR-0007  
**Governing issue:** ARCH-001

## Authority

This file is the proposed planning reference for the architecture review. It becomes the canonical planning baseline only after ARCH-001 and ADR-0007 are explicitly approved.

Until then, the repository's existing approved records remain authoritative where they conflict with this proposal.

## Modules

1. core — contracts / models / errors / events
2. identity — device identity + key lifecycle
3. crypto — sessions + encryption/decryption
4. protocol — envelope + framing + serialization + versioning
5. storage — persistence / repository
6. mesh — discovery + transport + routing + forwarding
7. ui — presentation
8. integration — dependency wiring only
9. security — verification / audit / gates
10. observability — sanitized telemetry
11. documentation — architectural / project records

## Dependency rules

```
core
├── identity
├── crypto
├── storage
├── protocol
├── mesh
└── ui

identity → crypto
crypto → protocol
protocol → mesh

integration → implementations for wiring
security → read/test/audit
observability → sanitized contracts
```

The effective module rules are:

- storage → core only
- crypto → core + identity contracts
- protocol → core + crypto contracts
- mesh → core + protocol contracts
- no crypto ↔ mesh circular dependency
- no storage → protocol dependency

## Security flow

### Send

`plaintext → crypto → encrypted envelope → protocol → mesh → transport`

### Receive

`transport → mesh → protocol → crypto → plaintext → application/UI`

### Forbidden exposure

- mesh: no plaintext
- protocol: no application plaintext
- storage: no private keys
- UI: no private keys / crypto implementation
- logs: no plaintext / secrets

## ADR separation

An ADR records a decision. An issue implements or verifies that decision.

Example:

`ADR-0010 → PROTO-003`

## Approval gate

This document must not be described as canonical until:

- ARCH-001 is reviewed and accepted.
- ADR-0007 is accepted.
- ADR-0001 → ADR-0006 compatibility is verified.
- Security and architecture reviews are recorded.
- Repository evidence is linked.
