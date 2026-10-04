# Eagle / PrivateMesh — Phone-First Execution Plan

**Scope:** Phase 1 — Android-first foundation with a platform-neutral core and KMP application layer.
**Branch:** `execution/core-foundation-v1`
**Status:** Implementation baseline; cryptographic/profile decisions remain gated.

## Team tracks

| # | Role | Phase 1 deliverable | Future reuse | AI assistance |
|---|---|---|---|---|
| 1 | Cross-platform architect | Freeze module boundaries and contracts; keep core independent from Android | iOS/web/desktop adapters | Reasoning/planning |
| 2 | Android lead | Android shell + lifecycle adapter around stable core APIs | iOS migration patterns | Kotlin/Java code generation |
| 3 | UI/UX | Trust, onboarding, session, error, and security-blocked flows | Web/desktop variants | Generative UI/UX |
| 4 | Security/crypto | Threat model, key boundary, platform key adapter contract | Cross-platform security profiles | Anomaly/vulnerability detection |
| 5 | Network/protocol | Protocol/transport contracts; no plaintext in mesh | Multi-platform transport adapters | Network optimization |
| 6 | Backend/API | Versioned API contracts, auth boundary, sync interface | iOS/web backend reuse | Code generation/query optimization |
| 7 | QA | Unit, negative, state-machine and fuzz test matrix | Cross-platform conformance | Test automation |
| 8 | DevOps/CI-CD | Build/test gates and reproducibility evidence | Multi-platform pipelines | AIOps |
| 9 | Data/performance | Resource budgets and sanitized telemetry | Cross-platform performance baselines | Data analytics |
| 10 | Cross-platform/documentation | API docs, architecture map, platform portability guide | Lead iOS/web/desktop rollout | Technical writing/code translation |

## Execution order

1. Security Kernel and state machines.
2. Protocol contracts and conformance vectors.
3. KMP application boundary and FFI contract scaffold.
4. Storage/recovery/deletion boundaries.
5. Transport adapters.
6. Android secure-storage and lifecycle adapters.
7. Android application services.
8. UI integration.
9. Cross-module, negative, fuzz and failure-injection tests.
10. Provenance/SBOM/signing evidence.
11. Independent verification and release gate.

## Hard gates

- No custom cryptography.
- No protocol/profile is considered adopted until its ADR gate is closed.
- AI/MCP output is advisory and cannot override deterministic policy.
- UI cannot access private keys or crypto internals.
- Mesh/protocol boundaries do not receive application plaintext.
- Storage does not become a private-key repository.
- A feature is not marked verified without executable evidence.
- Generated UniFFI bindings are platform-specific build artifacts, not commonMain domain code.

## Current implementation note

The Phase 1 Rust core remains intentionally cryptography-free. It implements deterministic trust/session state transitions, protocol validation, and the platform-neutral storage record/deletion/recovery contract.

The new KMP layer is currently a domain/port scaffold. The Rust FFI exposes the future contract but deliberately fails closed for operations that require the still-pending cryptographic, key-management, and serialization ADRs. Storage likewise remains a contract/test layer until ADR-0011 is accepted.

Generated bindings are not committed yet because the native library packaging targets have not been defined. The generator script is provided for reproducible binding generation once the Rust artifact is available.
