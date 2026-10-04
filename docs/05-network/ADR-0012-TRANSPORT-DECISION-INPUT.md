# ADR-0012 — Transport Architecture Decision Input

Status: Decision input only. Not an accepted ADR.

## Decision question
Select the transport/connectivity composition for PrivateMesh while preserving the opaque-frame contract and P2P-first behavior.

## Candidate composition
- Application transport: QUIC or another approved reliable transport.
- NAT traversal: ICE where required.
- Candidate servers: STUN where required by the selected ICE deployment.
- Relay: TURN or another approved relay mechanism.
- WebRTC: only where its media/data-channel semantics are justified; it is not assumed to be the universal transport.

## Required properties
1. Direct P2P is preferred.
2. Relay is fallback.
3. Relay does not replace E2E.
4. Transport choice does not change Protocol/Crypto boundaries.
5. Background/mobile lifecycle must be represented as adapter behavior, not hidden in the Protocol contract.
6. Candidate gathering and connectivity checks are testable independently.
7. Failure and recovery are deterministic.
8. Interoperability and maintenance evidence must be available before adoption.

## Decision gate
Before marking a transport ADOPTED:
- official specification reviewed
- security history reviewed
- implementation maturity reviewed
- license reviewed
- interoperability PoC completed
- failure matrix passes
- resource/performance envelope measured
- security review recorded
- ADR approved

## Explicit non-decisions
This document does not approve QUIC, WebRTC, ICE, STUN, or TURN individually.
