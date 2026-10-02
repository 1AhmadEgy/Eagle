# Eagle Project Handoff Record — 2026-10-02

## Purpose

This record preserves the substantive project-planning changes captured during the current coordination session so the conversation can be removed without losing the working baseline.

## Repository evidence observed

At the time this record was created:

- repository: `1AhmadEgy/Eagle`
- default branch observed through repository file links: `main`
- `SECURITY.md` exists on main and requires maintained / well-reviewed security dependencies, input validation at trust boundaries, automated tests and security checks before merge, and documentation of security assumptions.
- `README.md` exists on main.
- root-level `ARCHITECTURE.md` and `DECISION_LOG.md` were not found at those exact paths on main.
- branch `chore/canonical-planning-baseline-v1` exists and was three commits ahead of main when checked; its current changes include the planning-baseline document plus ARCH-001 and ADR-0007 issue records.

## Planning decisions captured

### Modules

core, identity, crypto, protocol, storage, mesh, ui, integration, security, observability, documentation.

### Separation of responsibility

- core: contracts / models / errors / events
- identity: device identity + key lifecycle
- crypto: sessions + encryption / decryption
- protocol: envelope + framing + serialization + versioning
- storage: persistence / repository
- mesh: discovery + transport + routing + forwarding
- ui: presentation
- integration: dependency wiring only
- security: verification / audit / gates
- observability: sanitized telemetry
- documentation: architectural / project records

### Security flow

Send:
`plaintext → crypto → encrypted envelope → protocol → mesh → transport`

Receive:
`transport → mesh → protocol → crypto → plaintext → application/UI`

### Explicit prohibitions

- mesh: no plaintext
- protocol: no application plaintext
- storage: no private keys
- ui: no private keys / crypto internals
- logs / telemetry: no plaintext / secrets

### Dependency direction

- identity → core
- crypto → core + identity contracts
- protocol → core + crypto contracts
- storage → core
- mesh → core + protocol contracts
- ui → core + application services
- integration → wiring only
- security → read / test / audit
- observability → sanitized contracts

### ADR / issue separation

ADRs decide. Issues implement or verify accepted decisions.

The serialization example is:
`ADR-0010 → PROTO-003`

## Current approval state

The planning baseline remains **proposed** until:

1. ARCH-001 is reviewed and accepted.
2. ADR-0007 is accepted.
3. ADR-0001 → ADR-0006 compatibility is explicitly checked.
4. Architecture and security reviews are recorded.
5. Repository evidence is linked.

No technical option such as Signal, X3DH, Noise, MLS, Tink, libsodium, Protobuf, or CBOR is treated as approved merely because it appears in planning notes.

## Conversation-retention rule

This repository record is intended to preserve the project decisions, assumptions, open items, and evidence needed to continue the work without depending on the deleted conversation history.
