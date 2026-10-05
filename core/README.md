# Eagle Rust Core

Platform-independent Security Kernel boundary.

Implemented in this specialization:
- fail-closed trust and session state machines;
- explicit device trust lifecycle;
- capability authorization;
- bounded protocol negotiation and downgrade rejection;
- bounded structural frame decoding with truncation/length checks;
- validated encrypted-envelope construction;
- immutable identifier/session representations;
- unsafe Rust forbidden;
- zero external runtime dependencies.

Deferred by security gate:
- cryptographic primitives and primitive selection;
- concrete Signal/PQXDH/MLS adoption;
- key hierarchy and secure storage;
- canonical serialization/wire encoding;
- transport;
- platform keystore integration;
- UniFFI security ABI;
- authenticated replay/sequence semantics (test-only replay seams exist for invariant validation).

Trust elevation remains internal until an approved authentication and cryptographic verification path exists.

Device-bound session transitions revalidate current device authority at authorization, protocol negotiation, establishment, and rekey boundaries. `trust_state()` is a derived context state; it is not an authorization decision.

The authoritative specialization contract and threat model are in `docs/06-security/`.
