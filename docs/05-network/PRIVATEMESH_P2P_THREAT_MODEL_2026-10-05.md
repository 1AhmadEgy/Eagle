# PrivateMesh — P2P Threat Model

Status: SECURITY BASELINE / IMPLEMENTATION-GATED

Scope: discovery, candidates, NAT traversal, connection lifecycle, direct transport, relay, routing/forwarding, retry/recovery and network telemetry.

## Assets

- Peer identity binding at the Mesh boundary.
- Connection state integrity.
- Opaque encrypted envelopes in transit.
- Availability of the device/network service.
- Minimal network metadata.
- Resource quotas and battery/data budget.

## Adversaries

- Remote unauthenticated Internet peer.
- Authenticated-but-malicious peer.
- Malicious discovery/rendezvous source.
- Malicious relay.
- On-path packet observer/modifier.
- Local network attacker able to spoof or perturb connectivity metadata.
- Resource-exhaustion attacker able to create many candidates/connections/frames.

## Threats and required controls

| Threat | Example | Required control |
|---|---|---|
| Discovery poisoning | attacker advertises a false endpoint for a valid PeerId | authenticate the identity binding outside Mesh; validate freshness/source; quarantine failures |
| Candidate explosion | peer sends thousands of endpoints | hard candidate-count and byte limits before allocation |
| Address rebinding confusion | NAT mapping changes during an active session | re-candidate and re-check; never equate a new endpoint with a new trusted identity |
| Path confusion | attacker induces switch to an unintended endpoint/path | bind connection attempts to expected PeerId/context and protocol profile |
| Downgrade | direct path fails and weaker protocol is selected | fallback may change transport path only; never lower protocol/security policy |
| Relay trust escalation | relay claims it can authenticate/decrypt content | relay receives only opaque frames; application authentication stays endpoint-to-endpoint |
| Relay exhaustion | many expensive relay reservations | quotas, leases, rate limits, expiry and per-peer/global caps |
| QUIC/parser DoS | malformed frames or transport parameters | use maintained implementation; reject before unbounded allocation; fuzz parser paths |
| Retry storm | interface flaps or sleep/wake cycles trigger reconnect loops | bounded exponential backoff, jitter policy, circuit-breaking/hysteresis |
| State-machine race | simultaneous connect/reconnect attempts diverge | deterministic connection key and convergence rules; transition validation |
| 0-RTT replay | replayed early data triggers an action | disable for state-changing operations or prove replay-safe handling |
| Duplicate/reorder | network duplicates or reorders frames | transport handles ordering/reliability; application replay/ack semantics stay above Mesh |
| Route loop | malicious peer forwards traffic repeatedly | hard hop bound, visited-path tracking or equivalent loop prevention |
| Metadata leakage | telemetry logs IP/candidate/payload details | minimize and sanitize telemetry; test forbidden fields |
| Resource amplification | one remote peer multiplies sockets/tasks/timers | per-peer and global quotas; release resources on timeout/cancel |

## Security invariants

- Mesh never decrypts application payloads.
- Network-derived metadata is untrusted until validated.
- Transport authentication is not the canonical Eagle identity.
- Path migration cannot mutate trust state.
- Relay cannot become a cryptographic or authorization authority.
- Every attacker-controlled collection and timer is bounded.
- Failure must be fail-closed with deterministic cleanup.

## Verification requirements

Each threat above requires at least one negative or adversarial test before release. Critical availability controls also require repeatable failure-injection evidence and resource-bound measurements.
