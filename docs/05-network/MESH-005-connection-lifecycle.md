# MESH-005 — Connection Lifecycle
Status: SPECIFIED / IMPLEMENTATION-GATED

## State machine
DISCONNECTED -> DISCOVERING -> GATHERING -> CHECKING -> CONNECTING -> CONNECTED -> DEGRADED -> RECONNECTING -> DISCONNECTED

FAILED is terminal for one attempt and returns to a bounded recovery policy.

## Invariants
- Invalid transitions are rejected.
- Cancellation is idempotent.
- Timers are bounded.
- Duplicate peer sessions are deterministically reconciled.

## Acceptance
State-machine, timeout, cancellation, simultaneous-connect, and restart tests exist.
