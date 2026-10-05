# Eagle — External Cryptography Reference Register

## Normative / primary references

- Signal PQXDH specification, Revision 3 (last updated 2024-01-23).
- Signal Double Ratchet specification, Revision 4 (2025-11-04).
- RFC 9420 — Messaging Layer Security (MLS).
- NIST FIPS 203 — ML-KEM.
- NIST FIPS 204 — ML-DSA.
- NIST SP 800-57 Part 1 Rev. 5 — key-management guidance.

## Current review notes

### PQXDH
PQXDH is designed for asynchronous communication where a user can publish prekey information while offline. Eagle's P2P-only constraint therefore requires a separately frozen deployment profile; stock PQXDH assumptions must not be silently changed.

### Double Ratchet
The adopted specification must be pinned to an exact revision/profile and validated against approved vectors. Eagle must not implement an older paper or a bespoke variant.

### MLS
RFC 9420 provides asynchronous group key establishment with forward secrecy and post-compromise security. Eagle will not claim production MLS support until an implementation and interoperability evidence exist.

### Post-quantum
ML-KEM and ML-DSA are standardized NIST algorithms. Eagle treats them as protocol-approved building blocks only where the adopted protocol profile explicitly specifies their role; no ad-hoc PQ composition is permitted.

### Key management
NIST SP 800-57 Part 1 Rev. 5 remains the current final publication found during this review; NIST's Revision 6 material was still an initial public draft in the official project record, so it is tracked as forward-looking input rather than the normative baseline.

## Dependency rule

External cryptographic references are not dependency approval. Every production dependency requires exact version/commit, provenance, license review, platform support, conformance, supply-chain checks, and independent security review.
