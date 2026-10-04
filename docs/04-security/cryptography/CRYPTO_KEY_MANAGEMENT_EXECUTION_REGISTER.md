# Eagle — Cryptography / Key Management Execution Register

## Ordered specialty status

| Stage | Status |
|---|---|
| Inventory | COMPLETE |
| Provenance | COMPLETE for available Git evidence |
| Classification | COMPLETE |
| Document audit | COMPLETE |
| Historical comparison | COMPLETE for located crypto records |
| Canonical reference | COMPLETE |
| Requirements/gaps | COMPLETE |
| Design correction | COMPLETE |
| Implementation boundary | COMPLETE; production provider not yet approved |
| Test design | COMPLETE |
| Security review | COMPLETE at architecture level |
| Executable crypto verification | BLOCKED until approved provider is integrated |
| Independent cryptographic review | REQUIRED |
| Production release | BLOCKED |

## Specialty decisions

- CRYPTO-DEC-001: Signal Protocol family is the one-to-one reference.
- CRYPTO-DEC-002: PQXDH is the session-establishment reference, subject to a frozen P2P deployment profile.
- CRYPTO-DEC-003: Double Ratchet is the message-key evolution reference.
- CRYPTO-DEC-004: MLS RFC 9420 is the group-crypto standards track.
- CRYPTO-DEC-005: No custom primitive and no bespoke ratchet.
- CRYPTO-DEC-006: libsignal remains reference/conformance material until exact production use is separately approved.
- CRYPTO-DEC-007: Platform hardware protection is preferred, but hardware residency is claimed only when the exact platform operation is proven.
- CRYPTO-DEC-008: Recovery is a separate trust domain and never a universal decryptor.
- CRYPTO-DEC-009: Transport components never receive plaintext or E2EE secret material.

## Evidence rule

No stage is PASS solely because a design file exists. Executable stages require implementation, test, interoperability, provenance, or independent-review evidence appropriate to the claim.
