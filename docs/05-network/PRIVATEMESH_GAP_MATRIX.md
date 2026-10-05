# PrivateMesh — Requirements / Gap / Remediation Matrix

Status: IN REVIEW

| ID | Requirement | Current evidence | Gap | Remediation | Gate |
|---|---|---|---|---|---|
| PM-NET-001 | Opaque mesh boundary | Mesh contracts + invariants | No source enforcement | Implement typed opaque-frame boundary; add negative tests | Security |
| PM-NET-002 | Peer/candidate validation | Contract definitions | No implementation validator | Implement strict parser/validator before state mutation | Testing |
| PM-NET-003 | Discovery freshness | Failure scenarios | No runtime expiry evidence | Add bounded TTL/expiry behavior and poisoning tests | Testing |
| PM-NET-004 | NAT traversal | Decision input | No approved stack | Approve transport/ICE composition through ADR-0012 | Architecture |
| PM-NET-005 | Direct P2P preference | Path policy | No runtime selector | Implement deterministic selector + hysteresis tests | Verification |
| PM-NET-006 | Relay fallback | Relay contract/scenarios | No relay implementation | Implement opaque relay adapter after stack approval | Security |
| PM-NET-007 | No silent downgrade | Invariants + version rules | No executable enforcement | Add version/profile rejection tests | Security |
| PM-NET-008 | Bounded retries | Retry contract | No constants/runtime | Define registry-backed bounds and test backoff | DoS |
| PM-NET-009 | Bounded resource use | Contract requirement | Limits not yet authoritative | Approve numeric resource envelope before code | DoS |
| PM-NET-010 | Network-change recovery | FSM + scenarios | No platform-neutral implementation | Implement event-driven recovery in approved core | Verification |
| PM-NET-011 | Route-loop control | Route contract/scenario | No forwarding implementation | Add hop bound, visited-path rule and loop tests | Security |
| PM-NET-012 | Sanitized telemetry | Event contract | No runtime evidence | Add structured events with secret/PII exclusion tests | Security |
| PM-NET-013 | Transport interoperability | Candidate matrix required | No PoC evidence | Execute candidate PoCs and record versions/results | Architecture |
| PM-NET-014 | Failure injection | 20 scenarios | Scenario definitions only | Bind scenarios to deterministic test harness | Verification |
| PM-NET-015 | Release evidence | Acceptance gate | No implementation evidence | Produce traceable test/security/review artifacts | Release |

## Current blocking set

The P2P track is implementation-gated by:
1. accepted transport architecture;
2. verified Rust/core integration point;
3. numeric resource envelope;
4. executable network fault harness;
5. security review of the implementation.

Until these exist, the correct state is GATED rather than PASS.
