# ARCH-002 — Core Contracts

**Module:** core  
**Phase:** foundation  
**Priority:** critical  
**Status:** Proposed / blocked by ADR-0007 acceptance

## Purpose
Define stable contracts, models, errors, and events shared by implementation modules. `core` contains no security or platform implementation.

## Contracts
- IdentityProvider: public identity, fingerprint, signing, verification, revocation; never exposes private keys.
- MessageCipher: encrypt, decrypt, establish session, rotate session; concrete algorithms are governed by an accepted ADR.
- MeshTransport: send/receive/discover/route opaque packets; Mesh must not decrypt them.
- MessageStore: persist and retrieve encrypted records; never expose private keys or session secrets.

## Acceptance criteria
- [ ] core is isolated from implementation modules
- [ ] contracts are stable and documented
- [ ] shared models are immutable
- [ ] private-key exposure is impossible through public contracts
- [ ] architecture and security checks pass

## Dependencies
Requires: ARCH-001, ADR-0007 acceptance.
Blocks: implementation tasks that consume the contracts.

## Note
These are planning contracts; they may change during contract review before implementation.