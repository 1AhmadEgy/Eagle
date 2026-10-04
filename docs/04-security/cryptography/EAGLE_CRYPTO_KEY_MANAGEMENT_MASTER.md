# Eagle — Cryptography / Key Management Master Baseline

**Scope:** Cryptography / Key Management only  
**Baseline:** security/reconciled-foundation-2026-10-05 + Rust security-kernel head ddad0be5f7fb1700084215c20b5fff68c3781354  
**Status:** Architecture complete; provider implementation and independent review remain mandatory evidence gates.

## Corpus and canonical authority

The specialty is reconciled against the repository's provenance and historical reference material, including:

- `docs/MASTER_PROJECT_SOURCE_INDEX.md`
- `docs/FILE_PROVENANCE_REGISTER.md`
- `archive/chatgpt-historical/Eagle_MASTER_REFERENCE_INDEX.md`
- `archive/chatgpt-historical/Eagle_MASTER_PROJECT_REFERENCE.md`
- `archive/chatgpt-historical/ADR_Decision_Pack.md`
- `docs/03-architecture/adr/ADR-0009.md`
- the dedicated cryptography package under `docs/04-security/cryptography/`

Historical material is evidence/input, not automatically canonical.

## 1. Cryptographic profile

### One-to-one

Signal Protocol family is the protocol reference.

- Session establishment reference: PQXDH.
- Message-key evolution: Double Ratchet.
- No custom primitive.
- No bespoke ratchet.
- No protocol-profile drift without an ADR and conformance evidence.

PQXDH is specified for asynchronous operation with published prekey material. Strict P2P therefore creates a deployment constraint: Eagle must not introduce a central trust root or unrestricted server-held secret material. The exact P2P bootstrap/prekey publication profile must be frozen before interoperability is claimed. citeturn104616search0

The current Double Ratchet specification is Revision 4 (2025-11-04). Eagle must pin the exact adopted revision/profile and test vectors; it must not implement an older paper or an ad-hoc variant. citeturn779275search0

### Group

MLS RFC 9420 is the standards-track reference for group key establishment, with forward secrecy and post-compromise security. Production adoption remains blocked until an implementation and interoperability corpus are verified. citeturn779275search1

## 2. Library policy

`libsignal` is reference/conformance material only until its exact production use is legally, technically, and operationally approved.

Current upstream facts verified during review:

- repository license: AGPL-3.0-only;
- upstream README explicitly says use outside Signal is unsupported;
- APIs/bridge layers may change without notice;
- workspace version observed: 0.104.0. citeturn779275search3turn104616search2

Therefore:

1. Do not copy Signal crypto code into Eagle.
2. Do not write a replacement Signal implementation.
3. Do not mark libsignal "approved for production" solely because it is the reference implementation.
4. Production dependency approval requires exact commit/version, license decision, provenance, target-platform build proof, conformance vectors, interoperability proof, and independent security review.
5. Until that gate passes, Eagle's crypto boundary is intentionally fail-closed.

## 3. Post-quantum path

- ML-KEM is the NIST KEM reference.
- ML-DSA is the NIST signature reference.
- Eagle may use them only where the adopted protocol profile defines their use; it must not bolt PQ primitives onto message formats ad hoc.

NIST FIPS 203 defines ML-KEM-512/768/1024; FIPS 204 defines ML-DSA. Both are current NIST standards, with published errata notes that must be tracked. citeturn779275search2turn104616search8

## 4. Key hierarchy

Account identity
→ Device identity key
→ Signed prekey / one-time prekeys / PQ prekey material
→ Session state
→ Message keys

Separate trust domains:

Storage-wrapping key
Recovery key

Hard rules:

- identity keys are never storage-wrapping keys;
- session/message keys are never identity keys;
- recovery material is never a universal decryptor;
- transport credentials are never message keys;
- secret material never enters logs, telemetry, crash reports, or diagnostics.

## 5. Typed core boundary

The Rust core now contains a **typed, fail-closed boundary** without implementing cryptographic primitives.

Key-domain handles are deliberately distinct:

- IdentityKeyHandle
- SignedPreKeyHandle
- OneTimePreKeyHandle
- PostQuantumPreKeyHandle
- SessionKeyHandle
- MessageKeyHandle
- StorageWrappingKeyHandle
- RecoveryKeyHandle

The provider interfaces never expose private key bytes. The default provider is unavailable and returns `ProviderUnavailable`. This prevents the scaffold from accidentally becoming production crypto.

## 6. Platform storage contract

### Android

Android Keystore is the primary secure storage boundary. Prefer hardware-backed protection where supported; StrongBox is a stronger optional isolation tier. Key Attestation is assurance evidence, not the application's messaging identity mechanism. OWASP explicitly calls for cryptographic keys to be stored in the platform secure keystore. citeturn729934search5turn729934search9

### Apple

Keychain is the persistent secret/key store. Secure Enclave is only used for operations/key types that the platform actually supports. Apple's current documentation states that Secure Enclave protects P-256 keys and cannot import preexisting keys. Consequently Eagle must not claim that X25519/Ed25519/other protocol identity keys are Secure-Enclave resident unless exact platform evidence exists. citeturn729934search0turn729934search1

### Desktop

Use native protected key/credential facilities per OS. Application-level encrypted storage is fallback only and receives a lower assurance label when hardware-backed protection is unavailable.

## 7. Lifecycle

Generate → register/publication → session use → ratchet → rotate → revoke → destroy.

Additional requirements:

- CSPRNG from approved implementation/library;
- explicit key identifiers and versions;
- fail-closed on invalid or unknown key state;
- one-time key material is consumed according to the adopted protocol;
- revocation is monotonic;
- deletion/rotation is tested across crash, power-loss, restore, and migration paths.

NIST SP 800-57 Part 1 Rev. 5 is the key-management baseline for lifecycle and protection requirements. citeturn729934search4

## 8. Recovery

Recovery and message-history decryption remain separate capabilities.

- no universal server-held decryption key;
- backup encryption uses an independent key domain;
- restore does not silently recreate trust;
- restore does not silently un-revoke devices;
- irreversible-loss conditions are explicit;
- recovery operations are auditable.

## 9. Mandatory invariants

1. Plaintext never crosses the transport/mesh boundary.
2. No private key bytes leave the approved provider boundary.
3. No silent downgrade.
4. No silent identity-key change acceptance.
5. Revoked devices cannot regain future authorization.
6. Unsupported cryptographic capability fails closed.
7. Protocol state is versioned and migration is explicit.
8. Key purpose is non-overlapping and typed.
9. Crypto provider selection is explicit and pinned.
10. Release authorization requires real evidence.

## 10. Verification and release

Required before production:

- PQXDH conformance and negative vectors;
- Double Ratchet conformance and reordering/loss/replay vectors;
- group MLS interoperability if groups ship;
- platform keystore/invalidation/restore testing;
- fuzzing and property tests;
- dependency/SCA and provenance review;
- independent cryptographic review;
- exact-version reproducibility;
- release gate G0–G9 in `CRYPTO_RELEASE_GATE.md`.

**Current status:** design complete; implementation boundary hardened; production crypto is **not approved** until an audited provider/library and all evidence gates pass.
