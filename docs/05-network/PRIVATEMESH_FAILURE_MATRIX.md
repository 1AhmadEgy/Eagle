# PrivateMesh — Failure Injection and Network Test Matrix

Status: Proposed test baseline.

| ID | Fault | Expected result |
|---|---|---|
| NET-001 | Peer disappears during discovery | discovery expires; no stale active route |
| NET-002 | Candidate set empty | connection fails safely; bounded retry |
| NET-003 | NAT mapping changes | re-gather/reconnect; preserve protocol semantics |
| NET-004 | Direct path fails after CONNECTED | DEGRADED → RELAY/RECONNECTING according to policy |
| NET-005 | Relay unavailable | bounded retry; no plaintext fallback |
| NET-006 | Relay recovers | prefer direct path after healthy connectivity check |
| NET-007 | Packet loss burst | retransmission/transport recovery without duplicate application delivery |
| NET-008 | Reordering | transport/protocol boundaries preserve required ordering guarantees |
| NET-009 | Duplicate frame | idempotent duplicate handling |
| NET-010 | Malformed frame | reject before state mutation |
| NET-011 | Oversized frame | reject before unbounded allocation |
| NET-012 | Unknown protocol version | safe rejection; no silent downgrade |
| NET-013 | Connection timeout | transition to bounded recovery state |
| NET-014 | Rapid network interface changes | deterministic reconnect and candidate refresh |
| NET-015 | Sleep/wake | resume through lifecycle policy; no trust escalation |
| NET-016 | Simultaneous connections | deterministic deduplication of peer sessions |
| NET-017 | Route loop | detect and terminate route before unbounded forwarding |
| NET-018 | Relay path downgrade attempt | reject unauthorized path policy change |
| NET-019 | Discovery poisoning | validate and quarantine untrusted record |
| NET-020 | Peer authentication failure | connection/session rejected; no plaintext exposure |

## Required test classes
- unit
- state machine
- protocol/transport contract
- integration
- failure injection
- property/fuzz where parsers/state transitions justify it
- regression
- security negative tests

## Evidence per test
TEST-ID, commit, environment, stimulus, expected behavior, observed behavior, artifact/hash, reviewer, final status.
