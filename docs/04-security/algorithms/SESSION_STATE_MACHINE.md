# Eagle — Session State Machine

Status: Implemented in Kotlin; build verification pending
Implementation: app/src/main/java/com/eagle/app/security/SessionStateMachine.kt

## State model

NEW -> AUTHENTICATING -> ESTABLISHED -> REKEYING -> ESTABLISHED

Any non-closed state may transition to CLOSED.
CLOSED is terminal.
Invalid transitions are rejected without mutating state.

## Security boundary

This state machine is an orchestration primitive, not an authentication mechanism.

The caller is responsible for authenticating the peer, validating negotiated cryptographic parameters, binding keys/transcript/session identity, and ensuring authorization policy permits the requested operation.

A session must not enter ESTABLISHED merely because a caller requests the transition without completing those checks.

## Replay integration

A ReplayGuard instance is scoped to one authenticated session. Its lifecycle must follow the session state. On transition to CLOSED, the owning session layer must discard the associated replay state and other session-sensitive material according to the key-management policy.

## Tests

JUnit tests cover complete valid lifecycle, invalid transition rejection without mutation, early close, repeated same-state transition, and invalid rekey before establishment.

## Current limitation

The repository does not yet contain the full authentication/key-management/session coordinator. This state machine remains a small deterministic building block until those higher-level requirements are frozen and implemented.
