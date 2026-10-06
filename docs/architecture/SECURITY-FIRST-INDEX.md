# Security-First Documentation Index

This index is the entry point for the current accepted security/runtime baseline.

## Canonical documents

1. [ADR-0001 — Security-First Runtime Boundary](./ADR-0001-security-first-runtime-boundary.md)
2. [Security-First Baseline](../security/security-first-baseline.md)
3. [Canonical Requirements & Gap Matrix](../traceability/canonical-requirements-matrix.md)

## Source and provenance

Primary source reviewed for this baseline:

- `10. فريق Android_4085322700343523373.md`

Important historical conflicts identified in the source are retained for provenance but are not implementation authority:

- FCM/Firebase runtime path
- Google Play Services / Play Integrity runtime path
- minSdk 29 variant
- server-mediated application-content pipeline
- server Outbox/PostgreSQL content path

## Architecture gate

GitHub Issue #86 remains the reference gate for the server-side Outbox proposal:

https://github.com/1AhmadEgy/Eagle/issues/86

Current disposition: do not merge the Outbox proposal into the application baseline unless a new architecture decision explicitly authorizes a server-mediated application-data path.

## Evidence rule

Design text, generated code, branches, or examples do not prove implementation.

A requirement becomes implemented only when repository evidence includes the implementation path, automated/manual test evidence, review state, and acceptance result.
