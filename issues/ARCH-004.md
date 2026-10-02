# ARCH-004 — Architecture Tests

**Module:** core  
**Phase:** foundation  
**Priority:** critical  
**Type:** test  
**Status:** Proposed

## Purpose
Create automated tests for dependency direction and security boundaries.

## Required coverage
- Dependency isolation and cycle detection
- Plaintext boundaries for Mesh and Protocol
- Private-key exposure boundaries
- Contract compliance
- Secrets scanning / unsafe logging controls

## Acceptance criteria
- [ ] architecture tests run in CI
- [ ] intentional violation tests prove the gate can fail
- [ ] all baseline boundaries are covered
- [ ] false-positive handling is documented
- [ ] test evidence is retained with CI results

## Dependencies
Requires: ARCH-001, ADR-0007 acceptance, ARCH-002, ARCH-003.