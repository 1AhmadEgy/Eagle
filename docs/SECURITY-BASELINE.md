# Eagle Security Baseline

## Principles

1. Least privilege for humans, CI jobs, services, and credentials.
2. Deny by default at trust boundaries.
3. No secrets in source, history, logs, artifacts, or test fixtures.
4. Treat external input as hostile until validated.
5. Use established, maintained cryptographic libraries; do not implement cryptographic primitives.
6. Keep security-sensitive behavior covered by automated tests.
7. Make dependency and CI supply-chain changes reviewable.
8. Record security assumptions and residual risks.

## Minimum CI gates

- Repository secret scanning.
- Dependency/security scanning when a dependency manifest exists.
- Static analysis appropriate to the selected language.
- Unit/integration tests.
- Build verification.
- Artifact provenance controls before production releases.

## Threat-model placeholder

The product-specific threat model cannot be completed until the authoritative V1 architecture and trust boundaries are available.
