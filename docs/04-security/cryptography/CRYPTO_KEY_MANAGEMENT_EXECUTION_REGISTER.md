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
CRYPTO-DEC-010: Production wire serialization must use an approved standard format; the current minicbor-based fixed-array baseline is implementation evidence only until protocol review closes.

## Executable foundation

KEY-FOUNDATION-001: Rust core exposes only non-secret key references, purpose-bound custody policy, and lifecycle state.
KEY-FOUNDATION-002: All current key policies are non-exportable; export attempts fail closed.
KEY-FOUNDATION-003: Revocation is terminal at the key-contract layer.
KEY-FOUNDATION-004: Long-lived key purposes require platform-secure custody; hardware backing can be made mandatory per policy.
KEY-FOUNDATION-005: Session keys are classified as ephemeral by policy.
KEY-FOUNDATION-006: This contract does not claim secure hardware storage, key generation, cryptographic execution, or cross-platform enforcement until platform implementations and tests exist.

## Evidence rule

A design stage is not equivalent to executable PASS. Implementation, conformance, adversarial verification, and independent review remain separate gates.
