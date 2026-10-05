# Protocol Conformance Matrix — 2026-10-05

| Control | Required evidence | Current status |
|---|---|---|
| Protocol profile | accepted protocol ADR + exact profile | OPEN |
| Version negotiation | authenticated/transcript-bound vectors | PARTIAL (structural guard only) |
| Downgrade resistance | negative tests + no state mutation | PARTIAL |
| Capability negotiation | authenticated capability transcript | OPEN |
| Session binding | identity + trust epoch + transcript | OPEN |
| Replay protection | duplicate/sequence/content-binding tests | IMPLEMENTED / PENDING CI |
| Ordering | bounded out-of-order/skipped-key tests | IMPLEMENTED / PENDING CI |
| Expiry | freshness policy + deterministic tests | IMPLEMENTED / PENDING CI |
| Framing | exact length/size/resource limits | PARTIAL |
| Serialization | deterministic encoding profile + vectors | OPEN |
| Fragmentation | authenticated fragment contract | OPEN |
| Crypto integration | approved Signal-family session | OPEN |
| Group protocol | MLS implementation decision + exact revision | OPEN |
| P2P transport | direct-connectivity conformance | OPEN |
| Android/iOS/Desktop | cross-platform interoperability | OPEN |
| Fuzzing | unauthenticated parser fuzz corpus + clean runs | OPEN |
| Supply chain | pinned revisions + provenance + advisories | OPEN |
| Independent review | protocol/security review | OPEN |

Security note: ContentBinding is deliberately opaque. Protocol Core does not implement a cryptographic hash; an approved cryptographic/session layer must supply the binding value.

Release status: NOT APPROVED / PRODUCTION BLOCKED.