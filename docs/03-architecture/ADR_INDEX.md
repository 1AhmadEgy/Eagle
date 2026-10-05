# Eagle ADR Index

## Existing sequence
ADR-0001 → ADR-0006 — existing project decisions; compatibility remains required where relevant.

## Proposed / review sequence
- ADR-0007 — Architecture & Task Baseline
- ADR-0008 — Cryptographic Protocol
- ADR-0009 — Key Management
- ADR-0010 — Serialization
- ADR-0011 — Local Storage
- ADR-0012 — Direct P2P Transport Architecture
- ADR-0013 — Architecture Enforcement
- ADR-0014 — Observability
- ADR-0015 — Rust FFI Boundary + Kotlin Multiplatform Application Layer
- ADR-0016 — Storage / Recovery Contract

## Rule
ADR = decision. Issue/PR = implementation, verification, migration, or follow-up.

A Proposed ADR is not an approved technical choice. A later implementation record cannot silently promote it. Conflicts must be reconciled explicitly.

## Architecture consequence
The implementation baseline may establish contracts and fail-closed scaffolding while technical choices remain pending, but production integration of a gated technology requires the corresponding ADR approval and executable evidence.
