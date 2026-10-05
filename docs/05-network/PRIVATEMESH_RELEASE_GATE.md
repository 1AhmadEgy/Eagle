# PrivateMesh — Release Gate

Status: NO-GO

Scope: P2P / Networking / PrivateMesh only.

## Mandatory gates

| Gate | Required evidence | Current |
|---|---|---|
| Architecture | Accepted ADR-0012 | BLOCKED |
| Stack | Approved transport + exact versions | BLOCKED |
| Core | Verified transport integration point | BLOCKED |
| Boundary | Opaque-frame enforcement in source | BLOCKED |
| Connectivity | Direct/NAT traversal implementation | BLOCKED |
| Relay | Opaque relay adapter + recovery | BLOCKED |
| Abuse resistance | Numeric resource limits enforced before allocation | BLOCKED |
| Tests | NET-001..020 executable against implementation | BLOCKED |
| Security | Security review + negative tests | BLOCKED |
| Verification | Independent evidence | BLOCKED |
| CI | Passing relevant verification checks | BLOCKED / no status evidence |
| Branch | Clean integration lineage | BLOCKED |

## Release prohibition

Do NOT:
- mark MESH-* VERIFIED;
- publish a transport as adopted;
- create a release/tag for PrivateMesh;
- claim direct P2P, NAT traversal, or relay support is implemented;
- relax the opaque-frame or fail-closed boundary to unblock progress.

## Current disposition

Documentation and security controls are committed on the working branch. The release decision remains NO-GO until implementation evidence satisfies every mandatory gate.

## Security principle

A missing capability is safer than an unverified capability. PrivateMesh must fail closed at every unresolved boundary rather than infer transport, trust, or compatibility behavior.
