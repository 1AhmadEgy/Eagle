# ARCH-003 — Dependency Rules

**Module:** core  
**Phase:** foundation  
**Priority:** critical  
**Status:** Proposed

## Purpose
Define and automate allowed and denied dependency directions between Eagle modules.

## Allowed
- core → no implementation-specific dependency
- identity → core
- crypto → core + identity contracts
- protocol → core + crypto contracts
- storage → core
- mesh → core + protocol contracts
- ui → core + application services
- integration → implementation wiring only
- security → read / test / audit
- observability → sanitized contracts

## Denied
- UI → Android Keystore
- UI → crypto implementation internals
- UI → database internals
- Mesh → plaintext
- Mesh → database internals
- Storage → UI
- Identity → UI
- Crypto ↔ Mesh circular dependency
- Storage → Protocol

## Enforcement
Use repository-supported module/package architecture tests and CI. The exact enforcement technology remains a decision under ADR-0013. String-search heuristics are not the primary security boundary.

## Acceptance criteria
- [ ] dependency rules documented
- [ ] automated CI checks exist
- [ ] intentional violations fail the gate
- [ ] rules are maintainable and testable
- [ ] security-sensitive boundaries have explicit tests

## Dependencies
Requires: ARCH-001, ADR-0007 acceptance.