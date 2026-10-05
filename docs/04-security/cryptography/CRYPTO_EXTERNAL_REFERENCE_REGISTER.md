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


## 2026-10-05 research refresh

- Signal PQXDH specification: asynchronous first-contact model explicitly assumes a server publishes/furnishes prekey bundles; strict P2P deployment therefore requires a separately defined transport/rendezvous profile and must not claim stock asynchronous interoperability without that profile.
- Signal Double Ratchet specification revision 4 (2025-11-04): message keys evolve per message and DH public values are mixed into the ratchet state.
- libsignal v0.104.0 is current upstream release observed during this review; upstream states use outside Signal is unsupported and the workspace is AGPL-3.0-only. Treat as reference/conformance candidate, not automatically approved dependency.
- vodozemac 0.10.0 is Apache-2.0 and reports one Least Authority audit with no significant findings; it implements Olm/Megolm and is not a PQXDH replacement.
- OpenMLS 0.9.0 is the current observed release and continues toward standards-compliant MLS RFC 9420; its release notes include security fixes and storage-state migration changes.
- NIST's current PQC migration guidance says ML-KEM and ML-DSA are ready for implementation; Eagle must track finalized standards rather than draft/withdrawn candidates.
- Android Keystore documentation confirms non-exportable key material, secure-hardware binding, StrongBox, and hardware key attestation. Hardware backing must be verified rather than assumed.
- Apple Secure Enclave documentation confirms hardware isolation but limits supported private-key operations/key types; Eagle must not mislabel unsupported PQXDH keys as Secure-Enclave protected.

## 2026-10-05 research refresh

- Signal PQXDH's standard asynchronous model uses a server for prekey publication/fetching; strict direct-P2P deployment therefore requires a separately frozen rendezvous profile.
- Signal Double Ratchet revision 4 (2025-11-04) continues the per-message key evolution and DH-mixing model.
- libsignal v0.104.0 is current upstream in this review; upstream explicitly says use outside Signal is unsupported and the workspace is AGPL-3.0-only.
- vodozemac 0.10.0 is Apache-2.0 and reports one Least Authority audit with no significant findings; it implements Olm/Megolm and is not a PQXDH replacement.
- OpenMLS 0.9.0 continues toward standards-compliant MLS RFC 9420 and includes recent security/storage hardening.
- NIST's 2026 PQC migration material continues to recommend finalized standards such as ML-KEM/ML-DSA rather than draft candidates.
- Android and Apple platform evidence requires capability-specific custody claims; hardware presence alone is insufficient.
