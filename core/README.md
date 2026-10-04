# Eagle Core Security Boundary

The core directory is the security-critical Rust boundary for Eagle.

## Non-negotiable rules

1. Private keys never cross the Rust FFI boundary and are never exported or serialized by application code.
2. Security decisions remain in Rust. Kotlin may coordinate and present state, but it does not authorize trust or implement cryptography.
3. Every FFI contract change requires security review and corresponding contract tests.
4. unsafe code is forbidden in the core crates unless a documented, reviewed exception is introduced under the repository security process.
5. Security tests belong in Rust: state-machine, property-based, fuzz, negative-path, and conformance tests as applicable.
6. Kotlin must not re-implement cryptographic primitives. A new capability must be added to the reviewed Rust contract.
7. Generated UniFFI bindings are build artifacts. They must not become a second hand-maintained FFI implementation.
8. The current FFI methods are contract scaffolding only. They fail closed with ContractNotReady until the relevant cryptographic, key-management, and serialization ADRs are approved and implemented.

## Boundary model

Android/iOS/desktop UI
→ KMP shared domain
→ UniFFI-generated bindings
→ Rust core
→ approved security/storage/protocol implementations

The KMP commonMain layer depends on a small EagleCore port only. Generated bindings belong in platform-specific integration layers and must not become common-domain dependencies.
