# PrivateMesh — Evidence Index

Status: GATED

This index prevents a specification-only record from being mistaken for implementation evidence.

| Evidence type | Required artifact | Current state |
|---|---|---|
| Architecture | Accepted ADR-0012 | Pending |
| Core contract | Stable MeshTransport/opaque-frame contract | Proposed |
| Transport research | Current candidate/security research | EVIDENCED |
| Implementation | Rust/core source linked to contract | Not verified in current branch |
| Unit tests | Parser/state/path tests | Not verified |
| Integration tests | Direct/relay/NAT harness | Not verified |
| Failure injection | NET-001..020 executable results | Scenario definitions only |
| Security review | Checklist + reviewer evidence | Pending |
| Interoperability | Approved candidate/versions + PoC | Pending |
| Resource envelope | Numeric limits + measurements | Pending |
| 0-RTT policy | Enforced transport/application policy | Pending |
| Dependency security | Exact lockfile + advisory evidence | Pending |
| Independent verification | Review record | Pending |
| Release evidence | Signed/tagged evidence bundle | Pending |

## Evidence rule

A status may advance only when the artifact named in the evidence column exists, is reproducible, and is traceable to a commit/version. Documentation alone cannot satisfy implementation, testing, security review, or verification gates.
