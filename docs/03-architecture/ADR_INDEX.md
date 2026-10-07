# Eagle ADR Index

## Authority and lifecycle

ADR status is authoritative only when recorded here and supported by repository evidence.

Allowed lifecycle states:

```text
PROPOSED → ACCEPTED → IMPLEMENTED → VERIFIED → RELEASED
             │            │            │
             └────────────┴────────────┴→ BLOCKED / SUPERSEDED / REJECTED
```

These states are not interchangeable:

- `PROPOSED != ACCEPTED`
- `ACCEPTED != IMPLEMENTED`
- `IMPLEMENTED != VERIFIED`
- `VERIFIED != RELEASED`

A PR, branch, archive, or conversation cannot promote an ADR state by itself.

## Existing sequence

ADR-0001 → ADR-0006 — existing project decisions; compatibility must be preserved.

## Proposed sequence

- ADR-0007 — Architecture & Task Baseline
- ADR-0008 — Cryptographic Protocol
- ADR-0009 — Key Management
- ADR-0010 — Serialization
- ADR-0011 — Local Storage
- ADR-0012 — Transport Architecture
- ADR-0013 — Architecture Enforcement
- ADR-0014 — Observability

## Reconciliation rules

1. No duplicate ADR numbering.
2. Existing ADR numbers are not renumbered during G0-A.
3. Proposed ADRs have no implementation authority.
4. Accepted ADRs must identify their implementation and verification evidence.
5. Material contradictions between an ADR and current `main` are recorded as gaps; they are not silently resolved by changing status.
6. Protocol, cryptography, key-management, and serialization decisions remain blocked until their required reviews and evidence are complete.

## Required index fields

`ADR ID`, `Title`, `Status`, `Authority`, `Owner`, `Deciders`, `Related requirements`, `Implementation reference`, `Related tests`, `Evidence`, `Supersedes`, `Superseded by`, `Last reviewed`.

## Rule

ADR = decision. Issue/PR = implementation, verification, migration, or follow-up. A candidate implementation does not become canonical merely because it exists in a PR.
