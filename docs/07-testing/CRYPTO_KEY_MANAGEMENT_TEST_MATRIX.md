# Eagle — Cryptography / Key Management Test Matrix

| ID | Area | Required evidence |
|---|---|---|
| CKM-001 | Key-domain separation | identity/session/message/storage/recovery handles cannot be interchanged |
| CKM-002 | Provider fail-closed | unavailable provider never emits key material or plaintext |
| CKM-003 | Identity lifecycle | generate/register/revoke/replace semantics are monotonic |
| CKM-004 | Prekeys | signed, one-time and PQ prekey consumption semantics match adopted protocol |
| CKM-005 | PQXDH | official/approved conformance vectors + negative cases |
| CKM-006 | Double Ratchet | send/receive, skipped keys, loss, reorder, duplicate and replay cases |
| CKM-007 | Rollback | stale session state cannot overwrite newer cryptographic state |
| CKM-008 | Crash/power loss | transactional key-state transitions after interruption |
| CKM-009 | Restore/migration | restore never silently recreates trust |
| CKM-010 | Android | Keystore security level, invalidation, backup/restore behavior |
| CKM-011 | StrongBox | capability and fallback behavior where supported |
| CKM-012 | Apple | Keychain accessibility, Secure Enclave capability matrix |
| CKM-013 | Desktop | native protected storage and lower-assurance fallback labels |
| CKM-014 | Secret leakage | logs, telemetry, crash reports, temporary files, caches |
| CKM-015 | Fuzzing | parsers, state machines, envelopes, protocol negotiation |
| CKM-016 | Dependency | exact-version/SBOM/license/advisory/provenance controls |
| CKM-017 | Independent review | cryptographic and key-management review findings closed |
| CKM-018 | Release | G0–G9 gate all PASS |

**Rule:** a design document or passing unit test does not substitute for interoperability or independent cryptographic review.


## Key-lifecycle extensions

| ID | Area | Required evidence |
|---|---|---|
| CKM-019 | Generation monotonicity | replacement generations never decrease |
| CKM-020 | Epoch monotonicity | lifecycle epochs never repeat or move backward |
| CKM-021 | Atomic rotation | failed rotation leaves old key unchanged and no new active record |
| CKM-022 | One-Time PreKey | successful consumption is single-use and permanently non-active |
| CKM-023 | Provider approval | unapproved provider cannot reach production crypto path |
| CKM-024 | Capacity exhaustion | metadata exhaustion fails closed without eviction |

| CKM-025 | Provider capability gate | approved provider must prove non-exportable identity keys, PQ KEM, and message-ratchet capability |
| CKM-026 | Epoch upper-bound safety | multi-epoch allocation near `u64::MAX` fails closed without overflow or state mutation |

| CKM-027 | Hardware attestation gate | provider approval fails closed without verifiable hardware-backed attestation |
| CKM-028 | Protocol profile binding | provider approval fails closed when the exact Signal/MLS protocol profile is not bound |
