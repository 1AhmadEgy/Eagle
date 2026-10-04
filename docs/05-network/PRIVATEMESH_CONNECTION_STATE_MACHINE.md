# PrivateMesh — Connection State Machine

Status: Proposed / implementation-gated.

## States
DISCONNECTED
DISCOVERING
GATHERING
CHECKING
CONNECTING
CONNECTED
DEGRADED
RECONNECTING
FAILED

## Valid transitions
DISCONNECTED -> DISCOVERING
DISCOVERING -> GATHERING
DISCOVERING -> FAILED
GATHERING -> CHECKING
GATHERING -> FAILED
CHECKING -> CONNECTING
CHECKING -> FAILED
CONNECTING -> CONNECTED
CONNECTING -> DEGRADED
CONNECTING -> FAILED
CONNECTED -> DEGRADED
CONNECTED -> DISCONNECTED
DEGRADED -> RECONNECTING
DEGRADED -> CONNECTING
DEGRADED -> DISCONNECTED
RECONNECTING -> DISCOVERING
RECONNECTING -> CONNECTING
RECONNECTING -> FAILED
FAILED -> DISCONNECTED

## Invariants
- Every transition is explicit.
- Failed attempts cannot silently become trusted sessions.
- Cancellation is idempotent.
- Timeouts are bounded.
- A connection state change does not mutate application plaintext.
- A path change does not mutate cryptographic trust.

## Test requirements
- every valid transition;
- every invalid transition;
- timeout at each blocking phase;
- cancellation at each blocking phase;
- simultaneous connect;
- peer disappearance;
- direct-to-relay;
- relay-to-direct;
- restart during each state.
