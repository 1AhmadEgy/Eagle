# Rust Core / Security Kernel — Requirements & Gap Matrix

**Date:** 2026-10-05  
**Scope:** Rust Core / Security Kernel only

| Requirement | Source / authority | Current evidence | Status | Gap / next dependency |
|---|---|---|---|---|
| Layer C is Rust Security Core | `docs/03-architecture/PLATFORMS.md` | current crate under `core/` | PASS | none for boundary |
| Security-sensitive state is private | Security Kernel contract | private fields + guarded methods | PASS | none |
| Authority-bearing values are not implicitly duplicated | Security research / security invariant | `SecurityContext`, `Device`, `Session` no longer implement `Copy`/`Clone` | PASS | retain regression coverage |
| Public caller cannot self-promote trust | Security Kernel contract / Identity threat model | verifier seam is crate-test-only | PASS | real verifier comes from approved protocol |
| Unknown/pending trust cannot authorize | threat model | `authorize()` guards | PASS | none for deterministic slice |
| Session establishment requires trusted authenticated state | Security Kernel contract | state preconditions | PASS | real authentication integration pending |
| Device and session trust share one canonical revocation authority | threat model / research | two state holders currently exist | PENDING | unify identity/session revocation semantics |
| Rekey requires established trusted session | Security Kernel contract | guarded transition + tests | PASS | cryptographic rekey semantics pending |
| Revocation is terminal for context | threat model | revocation closes session | PASS | distributed/offline reconciliation is external |
| Replaced device fails closed | threat model | terminal `Replaced` state | PASS | membership protocol integration pending |
| Protocol downgrade rejected | ADR-0008 proposal + security invariants | bounded monotonic negotiation | PASS | protocol ADR still pending |
| Envelope/resource bounds enforced | Security Kernel contract | constructor and payload limits | PASS | canonical wire serialization pending |
| Replay protection defined and authenticated | protocol/security research | not present in structural envelope | PENDING | accepted protocol/key design required |
| Wire parser bounds allocations before untrusted lengths | hostile-input requirement | object constructors bounded | PENDING | implement decoder with pre-allocation limits |
| Frame flags are fail-closed / versioned | protocol security requirement | raw `u16` currently accepted | PENDING | define reserved/known bits in protocol ADR |
| No unsafe Rust | project security baseline | crate root `forbid(unsafe_code)` | PASS | none for current crate |
| No secrets in source | security baseline | no secret material introduced | PASS | continuous CI scan |
| No custom cryptography | ADR-0008 gate | no crypto implementation added | PASS | approved cryptographic stack required later |
| Cross-platform binding cannot bypass kernel | PLATFORMS.md / FFI rule | documented interface rule | PASS | UniFFI ABI/security tests pending |
| Deterministic unit/negative tests | DoD / Test Matrix | tests committed | PASS | current-head CI verification pending |
| Independent CI verification | project gate | fresh current-head checks queued/in progress | PENDING | await workflow results |
| Independent security review | DoD | no review evidence yet | PENDING | required before security-sensitive merge |
| Production release of specialization | release gate | PR #81 draft | BLOCKED | verification + human review + dependency gates |

## Gap closure rule

A row marked **PENDING** or **BLOCKED** is not promoted to PASS by documentation. The gap is closed only by new repository evidence.

## Security posture

The deterministic kernel is materially hardened but remains a non-cryptographic security-boundary implementation slice. It is not a production E2E security protocol.
