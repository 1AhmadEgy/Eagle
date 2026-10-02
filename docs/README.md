# Eagle — PrivateMesh Protocol Documentation

This documentation tree establishes the normative and non-normative documentation baseline for the PrivateMesh protocol work maintained in this repository.

## Status

- Repository: 1AhmadEgy/Eagle
- Documentation branch: docs/privatemesh-hardening-2026-10
- Protocol scope: PrivateMesh V1
- Documentation state: IN REVIEW
- Independent security audit: NOT CLAIMED

## Documentation rules

1. Normative protocol behavior MUST be specified before implementation is considered conformant.
2. A registry identifier MUST have one authoritative definition.
3. Cryptographic profiles MUST specify exact algorithms, encodings, context binding, failure behavior, and test vectors.
4. Security claims MUST be limited to the defined threat model.
5. Documentation completeness MUST NOT be treated as certification, independent audit, or production readiness.
6. Unresolved Critical/High ambiguities remain release blockers.

## Structure

docs/
├── 00-overview/
├── 01-normative/
├── 02-baseline/
├── 03-architecture/
├── 04-security/
├── 05-implementation/
├── 06-testing/
├── 07-operations/
├── 08-development/
├── 09-reference/
├── 10-vectors/
├── 11-decisions/
├── 12-audits/
└── 13-non-normative/

## Current workstream

The initial hardening pass focuses on:

- Master File Registry and provenance
- Registry collision removal
- Trust decision model normalization
- Wire-format canonicalization
- PQXDH/Double Ratchet profile precision
- Offline queue semantics
- Recovery authority boundaries
- Cryptographic deletion semantics
- Traceability from requirements to tests
