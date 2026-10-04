# Eagle — Cryptography Release Gate

## G0 — No bespoke cryptography
PASS requires no custom primitive, custom ratchet, or undocumented protocol construction.

## G1 — Protocol conformance
PASS requires exact PQXDH and Double Ratchet profile/version, defined message encoding, conformance vectors, negative tests, and interoperability evidence.

## G2 — Key boundary
PASS requires isolated identity, prekey, session, message, storage, and recovery key domains with documented lifecycle.

## G3 — Platform protection
PASS requires Android Keystore testing, StrongBox capability testing where applicable, Apple Keychain testing, Secure Enclave capability testing, and desktop protected-storage testing.

## G4 — Device trust
PASS requires authenticated pairing, explicit trust changes, revocation, restore semantics, and key-change handling.

## G5 — Recovery and deletion
PASS requires independent backup encryption, recovery-abuse tests, deletion behavior evidence, and accurate user-facing claims.

## G6 — Adversarial testing
PASS requires fuzz/property tests, replay tests, ordering tests, rollback tests, downgrade tests, crash/power-loss tests, local-database theft tests, and interoperability tests.

## G7 — Supply chain
PASS requires exact dependency provenance, pinned versions, license review, security-advisory monitoring, and reproducible-build evidence where applicable.

## G8 — Independent review
PASS requires an independent cryptographic/security review and remediation or explicit authorized risk acceptance.

## G9 — Production authorization
Production release is denied unless G0 through G8 are PASS.

Current:
- G0–G5: design evidence complete.
- G6–G8: implementation/independent evidence required.
- G9: BLOCKED.
