# PrivateMesh — Transport Security Research 2026-10-05

Status: RESEARCH / CANDIDATE INPUT — NOT AN ADOPTED ADR

Scope: P2P / Networking / PrivateMesh only.

## Executive conclusion

For Eagle's security-first, opaque-frame architecture, the preferred candidate direction is a **minimal transport composition** rather than adopting a full networking framework as the application protocol.

Candidate A — QUIC transport + separately vetted ICE/NAT traversal + approved relay is the leading architecture candidate.

Candidate B — a tightly scoped rust-libp2p transport/NAT/relay subset remains viable, but only with strict component selection, exact-version advisory gates, and an explicit boundary preventing libp2p identity/security semantics from becoming Eagle's canonical application identity.

WebRTC full-stack is deferred unless an actual browser/media/data-channel interoperability requirement justifies its larger surface.

No candidate is adopted by this research record.

## Evidence reviewed

### QUIC / Quinn

RFC 9000 defines QUIC v1 as a UDP-based secure transport with streams, low-latency establishment and path migration. RFC 9001 defines TLS integration and explicitly warns that QUIC 0-RTT application data is replayable and therefore must not carry replay-sensitive actions.

Current crates.io evidence shows quinn 0.11.12 and quinn-proto 0.11.19 in the 0.11 line. quinn-proto 0.11.19 was published 2026-09-30.

Security advisories require continuous version discipline:
- quinn-proto 0.11.13 had a remote unauthenticated panic on malformed transport parameters; fixed in 0.11.14.
- quinn-proto versions through 0.11.16 had an unbounded pending retired-connection-ID growth issue; fixed in 0.11.17.

Therefore the implementation gate MUST inspect the complete resolved Cargo.lock, not only the direct quinn version.

### ICE / STUN / TURN

RFC 8445 defines ICE for UDP NAT traversal and is updated by RFC 8863. RFC 8489 is the current STUN specification and obsoletes RFC 5389. RFC 8656 specifies TURN relay operation.

The architecture should therefore model:
candidate gathering → validation → connectivity checks → direct path → relay fallback.

A STUN address is connectivity metadata, not peer authentication.

TURN is a transport fallback, not a trust service.

### rust-libp2p

Current rust-libp2p upstream workspace declares libp2p 0.57.0. The project provides integrated hole punching, AutoNAT, Circuit Relay v2 and DCUtR, which can reduce bespoke NAT/relay code.

However, current upstream security records include high-severity advisories affecting components such as libp2p-quic, rendezvous, and gossipsub. The correct response is not to reject libp2p categorically, but to:
- depend only on required crates/features;
- pin exact reviewed versions;
- require RustSec/GitHub advisory checks in CI;
- avoid unused attack-surface-heavy protocols;
- test all resource bounds at the Eagle boundary.

In particular, the Eagle design does not require gossipsub merely because mesh networking exists.

### WebRTC / webrtc-rs

Current webrtc-rs has a 0.20.0 release, but its project roadmap still describes a path to 1.0 and includes unresolved pre-1.0 work. The stack contains media, DTLS, SCTP, data-channel and ICE machinery.

This makes it attractive for broad WebRTC interoperability, but unnecessarily large for an Android/Rust encrypted messaging transport unless browser/data-channel interoperability is a V1 requirement.

The standalone webrtc-ice crate is a narrower alternative, but its current published documentation coverage is incomplete and it remains a component requiring independent review.

## Security ranking for Eagle

| Candidate | Security fit | Complexity | NAT/relay readiness | Eagle boundary fit | Current decision |
|---|---|---|---|---|---|
| Quinn + vetted ICE + approved relay | High potential | Medium | Requires integration | Strong | Preferred candidate |
| rust-libp2p selected subset | High potential | Medium/High | Strong built-in support | Requires strict adapter boundary | Conditional candidate |
| Full WebRTC stack | Good when WebRTC is required | High | Strong | Larger surface than needed | Deferred candidate |
| Custom UDP/TCP transport + custom NAT traversal | Low | High | Bespoke | Poor | Reject |

## Non-negotiable transport security properties

1. QUIC/transport security is never the Eagle E2EE boundary.
2. Eagle application authentication must remain independent of transport authentication.
3. QUIC 0-RTT is disabled for state-changing or replay-sensitive application operations unless an explicit replay-safe design proves otherwise.
4. Relay nodes never receive Eagle application plaintext or private keys.
5. Candidate/address metadata is treated as untrusted input and minimized.
6. Every peer, candidate, connection, stream, queue, retry, and relay allocation has a hard bound.
7. No transport fallback may silently lower protocol/security policy.
8. A path change must not change Eagle trust state.
9. All transport dependencies are locked to exact resolved versions and audited against current advisories.
10. The release build must record the resolved dependency graph and security-review date.

## Implementation gate

Before an implementation PR can select a transport:
- accepted ADR-0012;
- exact Rust/core integration point exists;
- exact crate versions and Cargo.lock are present;
- advisory scan passes;
- direct/NAT/relay failure matrix passes;
- 0-RTT policy is explicitly enforced;
- candidate/resource limits are implemented;
- fuzz/property tests cover network parsers and state transitions;
- security review and independent verification are recorded.

## Sources

- RFC 9000: https://www.rfc-editor.org/info/rfc9000/
- RFC 9001: https://www.rfc-editor.org/info/rfc9001/
- RFC 8445: https://www.rfc-editor.org/info/rfc8445/
- RFC 8863: https://www.rfc-editor.org/info/rfc8863/
- RFC 8489: https://www.rfc-editor.org/info/rfc8489/
- RFC 8656: https://www.rfc-editor.org/info/rfc8656/
- RFC 9846: https://www.rfc-editor.org/info/rfc9846/
- rust-libp2p security: https://github.com/libp2p/rust-libp2p/security
- Quinn security: https://github.com/quinn-rs/quinn/security/advisories
- Quinn crate evidence: https://docs.rs/crate/quinn/latest
