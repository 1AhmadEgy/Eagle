# Rust Core / Security Kernel

Status: executable non-cryptographic baseline
Date: 2026-10-05

Scope is limited to Rust Core.

Security invariants:
- unknown and pending trust cannot authorize;
- public callers cannot self-promote trust;
- rejected protocol input causes no state mutation;
- unsupported versions fail closed;
- revoked and replaced devices cannot authorize;
- administrative capability is denied;
- unsafe Rust is forbidden.

No custom cryptographic primitive or concrete E2E protocol implementation is introduced. Those remain gated by approved decisions and specifications.

FFI rule:
foreign bindings may not expose trust elevation, raw key material or policy bypasses.

Dependency rule:
new security dependencies require problem statement, candidate evaluation, official-source review, maintenance/license review, PoC evidence, ADR, tests and audit evidence.
