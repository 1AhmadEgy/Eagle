# Eagle — Identity & Trust Requirements / Gaps Matrix

**Scope:** Identity & Trust Engineering only  
**Status:** Controlled working baseline  
**Security posture:** fail-closed / P2P-only  
**Canonical implementation baseline:** `main` @ `6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee`

| ID | Requirement | Source / authority | Implementation | Test/evidence | Status | Blocking dependency |
|---|---|---|---|---|---|---|
| IDT-R01 | Account identity is independent of username/profile metadata | Identity spec + historical corpus | Defined | Structural review | BASELINE | Key identity profile |
| IDT-R02 | Device identity is distinct from account identity | Identity spec | Defined | Membership tests | BASELINE | Key hierarchy |
| IDT-R03 | Device identity is distinct from session keys | Identity spec / architecture | Defined | Boundary review | BASELINE | Protocol ADR |
| IDT-R04 | Trust is explicit state, not login side effect | ADR input + spec | Implemented | IT-001/IT-002 | PASS-local | ADR-005 approval |
| IDT-R05 | PENDING cannot self-promote | Security invariant I-03 | Implemented | IT-001 | PASS-local | None |
| IDT-R06 | Trust promotion requires verified approval evidence | Security invariant I-16 | Implemented verifier boundary | IT-027 | PASS-local | Final pairing protocol |
| IDT-R07 | Pairing binds account identity | Pairing spec | Implemented structural guard | IT-SC-005 | PASS-local | Transcript authentication |
| IDT-R08 | Pairing binds target device identity | Pairing hardening | Implemented | IT-SC-013 / IT-023 | PASS-local | Final protocol proof |
| IDT-R09 | Pairing is single-use | Pairing spec | Implemented | IT-SC-003 / IT-004 | PASS-local | Atomic persistence |
| IDT-R10 | Pairing expires | Pairing spec | Implemented | IT-SC-004 / IT-005 | PASS-local | Clock/persistence policy |
| IDT-R11 | Pairing is non-resumable after cancellation | Pairing spec | Implemented | IT-SC-012 / IT-012 | PASS-local | None |
| IDT-R12 | Pairing replay cannot authorize trust | Threat model | Boundary implemented | IT-SC-003 | PASS-local | Cryptographic transcript proof |
| IDT-R13 | Identity-key change causes quarantine | Threat model | Implemented | IT-SC-008 / IT-018 | PASS-local | Successor-proof protocol |
| IDT-R14 | Revoked device cannot authorize new sessions | Threat model | Implemented | IT-SC-007 / IT-007 | PASS-local | Distributed reconciliation |
| IDT-R15 | Replaced device is terminal | Trust state model | Implemented | IT-009 | PASS-local | Recovery policy |
| IDT-R16 | Stale trust epoch cannot restore authority | Threat model | Implemented | IT-SC-006 / IT-006 | PASS-local | Epoch distribution |
| IDT-R17 | Trust epoch is monotonic / overflow-safe | Threat model | Implemented | IT-011 / IT-021 / IT-022 | PASS-local | Persistent rollback detection |
| IDT-R18 | Account recovery does not imply historical data recovery | ADR-007 input | Implemented boundary | IT-SC-009 / IT-010 | PASS-local | Recovery ADR |
| IDT-R19 | Platform attestation is evidence, not root of trust | Platform security model | Enum/interface | IT-SC-010 | PASS-local | Platform adapter policy |
| IDT-R20 | Hardware identifiers are not identity roots | Privacy requirement | Implemented by model | Static/security review | PASS-local | Full privacy review |
| IDT-R21 | Security audit events exclude secrets | Security architecture | Implemented redaction boundary | Static/security review | PASS-local | Final telemetry policy |
| IDT-R22 | Unknown/inconsistent state fails closed | Security kernel | Implemented | Negative suite | PASS-local | State completeness review |
| IDT-R23 | Cross-account device reuse is rejected | Membership model | Implemented | IT-015 | PASS-local | Key-binding verifier |
| IDT-R24 | Duplicate device membership is rejected | Membership model | Implemented | IT-016 | PASS-local | Membership persistence |
| IDT-R25 | Membership validity windows are checked | Membership model | Implemented | IT-014 / IT-025 | PASS-local | Serialization |
| IDT-R26 | Malformed pairing context is rejected | Pairing hardening | Implemented | IT-024 | PASS-local | Serialization |
| IDT-R27 | Pairing expiry overflow fails closed | Pairing hardening | Implemented | IT-026 | PASS-local | None |
| IDT-G01 | Final account/device cryptographic key hierarchy | ADR-0009 | Missing / interface only | None | GAP | ADR-0009 |
| IDT-G02 | Final pairing transcript authentication | ADR-0008/0009 | Missing / interface only | Deferred adversarial tests | GAP | ADR-0008/0009 |
| IDT-G03 | Identity signature verification | ADR-0008/0009 | Missing | Deferred | GAP | Protocol/key selection |
| IDT-G04 | Identity successor / rotation proof | Key lifecycle | Missing | Deferred | GAP | Key hierarchy |
| IDT-G05 | Offline revocation convergence | Threat model | Missing E2E evidence | Deferred | GAP | Protocol + persistence |
| IDT-G06 | Persistent rollback detection | Threat model | Missing | Deferred | GAP | Storage ADR |
| IDT-G07 | Cross-platform parity | Platform strategy | Missing evidence | Deferred | GAP | Android/Desktop/iOS adapters |
| IDT-G08 | Recovery protocol and recovery authority | ADR-007 | Missing | Deferred | GAP | ADR-007 |
| IDT-G09 | Device-count policy | ADR-006 | Missing | None | GAP | Product/security decision |
| IDT-G10 | Independent security review | Release gate | Missing | None | GAP | External reviewer |

## Status semantics

- **PASS-local** = verified by the Security Core/unit-level boundary; not equivalent to end-to-end protocol security.
- **BASELINE** = defined but not independently proven.
- **GAP** = unresolved and release-blocking.
- **PASS** at workstream release means CI + protocol + cross-platform + security evidence all pass.

## Security decision

The safest admissible state at this stage is:

```text
Local identity/trust policy = implemented and testable
Protocol/key authority     = pending
Cross-platform proof       = pending
Independent security audit = pending
Release                    = BLOCKED
```
