# Canonical Planning Baseline v1

**Project:** Eagle  
**Version:** 1.0-proposed  
**Date:** 2026-10-02  
**Status:** Proposed — not yet approved  
**Governing ADR:** ADR-0007  
**Governing issue:** ARCH-001

## Authority
This is the proposed planning reference. It becomes canonical only after ARCH-001 and ADR-0007 are explicitly approved, compatibility with the existing ADR record is verified, and architecture/security evidence is recorded.

## Modules
1. core — contracts / models / errors / events
2. identity — device identity + key lifecycle
3. crypto — sessions + encryption/decryption
4. protocol — envelope + framing + serialization + versioning
5. storage — persistence / repository
6. mesh — discovery + transport + routing + forwarding
7. ui — presentation
8. integration — dependency wiring only
9. security — verification / audit / gates
10. observability — sanitized telemetry
11. documentation — architectural / project records

## Dependency direction
- identity → core
- crypto → core + identity contracts
- protocol → core + crypto contracts
- storage → core
- mesh → core + protocol contracts
- ui → core + application services
- integration → implementations for wiring only
- security → read / test / audit
- observability → sanitized contracts

Denied: UI→Keystore, UI→crypto internals, UI→database internals, Mesh→plaintext, Mesh→database internals, Storage→UI, Identity→UI, Crypto↔Mesh circular dependency, Storage→Protocol.

## Security flow
Send: plaintext → crypto → encrypted envelope → protocol → mesh → transport
Receive: transport → mesh → protocol → crypto → plaintext → application/UI

Required boundaries:
- mesh: no application plaintext
- protocol: no application plaintext
- storage: no private keys
- UI: no private keys / crypto implementation internals
- logs / telemetry: no plaintext / secrets

## ADR / implementation separation
ADR = decision. Issue = implementation, verification, migration, or follow-up.
Example: ADR-0010 decides serialization; PROTO-003 implements the accepted decision.

## Approval gate
- ARCH-001 accepted
- ADR-0007 accepted
- ADR-0001 → ADR-0006 compatibility verified
- independent architecture review recorded
- security review recorded
- repository evidence linked