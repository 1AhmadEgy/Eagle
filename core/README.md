# Eagle Rust Core

Platform-independent Security Kernel boundary.

Implemented:
- fail-closed trust and session state machines;
- explicit device trust lifecycle;
- capability authorization;
- bounded protocol negotiation and downgrade rejection;
- structural encrypted-envelope validation;
- unsafe Rust forbidden;
- zero external runtime dependencies.

Deferred by security gate:
- cryptographic primitives;
- concrete Signal, PQXDH or MLS adoption;
- key hierarchy and secure storage;
- canonical serialization;
- transport;
- platform keystore integration;
- FFI operations capable of changing trust.

Trust elevation remains internal until an approved authentication and cryptographic implementation exists.
