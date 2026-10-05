# PrivateMesh — Transport Interoperability Matrix

Status: CANDIDATE EVALUATION ONLY

## Evaluation rule

Technology is not adopted because it appears in historical planning. It must pass specification, security, maturity, licensing, interoperability, resource, and failure testing before ADR approval.

| Candidate | Role | Advantages relevant to Eagle | Risks / questions | Required evidence | Status |
|---|---|---|---|---|---|
| QUIC v1 | Reliable secure transport | Multiplexed streams, low-latency establishment, path migration | Implementation choice, 0-RTT policy, API/runtime fit | POC + loss/reorder/NAT/resource tests | CANDIDATE |
| ICE | NAT traversal framework | Standard candidate/check model using STUN/TURN | Integration complexity, lifecycle/resource policy | Connectivity matrix + attack/failure tests | CANDIDATE |
| STUN | Candidate gathering aid | Reflexive connectivity metadata | Privacy leakage and deployment assumptions | Candidate privacy review + PoC | CANDIDATE |
| TURN | Relay fallback | Works when direct path fails | Cost/availability, relay abuse, metadata exposure | Relay isolation + outage/recovery tests | CANDIDATE |
| WebRTC data channel | Optional data transport | Mature browser/mobile ecosystem | Extra stack/security semantics may be unnecessary | Requirement justification + PoC | DEFERRED CANDIDATE |

## Current standards references

- QUIC: RFC 9000.
- ICE: RFC 8445, updated by RFC 8863.
- TURN: RFC 8656.
- TLS 1.3: RFC 9846 (July 2026); RFC 8446 is obsolete. Normative adoption MUST track the current RFC.
- WebRTC transports/security: RFC 8835 / RFC 8826.

## Security decision rule

Prefer the smallest interoperable stack that:
- preserves the Mesh opaque-frame boundary;
- provides direct P2P as the preferred path;
- provides relay only as fallback;
- has bounded state/resources;
- has a maintained implementation for the approved Rust/core environment;
- supports deterministic failure and recovery tests.

No candidate is ADOPTED by this matrix alone.
