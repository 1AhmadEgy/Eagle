# Eagle — Cryptography & Key Management Master Baseline

**Specialty:** Cryptography / Key Management  
**Status:** Design-complete; implementation and production evidence gates remain open where evidence is not present.

## 1. Corpus and authority

Primary project evidence:
- docs/MASTER_PROJECT_SOURCE_INDEX.md
- docs/FILE_PROVENANCE_REGISTER.md
- docs/CHATGPT_ATTACHMENT_INTAKE_STATUS.md
- archive/chatgpt-historical/Eagle_MASTER_REFERENCE_INDEX.md
- archive/chatgpt-historical/Eagle_MASTER_PROJECT_REFERENCE.md
- archive/chatgpt-historical/ADR_Decision_Pack.md
- docs/03-architecture/adr/ADR-0009.md
- issues/ADR-0009.md
- docs/03-architecture/CANONICAL_PLANNING_BASELINE.md
- docs/04-security/SECURITY_BASELINE.md

Historical sources remain evidence/input until promoted by the project provenance rules.

## 2. Canonical cryptographic profile

### One-to-one messaging
- Protocol family: Signal Protocol profile.
- Session establishment: PQXDH.
- Message-key evolution: Double Ratchet.
- No bespoke cryptographic primitive or custom ratchet is permitted.

The PQXDH specification is asynchronous and assumes a service can publish prekey material. Eagle's P2P product constraint is therefore a separate transport/data-boundary decision; adopting PQXDH does not authorize central plaintext, secret-key, or unrestricted message storage.

### Group messaging
- Standards track: MLS, RFC 9420.
- No production implementation claim until a conformant implementation and interoperability evidence exist.

### Post-quantum migration
- ML-KEM is the primary NIST KEM family reference.
- ML-DSA is the primary NIST post-quantum signature reference.
- PQC algorithms are not substituted into Eagle message flows outside the adopted protocol profile without explicit protocol/version approval.

## 3. Implementation-library decision

libsignal is the reference implementation family for conformance study, but the upstream project states that use outside Signal is unsupported and licenses the repository under AGPLv3.

Decision:
- Use libsignal as reference/conformance material unless a separate license, API, platform, and integration review explicitly approves production use.
- Do not vendor it or bind it into Eagle solely because it is the reference implementation.
- Do not replace it with a bespoke Signal implementation merely to avoid dependency review.
- Production implementation requires an exact library, version/commit, license, provenance, target-platform support, test-vector pass, interoperability pass, and security review.

Current upstream version observed during review: v0.104.0.

## 4. Key hierarchy

Account
→ Device identity
→ Prekeys
→ Session state
→ Message keys
→ Local encrypted protocol state

Separate domain:
Recovery material

Hard separation rules:
- Identity keys are never storage-encryption keys.
- Session/message keys are never identity keys.
- Recovery material is never a universal decryption key.
- Transport credentials are never message keys.
- Logs and telemetry never contain private keys, session secrets, message keys, or plaintext.

## 5. Key lifecycle

Generate → Register → Publish public prekey material → Establish session → Ratchet → Rotate → Revoke → Recover where supported → Destroy obsolete state.

Requirements:
- CSPRNG from approved platform/library.
- Explicit key identifiers and versions.
- Fail closed on malformed or unknown cryptographic state.
- One-time material is consumed and deleted according to protocol semantics.
- Revocation prevents future authorization.
- Obsolete state is removed and storage protection is refreshed where required.

## 6. Identity and device trust

Each device owns an independent cryptographic identity.

Required fields:
- account identifier
- device identifier
- public identity key
- trust state
- revocation state
- protocol-state version
- platform assurance label

New-device linking is explicit and authenticated. Long-term private keys are not copied between devices through an informal export/import path.

## 7. Local protection

Recommended model:

Hardware/platform protection key
→ local wrapping key
→ encrypted protocol-state key
→ encrypted protocol database

The platform protection key is not the E2EE session key.

## 8. Platform contract

### Android
Use Android Keystore first. Prefer hardware-backed protection when supported. StrongBox is an optional stronger isolation tier. Key Attestation is an assurance mechanism, not the user's messaging identity mechanism.

Do not claim that X25519 or other protocol keys are hardware-backed unless the exact device/OS/API combination proves the required operation.

### Apple
Use Keychain for persistent secrets and cryptographic keys. Use Secure Enclave only for key types and operations actually supported by the target platform.

Do not claim Secure-Enclave residency for X25519/Ed25519 protocol identity keys without exact API/device evidence. Secure Enclave can be used for a compatible device-protection, wrapping, or authentication role.

### Desktop
Prefer native protected secret/key facilities on Windows, macOS, and Linux. Use encrypted application state as the fallback. Fallback protection must be classified as lower assurance when hardware protection is unavailable.

## 9. P2P cryptographic boundary

Identity
→ Session establishment
→ Message encryption
→ Encrypted envelope
→ P2P transport

No relay, discovery service, rendezvous service, or connectivity helper may receive:
- plaintext;
- private identity keys;
- session keys;
- message keys;
- recovery secrets.

Any asynchronous public prekey service must be limited to explicitly approved public/prekey material.

## 10. Recovery

- Account recovery and history recovery are different capabilities.
- No universal server-held recovery key.
- Recovery material is a separate trust domain.
- Backup encryption is independent from messaging session keys.
- Restore does not silently recreate trust.
- Restore does not silently un-revoke a device.
- Product claims must disclose irreversible-loss conditions.

## 11. Mandatory security invariants

1. Plaintext never crosses mesh/transport boundaries.
2. Private keys never cross platform boundaries without an explicitly reviewed protocol.
3. No secrets in logs, telemetry, crash reports, or diagnostics.
4. No silent downgrade.
5. No silent identity-key change acceptance.
6. Revoked devices cannot regain future authorization.
7. Recovery is not a universal decryptor.
8. Unsupported crypto/platform combinations fail closed.
9. Protocol state is versioned and migration is explicit.
10. Key purpose is explicit and non-overlapping.

## 12. Verification scope

Protocol:
- PQXDH conformance vectors.
- Double Ratchet vectors.
- malformed and invalid-state tests.
- duplicate, replay, out-of-order, loss, and reordering tests.
- downgrade and version-negotiation negative tests.
- interoperability tests.

Key management:
- generation
- persistence
- reload
- rotation
- revocation
- recovery
- deletion
- crash/power-loss consistency
- rollback protection
- migration

Platforms:
- Android hardware/software security levels
- StrongBox availability and fallback
- Android key invalidation and attestation
- Apple Keychain
- Apple Secure Enclave capability matrix
- desktop protected stores
- device migration and restore

Security:
- fuzzing/property tests
- memory-safety checks for the Rust core
- secret scanning
- dependency/SCA
- independent cryptographic review

## 13. Final status

Design: COMPLETE  
Corpus/provenance reconciliation: COMPLETE for available Git evidence  
Protocol profile: COMPLETE at architecture level  
Key hierarchy/lifecycle: COMPLETE  
Platform storage contract: COMPLETE  
Recovery model: COMPLETE  
P2P crypto boundary: COMPLETE  
Threat controls: COMPLETE  
Verification plan: COMPLETE  
Executable crypto implementation: NOT EVIDENCED in current baseline  
Exact production library/version: OPEN GATE  
Independent cryptographic review: REQUIRED  
Production release approval: NOT GRANTED
