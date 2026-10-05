# Security Requirements Traceability — Eagle

## Status semantics

VERIFIED = backed by observed implementation/evidence.
IMPLEMENTED = code exists but production proof is incomplete.
PARTIAL = some controls exist, cross-boundary proof is incomplete.
PENDING = required decision or implementation not yet evidenced.
BLOCKED = release cannot proceed until closed.

| ID | Requirement | Source / rationale | Current evidence | Status |
|---|---|---|---|---|
| SEC-001 | Network reachability never grants trust | security architecture + identity threat model | Rust trust gates | IMPLEMENTED |
| SEC-002 | Unknown/pending device cannot authorize | identity/security kernel | unit + integration negative tests | VERIFIED |
| SEC-003 | Revoked/replaced device cannot authorize | identity model | terminal-state tests | VERIFIED |
| SEC-004 | Protocol downgrade cannot silently succeed | security kernel + protocol baseline | version negative tests | VERIFIED |
| SEC-005 | Malformed/oversized identifiers and payloads are rejected before acceptance | parser boundary | constructor/length tests | VERIFIED |
| SEC-006 | No custom cryptographic primitives/ratchet | security architecture + crypto baseline | no crypto primitives in current core | VERIFIED |
| SEC-007 | Production E2E uses an approved standard protocol profile | crypto baseline | Signal/PQXDH target documented | PENDING |
| SEC-008 | Private keys remain inside approved secure-storage boundary | key-management baseline | non-exportable device-scoped key contract plus Android Keystore local-storage adapter; identity-key custody and cross-platform enforcement remain open | PARTIAL |
| SEC-009 | Application content never uses relay fallback | P2P architecture | policy/ADR documented; runtime transport absent | PENDING |
| SEC-010 | Direct P2P transport authenticates the peer and binds to Eagle identity | P2P threat model | implementation absent | PENDING |
| SEC-011 | Recovery cannot silently restore revoked authority or decrypt history | identity/key recovery model | design baseline exists | PENDING |
| SEC-012 | Security-critical UI/platform code cannot bypass Rust security policy | Rust kernel contract | current API boundary supports fail-closed control | IMPLEMENTED |
| SEC-013 | Third-party CI actions are fully pinned to immutable 40-character SHAs | CI security policy | corrected Test Lab workflow passes policy scan | VERIFIED |
| SEC-014 | Repository secret scan must pass | CI/security baseline | Gitleaks pass observed | VERIFIED |
| SEC-015 | Required test categories cannot be marked PASS when absent | AGENTS/Test Lab rules | category-aware verification infrastructure exists | VERIFIED |
| SEC-016 | Independent cryptographic/security review is required before release | release gate | gate document | PENDING |

## Critical gate

The existence of a requirement, design, or unit test does not imply production readiness. Cross-boundary and adversarial evidence remain mandatory.
