# ARCH-002 — Core Contracts

**Status:** Proposed / blocked by ADR-0007

Define stable interfaces/models/errors/events in core. No implementation logic.

Core contract families:
- IdentityProvider: public identity, fingerprint, signing, verification, revocation; never private keys.
- MessageCipher: encrypt, decrypt, establish session, rotate session.
- MeshTransport: send/receive/discover/route opaque packets.
- MessageStore: persist encrypted records without exposing private keys.

Acceptance: contracts documented, isolated from implementations, architecture checks pass, security boundary assumptions recorded.

Requires: ARCH-001 and accepted ADR-0007.