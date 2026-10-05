# PrivateMesh — Transport Interoperability Matrix

Status: CANDIDATE EVALUATION ONLY

## Evaluation rule

Technology is not adopted because it appears in historical planning. It must pass specification, security, maturity, licensing, interoperability, resource, and failure testing before ADR approval.

| Candidate | Role | Current evidence | Security risks / questions | Required evidence | Status |
|---|---|---|---|---|---|
| QUIC v1 via Quinn | Reliable secure transport | Quinn is a pure-Rust QUIC implementation; current published docs show quinn 0.11.12 and quinn-proto 0.11.19 | Recent remote DoS advisories in quinn-proto require exact resolved-version pinning and advisory checks; 0-RTT replay policy must be explicit | POC + loss/reorder/NAT/resource tests + cargo audit + exact lockfile | **PREFERRED CANDIDATE** |
| ICE | NAT traversal framework | RFC 8445, updated by RFC 8863 | Candidate abuse, state explosion, timing/race conditions, metadata leakage | Connectivity matrix + attack/failure tests + bounds | CANDIDATE |
| STUN | Candidate gathering aid | RFC 8489 is current and obsoletes RFC 5389 | Public-address disclosure, server trust assumptions, rate/TTL policy | Candidate privacy review + PoC + bounded requests | CANDIDATE |
| TURN | Relay fallback | RFC 8656; designed for ICE use | Relay abuse, allocation exhaustion, metadata exposure, availability/cost | Relay isolation + rate/lease bounds + outage/recovery tests | CANDIDATE |
| rust-libp2p selected subset | Networking/NAT/relay substrate | Current upstream workspace declares libp2p 0.57.0 and provides AutoNAT, Circuit Relay v2 and DCUtR | Recent security advisories; broad feature surface can add unnecessary attack surface; transport identity must not become Eagle canonical identity | Feature-minimized PoC + advisory scan + boundary tests | CONDITIONAL |
| WebRTC data channel | Optional data transport | webrtc-rs 0.20.0 is current published release in reviewed evidence | Larger stack (ICE/DTLS/SCTP/data channels), pre-1.0 roadmap and extra security surface | Requirement justification + PoC + lifecycle/resource tests | DEFERRED |

## Candidate ranking

For the current Eagle threat model and P2P scope:

1. QUIC v1 + separately vetted ICE + approved relay: preferred.
2. Minimal rust-libp2p subset: conditional alternative when its integrated NAT/relay value materially reduces risk and complexity.
3. Full WebRTC stack: only with a concrete interoperability requirement.
4. Custom transport/NAT traversal: rejected.

The ranking is not an adoption decision.

## Current standards references

- QUIC: RFC 9000.
- QUIC TLS integration: RFC 9001.
- QUIC loss/congestion: RFC 9002.
- ICE: RFC 8445, updated by RFC 8863.
- STUN: RFC 8489.
- TURN: RFC 8656.
- TLS 1.3: RFC 9846; this obsoletes RFC 8446.
- WebRTC transport/security references require separate applicability review.

## Security decision rule

Prefer the smallest interoperable stack that:
- preserves the Mesh opaque-frame boundary;
- provides direct P2P as the preferred path;
- provides relay only as fallback;
- has bounded state/resources;
- has a maintained implementation for the approved Rust/core environment;
- supports deterministic failure and recovery tests;
- has a current advisory status recorded at the exact resolved version.

No candidate is ADOPTED by this matrix alone.
