# Eagle — Cryptography & Key Management Master Baseline

**Specialty:** Cryptography / Key Management
**Status:** Design-complete; implementation and production evidence gates remain open where evidence is not present.

## Canonical cryptographic profile

### One-to-one
- Protocol family: Signal Protocol profile.
- Session establishment: PQXDH.
- Ongoing message-key evolution: Signal-defined Triple Ratchet profile (Double Ratchet + SPQR/SCKA) for hybrid post-quantum protection.
- Double Ratchet alone remains an interoperability fallback only when explicitly required and reviewed.
- No bespoke cryptographic primitive or custom ratchet.

### Group
- Standards track: MLS, RFC 9420.
- Production group implementation requires conformance and interoperability evidence.

### Library posture
libsignal is reference/conformance material only until license, support, API, platform, provenance, exact-version, and security review explicitly authorize production use. Current upstream research/release posture is treated as evidence for protocol capability, not as automatic authorization.

vodozemac is a candidate Rust implementation for ratchet-related research, but it is not treated as a drop-in replacement for PQXDH. Eagle must not assemble a new protocol from components without a dedicated protocol review.

## Key hierarchy

Account
-> Device identity
-> Prekeys
-> Session state
-> Message keys
-> Local encrypted protocol state

Separate domain: Recovery material.

Hard separation:
- Identity keys are never storage-encryption keys.
- Session/message keys are never identity keys.
- Recovery material is never a universal decryption key.
- Transport credentials are never message keys.
- Logs/telemetry never contain private keys, session secrets, message keys, or plaintext.

## Platform protection

### Android
Android Keystore is the primary key boundary. Prefer hardware-backed protection where supported. StrongBox is a higher-assurance capability tier. Attestation is assurance evidence, not the messaging identity root.

### Apple
Keychain is the primary persistent secret/key store. Secure Enclave is used only for supported algorithms/operations and only with device/API evidence.

### Desktop
Prefer native OS protected credential/key stores. Encrypted application state is a lower-assurance fallback when hardware-backed/native protection is unavailable.

## Recovery

Account recovery and history recovery are separate capabilities.

No universal server-held recovery key.
Restore must not silently recreate trust.
Restore must not silently un-revoke a device.
Backup encryption is independent from messaging session keys.

## Mandatory invariants

1. Plaintext never crosses mesh/transport boundaries.
2. Private keys never cross platform boundaries without explicit review.
3. No secrets in logs, telemetry, crash reports, or diagnostics.
4. No silent downgrade.
5. No silent identity-key change acceptance.
6. Revoked devices cannot regain future authorization.
7. Recovery is not a universal decryptor.
8. Unsupported crypto/platform combinations fail closed.
9. Protocol state is versioned and migrations are explicit.
10. Key purpose is explicit and non-overlapping.

## Verification

Protocol:
- PQXDH and Signal-defined Triple Ratchet profile vectors.
- invalid-state, replay, ordering, loss, downgrade, and interoperability tests.

Key management:
- generation, persistence, reload, rotation, revocation, recovery, deletion, crash/power-loss, rollback, migration.

Platforms:
- Android Keystore/StrongBox/attestation.
- Apple Keychain/Secure Enclave capability matrix.
- desktop protected stores.

Security:
- fuzz/property tests.
- memory-safety checks.
- secret scanning.
- dependency/SCA.
- independent cryptographic review.

## Status

Design: COMPLETE.
Executable crypto implementation: NOT EVIDENCED.
Exact production library/version authorization: OPEN.
Independent cryptographic review: REQUIRED.
Production release: NOT GRANTED.


## Current hardening baseline — 2026-10-05

The production crypto provider gate is a compound proof: exact immutable provider revision; license/support/platform review; conformance and independent review; non-exportable identity capability; hardware protection and applicable hardware-backed attestation; explicit protocol profile binding; finalized PQC algorithm selection; and separately proven strict-P2P semantics. Missing evidence fails closed and cannot be converted to PASS by documentation alone.


## Approval invariants

Production approval requires the provider record to bind the exact PQXDH + Triple Ratchet profile and demonstrate non-exportable identity keys, hardware protection/attestation, PQ KEM, message-ratchet and post-quantum-ratchet capabilities.