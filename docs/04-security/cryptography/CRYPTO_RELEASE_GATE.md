# Eagle — Cryptography Release Gate

## G0 — No bespoke cryptography
PASS only when no custom primitive, custom ratchet, or undocumented protocol construction is used.

## G1 — Protocol conformance
Requires exact protocol profile/version, defined encoding, conformance vectors, negative tests, and interoperability evidence.

## G2 — Key boundary
Requires isolated identity, prekey, session, message, storage, and recovery key domains with lifecycle evidence.

## G3 — Platform protection
Requires Android Keystore/StrongBox testing, Apple Keychain/Secure Enclave capability testing, and desktop protected-store testing.

## G4 — Device trust
Requires authenticated pairing, explicit trust changes, revocation, restore semantics, and key-change handling.

## G5 — Recovery and deletion
Requires independent backup encryption, recovery-abuse tests, deletion evidence, and accurate product claims.

## G6 — Adversarial testing
Requires fuzz/property, replay, ordering, rollback, downgrade, crash/power-loss, database-theft, and interoperability tests.

## G7 — Supply chain
Requires exact dependency provenance, pinned versions, license review, security monitoring, and controlled/reproducible build evidence where applicable.

## G8 — Independent review
Requires independent cryptographic/security review and documented remediation or authorized risk acceptance.

## G9 — Production authorization
Production is denied unless G0 through G8 are PASS.

Current:
- G0-G5: design evidence complete.
- G6-G8: implementation/independent evidence required.
- G9: BLOCKED.


## Additional hard gates — 2026-10-05

- Exact provider revision and explicit protocol profile binding verified.
- Required hardware-backed attestation/security-anchor evidence verified for each platform profile where claimed.
- No unsupported Secure Enclave/StrongBox custody claims for PQXDH/MLS keys.
- Strict-P2P profile semantics independently conformance-tested; no stock asynchronous PQXDH interoperability claim without its required rendezvous semantics.
- Current PQC algorithm selection is bound to finalized standards and reviewed against the current NIST migration baseline.
