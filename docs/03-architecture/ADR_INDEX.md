# Eagle ADR Index

## Existing sequence
ADR-0001 → ADR-0006 — existing project decisions referenced by the planning work; compatibility must be verified before ADR-0007 is accepted.

## Proposed sequence
- ADR-0007 — Architecture & Task Baseline
- ADR-0008 — Cryptographic Protocol
- ADR-0009 — Key Management
- ADR-0010 — Serialization
- ADR-0011 — Local Storage
- ADR-0012 — Transport Architecture
- ADR-0013 — Architecture Enforcement
- ADR-0014 — Observability
- ADR-0015 — Rust FFI Boundary + Kotlin Multiplatform Application Layer

## Rule
ADR = decision. Issue = implementation / verification / migration / follow-up.

An ADR marked Proposed is not an approved technical choice. Conflicts with earlier records must be reconciled explicitly.

## Integration rule
Foundation implementation is integrated through reviewable A→E pull requests. A proposed ADR may describe a boundary or contract without authorizing production cryptography, key management, serialization, or concrete transport implementation.
