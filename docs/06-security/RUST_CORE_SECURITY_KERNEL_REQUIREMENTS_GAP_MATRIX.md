# Rust Core / Security Kernel — Requirements & Gap Matrix

**Date:** 2026-10-05  
**Scope:** Rust Core / Security Kernel only

| Requirement | Source / authority | Current evidence | Status | Gap / next dependency |
|---|---|---|---|---|
| Layer C is Rust Security Core | `docs/03-architecture/PLATFORMS.md` | current crate under `core/` | PASS | none for boundary |
| Security-sensitive state is private | Security Kernel contract | private fields + guarded methods | PASS | none |
| Public caller cannot self-promote trust | Security Kernel contract / Identity threat model | verifier seam is crate-test-only | PASS | real verifier comes from approved protocol |
| Unknown/pending trust cannot authorize | threat model | `authorize()` guards | PASS | none for deterministic slice |
| Session establishment requires trusted authenticated state | Security Kernel contract | state preconditions | PASS | real authentication integration pending |
| Rekey requires established trusted session | Security Kernel contract | guarded transition + tests | PASS | cryptographic rekey semantics pending |
| Revocation is terminal for context | threat model | revocation closes session | PASS | distributed/offline reconciliation is external |
| Replaced device fails closed | threat model | terminal `Replaced` state | PASS | membership protocol integration pending |
| Protocol downgrade rejected | ADR-0008 proposal + security invariants | bounded monotonic negotiation | PASS | protocol ADR still pending |
| Envelope/resource bounds enforced | Security Kernel contract | constructor and payload limits | PASS | canonical wire serialization pending |
| No unsafe Rust | project security baseline | crate root `forbid(unsafe_code)` | PASS | none for current crate |
| No secrets in source | security baseline | no secret material introduced | PASS | continuous CI scan |
| No custom cryptography | ADR-0008 gate | no crypto implementation added | PASS | approved cryptographic stack required later |
| Cross-platform binding cannot bypass kernel | PLATFORMS.md / FFI rule | documented interface rule | PASS | UniFFI ABI/security tests pending |
| Deterministic unit/negative tests | DoD / Test Matrix | tests committed | PASS | current-head CI verification pending |
| Independent CI verification | project gate | previous run passed; latest run queued | PENDING | await current-head workflow result |
| Independent security review | DoD | no review evidence yet | PENDING | required before security-sensitive merge |
| Production release of specialization | release gate | PR #65 remains draft | BLOCKED | human review + current CI + merge gate |

## Gap closure rule

A row marked **PENDING** or **BLOCKED** is not promoted to PASS by documentation. The gap is closed only by new repository evidence.

## Security posture

The deterministic kernel boundary is complete as an implementation slice. It is deliberately not represented as an E2E cryptographic implementation.
