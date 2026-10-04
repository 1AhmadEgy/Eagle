# MESH-013 — Protocol / Transport Conformance
Status: SPECIFIED / IMPLEMENTATION-GATED

## Scope
Verify that Mesh preserves Protocol semantics while providing transport connectivity.

## Required tests
- valid opaque frame
- malformed frame
- size limit
- unknown version
- downgrade attempt
- duplicate frame
- delivery ordering where required
- transport failure isolation

## Acceptance
Conformance evidence is reproducible and tied to the exact implementation commit.
