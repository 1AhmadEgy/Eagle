# Eagle — Cryptography Release Gate

## G0 — No bespoke cryptography
PASS only when no custom primitive, custom ratchet, or undocumented protocol composition is shipped.

## G1 — Protocol conformance
PASS requires an exact PQXDH/Double Ratchet profile, version, encodings, conformance vectors, negative tests and interoperability evidence.

## G2 — Key separation
PASS requires distinct lifecycle domains for identity, prekeys, sessions, messages, storage wrapping and recovery.

## G3 — Platform protection
PASS requires Android Keystore/StrongBox evidence where applicable, Apple Keychain/Secure Enclave capability evidence, and desktop protected-storage evidence.

## G4 — Device trust
PASS requires authenticated pairing, identity-key change handling, revocation and restore semantics.

## G5 — Recovery/deletion
PASS requires independent backup protection, recovery-abuse tests, deletion behavior evidence and accurate product claims.

## G6 — Adversarial testing
PASS requires fuzzing/property tests, replay/order/loss tests, rollback and downgrade tests, crash/power-loss tests, local-state theft tests and interoperability tests.

## G7 — Supply chain
PASS requires exact dependency provenance, version pinning, SBOM/license review, advisory monitoring and reproducible-build evidence where applicable.

## G8 — Independent review
PASS requires independent cryptographic/security review and disposition of findings.

## G9 — Production authorization
Production release is denied unless G0–G8 are PASS.

Current state: G0–G5 have architecture evidence; G6–G8 require implementation/independent evidence; G9 is BLOCKED.
