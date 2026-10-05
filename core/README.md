# Eagle Rust Core

Platform-independent, fail-closed security foundation.

## Implemented in this branch

- fail-closed trust and session state machines;
- explicit device trust lifecycle;
- capability authorization;
- bounded protocol negotiation and downgrade rejection;
- immutable validated encrypted-envelope representations;
- typed cryptographic/key-purpose boundaries without custom primitives;
- deterministic key lifecycle with rotation, revocation, destruction and one-time prekey consumption;
- identity/device trust, pairing, reverification and trust-epoch controls;
- opaque encrypted-record storage contract with explicit recovery states;
- bounded replay/freshness and inbound delivery guards;
- opaque P2P transport contract and hop-limit enforcement;
- Android Keystore storage adapter with scoped alias hardening;
- static architecture security gate;
- unsafe Rust forbidden where this core is implemented;
- zero external runtime dependencies in the Rust core workspace.

## Intentionally gated

The following remain release-blocked until their approval/evidence gates close:

- production cryptographic primitive/provider integration;
- exact Signal/PQXDH/Double-Ratchet implementation/provider selection;
- canonical serialization/wire encoding;
- real direct P2P transport implementation;
- production storage backend/recovery implementation;
- UniFFI security ABI and cross-platform binding evidence;
- independent cryptographic and security review;
- adversarial interoperability/fuzz evidence.

## Security invariant

No component may treat transport reachability as trust, access private key material outside the security boundary, or receive application plaintext merely because it handles protocol/transport records.

Trust elevation remains constrained to an approved authentication/cryptographic verification path.

The architecture, requirements/gap matrix, reconciliation, and release gates are recorded under `docs/03-architecture/`.
