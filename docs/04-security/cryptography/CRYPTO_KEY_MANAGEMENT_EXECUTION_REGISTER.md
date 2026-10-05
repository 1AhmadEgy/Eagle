# Eagle — Cryptography / Key Management Execution Register

| Stage | Result |
|---|---|
| Inventory | COMPLETE |
| Provenance | COMPLETE for available Git evidence |
| Classification | COMPLETE |
| Document audit | COMPLETE |
| Historical version comparison | COMPLETE for located crypto records |
| Canonical reference | COMPLETE for available specialty evidence |
| Requirements and gaps | COMPLETE at design level |
| Design corrections | COMPLETE |
| Implementation preparation | COMPLETE |
| Test design | COMPLETE |
| Security design review | COMPLETE |
| Executable verification | PARTIAL — key custody contract has executable unit coverage; cryptographic/storage integration remains blocked |
| Independent cryptographic review | REQUIRED |
| Production release gate | BLOCKED |

## Specialty decisions

CRYPTO-DEC-001: Signal Protocol remains the one-to-one cryptographic reference.
CRYPTO-DEC-002: PQXDH is the target session-establishment profile.
CRYPTO-DEC-003: Double Ratchet is the target message-key evolution profile.
CRYPTO-DEC-004: MLS is the group-crypto standards track.
CRYPTO-DEC-005: No custom cryptographic primitive or bespoke ratchet.
CRYPTO-DEC-006: libsignal remains reference/conformance material until explicit production authorization.
CRYPTO-DEC-007: Hardware-backed platform protection is preferred, but claims require device/API evidence.
CRYPTO-DEC-008: Account recovery and history recovery remain separate trust domains.
CRYPTO-DEC-009: Transport never receives plaintext or E2EE secret material.

## Executable foundation

KEY-FOUNDATION-001: Rust core exposes only non-secret key references, purpose-bound custody policy, and lifecycle state.
KEY-FOUNDATION-002: All current key policies are non-exportable; export attempts fail closed.
KEY-FOUNDATION-003: Revocation is terminal at the key-contract layer.
KEY-FOUNDATION-004: Long-lived key purposes require platform-secure custody; hardware backing can be made mandatory per policy.
KEY-FOUNDATION-005: Session keys are classified as ephemeral by policy.
KEY-FOUNDATION-006: This contract does not claim secure hardware storage, key generation, cryptographic execution, or cross-platform enforcement until platform implementations and tests exist.

## Evidence rule

A design stage is not equivalent to executable PASS. Implementation, conformance, adversarial verification, and independent review remain separate gates.


## 2026-10-05 lifecycle hardening

KEY-LIFECYCLE-001: Key purposes are represented by distinct typed handles; cross-purpose use is rejected by policy.

KEY-LIFECYCLE-002: Key generations must be non-zero and strictly increase during rotation.

KEY-LIFECYCLE-003: Lifecycle epochs are strictly monotonic and epoch exhaustion fails closed.

KEY-LIFECYCLE-004: Rotation reserves all metadata transitions before mutation so a failed rotation cannot leave a new active key without revoking the predecessor.

KEY-LIFECYCLE-005: One-Time PreKeys transition to CONSUMED exactly once.

KEY-LIFECYCLE-006: Consumed, revoked, and destroyed key states cannot satisfy active-key requirements.

PROVIDER-POLICY-001: A production provider requires exact version, exact revision, license review, support review, platform review, protocol conformance and independent review.

PROVIDER-POLICY-002: The default provider is unavailable and therefore cannot silently execute plaintext fallback.

PROVIDER-POLICY-003: No candidate library is treated as production-approved solely from popularity, license, or protocol name.

## Current executable boundary

The current executable work is a **metadata/policy boundary only**. It is deliberately not a cryptographic implementation. This distinction is maintained until a full provider is approved.
