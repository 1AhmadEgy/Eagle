[object Object]

## Deep-research continuation — 2026-10-05

External review confirmed:
- PQXDH's standard asynchronous model uses a server for prekey publication/fetching; strict direct-P2P deployment therefore requires a separately frozen rendezvous profile.
- libsignal v0.104.0 remains upstream-current in this review, but upstream explicitly says use outside Signal is unsupported and the workspace is AGPL-3.0-only.
- vodozemac 0.10.0 remains a useful audited Olm/Megolm candidate, not a PQXDH replacement.
- OpenMLS 0.9.0 continues toward standards-compliant MLS RFC 9420 and includes recent security/storage hardening.
- NIST's 2026 PQC migration material continues to recommend finalized standards such as ML-KEM/ML-DSA rather than draft candidates.
- Android and Apple platform evidence requires capability-specific custody claims; hardware presence alone is insufficient.

Implementation hardening completed after that review:
- provider approval now binds an explicit protocol profile;
- provider approval now requires hardware protection plus verifiable hardware-attestation capability;
- platform contracts distinguish hardware security anchors from protocol-key custody;
- iOS and desktop key-storage adapter contracts were added;
- P2P deployment and release-gate documents now explicitly block unsupported asynchronous/interoperability claims;
- CKM-027 and CKM-028 were added for attestation/profile binding.

Verification remains pending because GitHub Actions are queued behind prior runs. No PASS or release authorization is claimed.
