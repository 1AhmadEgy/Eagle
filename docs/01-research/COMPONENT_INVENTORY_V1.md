# Eagle — Current Component Inventory v1

**Verified:** 2026-10-04  
**Branch:** implementation/repository-consistency-v1  
**Rule:** code is evidence; architecture prose alone is not implementation evidence.

| Area | Status | Evidence | Decision |
|---|---|---|---|
| Android app bootstrap | Implemented | `app/`, Android Test Lab success | Continue |
| Android build | Implemented | AGP 9.4.0, SDK 37/preview installation verified, unit/lint/build CI | Freeze while compatible |
| Rust state-machine foundation | Implemented foundation-only | `core/`, tests, rust-core CI | Keep and extend safely |
| Production Rust Security Core | Not implemented | No crypto/key boundary/envelope implementation | Do not claim implemented |
| Device cryptographic identity | Not implemented | No key generation/attestation boundary | Use Android Keystore + approved core later |
| 1:1 E2E protocol | Not approved/integrated | Protocol matrix only | Select before integration |
| libsignal | Not integrated | External research; support/license concerns | Study, do not add yet |
| vodozemac | Not integrated | Upstream v0.11.1, Apache-2.0 | Study/adapt candidate |
| OpenMLS | Not integrated | RFC 9420 implementation candidate | Defer to group phase |
| Noise | Not integrated | Handshake framework candidate | Study as building block |
| libp2p | Not integrated | P2P transport candidate | Adapt later |
| KMP shared layer | Not implemented | Architecture only | Later |
| Persistent message storage | Not implemented | No DB dependency | Decide after data/security model |
| Mesh | Not implemented | No working mesh engine | Later |
| CI/security policy | Implemented | Repository consistency + security-policy gates pass | Continue hardening |

## Adoption gate

No security-sensitive dependency is promoted from study to integration without:

`Requirement → Architecture Fit → Security History → Exact Version/Commit → License → Transitive Dependencies → Test Evidence → Operational Fit → Exit Strategy → Approval`

## Explicit safety rule

The Rust foundation is a policy/state layer. It does not verify cryptographic identity and does not encrypt messages. Any future API that performs those roles must be introduced only with the corresponding protocol/key-management evidence.
