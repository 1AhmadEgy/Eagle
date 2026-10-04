# Protocol Conformance Matrix — 2026-10-05

| Control | Required evidence | Current status |
|---|---|---|
| Protocol profile | accepted protocol ADR + exact profile | OPEN |
| Version negotiation | authenticated/transcript-bound vectors | OPEN |
| Downgrade resistance | negative tests + no state mutation | PARTIAL |
| Capability negotiation | authenticated capability transcript | OPEN |
| Session binding | identity + trust epoch + transcript | OPEN |
| Replay protection | duplicate/replay/sequence vectors | OPEN |
| Ordering | bounded out-of-order/skipped-key tests | OPEN |
| Expiry | freshness policy + deterministic tests | OPEN |
| Framing | exact length/size/resource limits | PARTIAL |
| Serialization | deterministic encoding profile + vectors | OPEN |
| Fragmentation | authenticated fragment contract | OPEN |
| Crypto integration | approved Signal/MLS implementation | OPEN |
| Group protocol | MLS decision + exact implementation | OPEN |
| P2P transport | direct-connectivity conformance | OPEN |
| Android/iOS/Desktop | cross-platform interoperability | OPEN |
| Fuzzing | parser fuzz corpus + clean runs | OPEN |
| Supply chain | pinned revisions + provenance + advisories | OPEN |
| Independent review | protocol/security review | OPEN |

PARTIAL means structural implementation exists but is insufficient for production protocol approval.