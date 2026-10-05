# ADR-0012 — Transport Architecture Decision Input

Status: Decision input only. Not an accepted ADR.

## Decision question
Select the transport/connectivity composition for PrivateMesh while preserving the opaque-frame contract and P2P-first behavior.

## Candidate shortlist

### Candidate A — minimal QUIC composition
- Reliable transport: QUIC v1.
- Rust implementation candidate: Quinn.
- NAT traversal: separately vetted ICE implementation.
- Candidate gathering: STUN only where required by the selected ICE deployment.
- Relay: TURN or another approved opaque relay.
- Eagle application identity/authentication remains above transport.

**Current ranking: preferred candidate, not adopted.**

### Candidate B — selected rust-libp2p subset
- QUIC transport where appropriate.
- AutoNAT / DCUtR / Circuit Relay v2 where demonstrably useful.
- Only required protocols/features enabled.
- libp2p transport identity remains subordinate to the Eagle identity contract.

**Current ranking: conditional alternative, not adopted.**

### Candidate C — WebRTC full stack
- Consider only when browser/data-channel interoperability is a V1 requirement.
- Not the default messaging transport.

**Current ranking: deferred.**

## Required security properties

1. Direct P2P is preferred.
2. Relay is fallback only.
3. Relay does not replace E2E.
4. Transport choice does not change Protocol/Crypto boundaries.
5. Eagle application identity is independent of transport identity.
6. QUIC 0-RTT MUST NOT carry replay-sensitive state-changing operations unless replay safety is explicitly proven.
7. Background/mobile lifecycle must be represented as adapter behavior, not hidden in the Protocol contract.
8. Candidate gathering and connectivity checks are independently testable.
9. Failure and recovery are deterministic and bounded.
10. Interoperability and maintenance evidence is required before adoption.

## Decision gate

Before marking a transport ADOPTED:
- official specification reviewed;
- security history reviewed;
- implementation maturity reviewed;
- license reviewed;
- exact released versions recorded;
- dependency graph/Cargo.lock reproduced;
- advisory scan passes for the resolved graph;
- interoperability PoC completed;
- failure matrix passes;
- resource/performance envelope measured;
- 0-RTT behavior tested;
- security review recorded;
- ADR approved.

## Explicit non-decisions

This document does not approve QUIC, Quinn, WebRTC, ICE, STUN, TURN, or rust-libp2p individually.
