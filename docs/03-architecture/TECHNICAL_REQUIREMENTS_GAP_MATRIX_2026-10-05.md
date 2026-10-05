# Technical Requirements & Gap Matrix — 2026-10-05

## Status legend

- VERIFIED = directly supported by current repository evidence
- DERIVED = architecture consequence of verified requirements
- PENDING = authoritative evidence/decision missing
- BLOCKED = cannot proceed safely
- PASS = verified implementation evidence only

| ID | Requirement / invariant | Source | Implementation | Test evidence | Status |
|---|---|---|---|---|---|
| AR-001 | P2P-only messaging architecture | project constraint + current architecture records | architecture baseline | architecture tests pending | DERIVED |
| AR-002 | No plaintext in mesh | planning baseline / security boundary | protocol/mesh contracts pending | negative tests pending | DERIVED |
| AR-003 | No private keys in UI/storage records | ADR-0009/0016 boundaries | security/storage contracts pending | negative tests pending | DERIVED |
| AR-004 | Rust owns security-sensitive semantics | platform strategy + Rust specialty PR | partial Rust foundation | current-head verification pending | DERIVED |
| AR-005 | Shared KMP application layer | accepted platform strategy | target only | build evidence pending | PENDING |
| AR-006 | Platform-specific behavior isolated behind adapters | platform strategy | target only | architecture enforcement pending | PENDING |
| AR-007 | Cryptography uses established implementation, no custom primitives | security baseline / crypto specialty | reference profile documented | interop + independent crypto review pending | DERIVED |
| AR-008 | Identity/device trust separation | identity specialty | PR #69 candidate | adversarial E2E pending | PENDING |
| AR-009 | Explicit fail-closed recovery states | ADR-0016 candidate | storage contract candidate | negative-path storage tests pending | PENDING |
| AR-010 | Protocol downgrade rejected | Rust foundation | implemented in specialty branch | current-head CI pending | PARTIAL |
| AR-011 | Security-sensitive changes get dual independent review | Definition of Done / ADR review records | governance rule exists | reviewer evidence pending per change | VERIFIED |
| AR-012 | Required test categories cannot be implied PASS | AGENTS.md / Test Lab rules | CI category framework exists | category evidence incomplete | VERIFIED |
| AR-013 | Current Android module is `:app` | current build files | implemented | repository inspection PASS | VERIFIED |
| AR-014 | Requirements are frozen before product implementation | execution-readiness policy | not fully frozen | approval evidence absent | BLOCKED |
| AR-015 | Release requires build/unit/integration/crypto/protocol/security/static/dependency/regression/fuzz evidence | Definition of Done | partial infrastructure | complete matrix pending | BLOCKED |

## Security-priority open gaps

- G-001: authoritative V1 requirements are incomplete.
- G-002: ADR-0007 through technical ADR approvals are not all closed.
- G-003: KMP shared module is target architecture, not a verified current implementation.
- G-004: Rust FFI/UniFFI production boundary is not yet evidenced.
- G-005: cryptographic dependency/provider exact version and license/support decision remains gated.
- G-006: protocol interoperability evidence is absent.
- G-007: storage backend and recovery implementation evidence is absent.
- G-008: P2P transport/discovery implementation and adversarial network testing are absent.
- G-009: cross-platform parity evidence is absent.
- G-010: full Release Gate evidence is absent.
- G-011: current conversation-only artifacts are not durably binary-transferred.
- G-012: active branch cleanup/disposition remains owner-controlled.

## Gap closure order

G-001 → G-002 → G-003/G-004 → G-005/G-006 → G-007/G-008 → G-009 → G-010
