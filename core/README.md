# Eagle Rust Core

Platform-independent Security Kernel boundary.

Implemented in this specialization:
- fail-closed trust and session state machines;
- explicit device trust lifecycle;
- capability authorization;
- bounded protocol negotiation and downgrade rejection;
- validated encrypted-envelope construction;
- canonical bounded envelope serialization;
- direct-only application-data transport policy requiring Eagle device binding;
- immutable identifier/session representations;
- unsafe Rust forbidden;
- zero external runtime dependencies;
- device-scoped, non-exportable key-reference/custody contract with terminal revocation.

Deferred by security gate:
- cryptographic primitives and primitive selection;
- concrete Signal/PQXDH/MLS adoption;
- concrete platform key-store implementation and secure storage;
- cryptographic key generation, signing/agreement, and protocol integration;
- concrete cross-platform canonical wire integration;
- concrete libp2p/QUIC transport implementation and NAT traversal;
- platform keystore integration;
- UniFFI security ABI.

Trust elevation remains internal until an approved authentication and cryptographic verification path exists.

The authoritative specialization contract and threat model are in `docs/06-security/`.
