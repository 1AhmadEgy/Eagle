# Eagle — External Cryptography Reference Register

## Normative / primary references

- Signal PQXDH specification, Revision 3.
- Signal Double Ratchet specification, Revision 4.
- RFC 9420 — Messaging Layer Security (MLS).
- NIST FIPS 203 — ML-KEM.
- NIST FIPS 204 — ML-DSA.
- NIST SP 800-57 Part 1 Rev. 5 — current final key-management baseline.
- NIST SP 800-57 Part 1 Rev. 6 — tracked as Initial Public Draft until a final publication supersedes Rev. 5.

## Current review notes

### PQXDH / P2P

PQXDH is designed for asynchronous communication with prekey publication. Eagle's strict P2P requirement therefore needs a separately frozen deployment profile; stock PQXDH assumptions must not be silently changed.

### Double Ratchet

The adopted revision must be pinned exactly and validated against approved vectors. Eagle must not implement a bespoke ratchet.

### MLS

RFC 9420 remains the group protocol authority. Current OpenMLS 0.9.0 documents classical MLS ciphersuites and should therefore be treated as a group protocol implementation candidate, not as a post-quantum 1:1 provider.

### Post-quantum

FIPS 203 and FIPS 204 are final NIST standards for ML-KEM and ML-DSA. Both pages now carry errata/planning notes, so exact standard revision/errata tracking remains part of provenance. FIPS 203 defines ML-KEM as a KEM for shared-secret establishment; it is not a bulk-message encryption primitive.

No ad-hoc combination of ML-KEM/ML-DSA with Signal protocols is permitted. The adopted protocol must define the exact role, transcript binding, downgrade behavior, KDF, serialization, and key lifecycle.

### Key management

NIST SP 800-57 Rev. 5 remains the final baseline found in the official publication index, while Rev. 6 remains an Initial Public Draft with changes that explicitly distinguish key-establishment and key-storage guidance. Eagle tracks Rev. 6 for forward migration but does not treat the draft as normative.

### Platform custody

Android Keystore can expose whether enforcement is software, TEE, or StrongBox; StrongBox is a higher-assurance but more constrained tier and supports a narrower algorithm set. Apple Secure Enclave keeps the protected private key inside the enclave but is limited to supported algorithms, notably P-256 in the documented key-management flow. Therefore Eagle must prove actual platform/security level per key class rather than make blanket hardware-backed claims.

## Dependency rule

External cryptographic references are not dependency approval. Every production dependency requires exact version/commit, provenance, license review, platform support, interoperability/conformance, supply-chain checks, and independent security review.

---
