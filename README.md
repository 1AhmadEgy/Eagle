# Eagle

Eagle is a private-communication project focused on a security-first architecture, with Android as the current application bootstrap and a planned evolution toward identity, secure messaging, protocol, storage, transport, and PrivateMesh capabilities.

## Current status

**Foundation / Evidence Collection / Stack Discovery**

The repository currently contains:

- project governance and engineering documentation;
- security and provenance policies;
- requirements/readiness/gap tracking;
- Android application bootstrap;
- test/bootstrap infrastructure;
- AI-agent development controls;
- historical-source integration records.

The executable application is **not yet a production-ready private messenger or completed PrivateMesh network**.

## Engineering principles

1. Evidence before claims.
2. Requirements before product invention.
3. Reuse mature components before writing replacements.
4. Do not implement novel cryptographic primitives when established reviewed primitives are suitable.
5. Security-sensitive changes require explicit verification.
6. Missing evidence remains **Pending**, not Pass.
7. Historical material is preserved with provenance and is not trusted automatically.

## Canonical documentation

- [Master Project Source & Integration Index](docs/MASTER_PROJECT_SOURCE_INDEX.md)
- [Execution Readiness](docs/EXECUTION-READINESS.md)
- [Implementation Readiness](docs/12-readiness/IMPLEMENTATION_READINESS.md)
- [Execution Continuation Plan](docs/13-execution/EXECUTION_CONTINUATION_PLAN.md)
- [Gap Register](docs/11-gaps/GAP_REGISTER.md)
- [Documentation Status](docs/08-status/DOCUMENTATION_STATUS.md)
- [Evidence, Architecture & Reuse Audit](docs/14-audit/EVIDENCE_ARCHITECTURE_REUSE_AUDIT_2026-10-04.md)
- [Reuse-First Technology Register](docs/14-audit/REUSE_FIRST_COMPONENT_REGISTER.md)

## First executable product slice

The recommended first end-to-end slice is:

Device A
→ Identity
→ Session/key establishment
→ Encryption
→ Protocol envelope
→ Transport
→ Device B
→ Verification
→ Decryption
→ Persistence
→ Automated evidence

Mesh discovery/routing should be added only after the basic secure messaging slice is executable and testable.

## Important status rule

Architecture documents describe intent unless the corresponding implementation and verification evidence exists in the repository.

## Security

See [SECURITY.md](SECURITY.md). Do not report Eagle as independently audited, production-secure, or fully implemented unless repository evidence supports that claim.
