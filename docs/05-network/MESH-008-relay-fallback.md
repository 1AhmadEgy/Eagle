# MESH-008 — Relay Fallback
Status: SPECIFIED / IMPLEMENTATION-GATED

## Principle
Relay is a connectivity fallback, never an E2E replacement.

## Rules
- Relay carries opaque/encrypted frames only.
- Relay cannot terminate application E2E encryption.
- Direct recovery is attempted when policy allows.
- Relay failure is isolated from identity and cryptographic state.

## Acceptance
Relay outage, forced relay, relay recovery, and downgrade-attempt tests pass before adoption.
