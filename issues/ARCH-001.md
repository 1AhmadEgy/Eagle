# ARCH-001 — Architecture & Task Baseline v1

**Module:** core  
**Phase:** foundation  
**Priority:** critical  
**Type:** feature  
**Milestone:** v0.1 — Foundation

## Purpose

Establish the proposed Architecture & Task Baseline v1 as the repository reference, after reconciling it with the repository's existing documentation and ADR-0001 through ADR-0006.

This issue does **not** choose cryptographic algorithms, serialization formats, storage engines, or transport implementations. Those decisions remain separate ADRs.

## Canonical modules

| Module | Responsibility |
|---|---|
| core | contracts / models / errors / events |
| identity | device identity + key lifecycle |
| crypto | sessions + encryption/decryption |
| protocol | envelope + framing + serialization + versioning |
| storage | persistence / repository |
| mesh | discovery + transport + routing + forwarding |
| ui | presentation |
| integration | dependency wiring only |
| security | verification / audit / gates |
| observability | sanitized telemetry |
| documentation | architectural / project records |

## Dependency rules

### Allowed

- core → no implementation-specific dependency
- identity → core
- crypto → core + identity contracts
- protocol → core + crypto contracts
- storage → core
- mesh → core + protocol contracts
- ui → core + application services
- integration → implementations for wiring only
- security → read / test / audit
- observability → sanitized contracts

### Denied

- UI → Keystore
- UI → Crypto implementation
- UI → Database internals
- Mesh → plaintext
- Mesh → Database internals
- Storage → UI
- Identity → UI
- Crypto ↔ Mesh circular dependency
- Storage → Protocol

**Important:** storage persists opaque encrypted records/session state through contracts; it does not become a protocol or cryptographic implementation dependency.

## Security data-flow invariant

Send:

`plaintext → crypto → encrypted envelope → protocol → mesh → transport`

Receive:

`transport → mesh → protocol → crypto → plaintext → application/UI`

The following boundaries are mandatory:

- mesh must not receive plaintext
- protocol must operate on encrypted envelopes/framing and must not receive application plaintext
- storage must not expose private keys
- UI must not access private keys or crypto implementation internals
- logs/telemetry must not contain plaintext or secrets

## ADR / implementation separation

- ADR = architectural/technical decision.
- Issue = implementation or verification of that decision.

Example:

- ADR-0010 = serialization decision.
- PROTO-003 = implementation of the accepted serialization decision.

## Acceptance criteria

- [ ] Existing ADR-0001 → ADR-0006 reviewed against this baseline.
- [ ] No unresolved contradiction is hidden by this baseline.
- [ ] ARCHITECTURE.md updated.
- [ ] DEPENDENCY_GRAPH.md created.
- [ ] MASTER_TASK_MAP.md created.
- [ ] Canonical planning baseline document created with approval state explicit.
- [ ] DECISION_LOG.md updated only after ADR-0007 is accepted.
- [ ] Independent architecture review completed.
- [ ] Security review of boundary rules completed.
- [ ] Evidence is linked to repository files/PRs.

## Dependencies

**Requires:** existing repository baseline and ADR-0001 → ADR-0006 review.  
**Blocks:** ADR-0007 acceptance and subsequent foundation issues.

## Definition of Done

- [ ] Documentation / implementation changes complete
- [ ] Architecture check
- [ ] Threat-model impact review
- [ ] Secrets scan
- [ ] Dependency/supply-chain check where applicable
- [ ] Documentation updated
- [ ] Code/review evidence recorded
