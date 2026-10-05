# PrivateMesh — Requirements / Gap / Remediation Matrix

Status: IN REVIEW

| ID | Requirement | Current evidence | Gap | Remediation | Gate |
|---|---|---|---|---|---|
| PM-NET-001 | Opaque mesh boundary | Mesh contracts + invariants | No source enforcement | Implement typed opaque-frame boundary; add negative tests | Security |
| PM-NET-002 | Peer/candidate validation | Contract definitions | No implementation validator | Implement strict parser/validator before state mutation | Testing |
| PM-NET-003 | Discovery freshness | Failure scenarios | No runtime expiry evidence | Add bounded TTL/expiry behavior and poisoning tests | Testing |
| PM-NET-004 | NAT traversal | RFC/transport research + ADR input | No approved stack | Approve transport/ICE composition through ADR-0012 | Architecture |
| PM-NET-005 | Direct P2P preference | Path policy | No runtime selector | Implement deterministic selector + hysteresis tests | Verification |
| PM-NET-006 | Relay fallback | Relay contract/scenarios | No relay implementation | Implement opaque relay adapter after stack approval | Security |
| PM-NET-007 | No silent downgrade | Invariants + version rules | No executable enforcement | Add version/profile rejection tests | Security |
| PM-NET-008 | Bounded retries | Retry contract | No constants/runtime | Define registry-backed bounds and test backoff | DoS |
| PM-NET-009 | Bounded resource use | Resource-limit contract | Numeric limits not authoritative | Approve numeric resource envelope before code | DoS |
| PM-NET-010 | Network-change recovery | FSM + scenarios | No platform-neutral implementation | Implement event-driven recovery in approved core | Verification |
| PM-NET-011 | Route-loop control | Route contract/scenario | No forwarding implementation | Add hop bound, visited-path rule and loop tests | Security |
| PM-NET-012 | Sanitized telemetry | Event contract | No runtime evidence | Add structured events with secret/PII exclusion tests | Security |
| PM-NET-013 | Transport interoperability | Current candidate research | No PoC evidence | Execute candidate PoCs and record exact versions/results | Architecture |
| PM-NET-014 | Failure injection | 20 scenarios | Scenario definitions only | Bind scenarios to deterministic test harness | Verification |
| PM-NET-015 | Release evidence | Acceptance gate | No implementation evidence | Produce traceable test/security/review artifacts | Release |
| PM-NET-016 | 0-RTT replay safety | RFC 9001 + transport research | Policy not encoded | Disable for state-changing operations or prove replay-safe semantics | Security |
| PM-NET-017 | Dependency advisory discipline | Current transport research | No Rust/Cargo dependency graph | Add lockfile + automated advisory/SBOM checks when implementation lands | Supply chain |
| PM-NET-018 | Transport identity separation | Threat model | No implementation boundary | Keep transport identity subordinate to Eagle PeerId | Identity/Security |

## Current blocking set

The P2P track is implementation-gated by:
1. accepted transport architecture;
2. verified Rust/core integration point;
3. numeric resource envelope;
4. executable network fault harness;
5. security review of the implementation;
6. dependency/advisory evidence for the exact resolved stack.

Until these exist, the correct state is GATED rather than PASS.
