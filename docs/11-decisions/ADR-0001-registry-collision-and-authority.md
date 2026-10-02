# ADR-0001 — Registry Collision and Authority Model

## Status

ACCEPTED FOR REVIEW

## Context

Previous PrivateMesh drafts assigned overlapping meanings within the 0x6201–0x6205 range. This creates a protocol ambiguity: two implementations can accept the same numeric identifier while applying different security semantics.

A registry must be a single source of truth for security-sensitive identifiers.

## Decision

1. PM-SPEC-011 is the authoritative registry specification.
2. 0x6201–0x6204 retain the signed-prekey, freshness, and session-idle assignments.
3. Attestation parameters move to 0x6210–0x6213.
4. Device idle timeout is assigned to 0x6214.
5. 0x6205 remains unassigned until explicitly registered.
6. Queue parameters are reserved at 0x6215–0x6217 until exact V1 values are approved.
7. No implementation may silently preserve conflicting legacy meanings.

## Consequences

### Positive

- Removes semantic collision.
- Establishes one registry authority.
- Makes implementation and test-vector generation deterministic.
- Enables automated collision checks.

### Negative

- Existing drafts or implementations using the conflicting assignments require migration.
- Compatibility cannot be assumed until vectors and implementation constants are updated.

## Required follow-up

- Update all normative specifications referencing affected IDs.
- Add registry collision CI.
- Add migration notes for implementations using old assignments.
- Generate deterministic registry artifacts from the canonical source.
- Review all remaining 0x62xx assignments.

## Security note

This ADR does not constitute a cryptographic proof or independent security audit.
