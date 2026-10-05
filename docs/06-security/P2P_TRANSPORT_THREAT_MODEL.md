# Eagle P2P Transport Threat Model — 2026-10-05

## Scope

Direct peer-to-peer networking only. Application content must not be routed through relay servers.

## Adversary model

The attacker can observe, delay, replay, inject, reorder, drop, and terminate packets; spoof addresses; operate a malicious peer; exhaust connection resources; or control discovery/rendezvous metadata.

## Assets

- peer identity binding;
- session authorization state;
- message ciphertext and attachment ciphertext;
- endpoint metadata;
- protocol/version negotiation state;
- local connection resources.

## Threats and controls

| ID | Threat | Severity | Control | Status |
|---|---|---:|---|---|
| P2P-01 | Active MITM during connection | Critical | transport identity + application identity binding | Pending final integration |
| P2P-02 | Peer identity spoofing | Critical | cryptographic peer ID authentication | Candidate control |
| P2P-03 | Replay of old session traffic | High | freshness/session replay defense | Pending protocol integration |
| P2P-04 | Downgrade to weaker version | High | monotonic authenticated version policy | Baseline kernel control |
| P2P-05 | Malicious peer sends oversized input | High | strict parser/resource bounds | Baseline control |
| P2P-06 | Connection exhaustion / handshake DoS | High | quotas, backoff, bounded work | Pending transport implementation |
| P2P-07 | Discovery service becomes trust root | High | non-authoritative rendezvous contract | Architecture control |
| P2P-08 | Relay fallback violates P2P-only | Critical | relay path disabled for application data | Policy control |
| P2P-09 | Endpoint/address correlation | Medium | metadata minimization and stable opaque IDs | Pending privacy review |
| P2P-10 | Stale revoked device reconnects | Critical | revocation epoch + reauthorization | Identity model control |
| P2P-11 | Malicious peer attempts protocol confusion | High | explicit version/ALPN/protocol separation | Pending final protocol |
| P2P-12 | Background lifecycle causes unsafe reconnection | High | platform adapter state machine + reauth | Pending platform implementation |

## Security invariants

1. Network reachability never grants authorization.
2. Direct transport establishment never grants message decryption rights.
3. Relay is never an application-content fallback.
4. Reconnection must revalidate current trust/session state.
5. Unsupported versions fail closed.
6. Resource limits apply before expensive cryptographic work.
7. Discovery metadata is untrusted input.


## Executable enforcement baseline

The Rust Security Core now exposes a default transport policy that permits application data only on a direct path and requires Eagle device identity binding. Relay and server-fallback paths are rejected fail-closed at the policy boundary. This does not yet prove the concrete network implementation cannot bypass that boundary; transport integration and adversarial runtime evidence remain release-gated.
