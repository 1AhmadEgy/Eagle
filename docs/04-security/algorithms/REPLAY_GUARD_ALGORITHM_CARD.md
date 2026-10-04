# Eagle — Replay Guard Algorithm Card

**Status:** Implemented in Kotlin; build verification pending  
**Runtime:** Android/JVM source set  
**Scope:** One authenticated session per ReplayGuard instance

## Purpose

Reject duplicate and stale authenticated sequence numbers using a deterministic sliding window. The guard does not authenticate the sender, verify message integrity, establish identity, or validate remote wall-clock timestamps.

## Required call order

`receive -> cryptographic authentication -> session binding -> ReplayGuard.evaluate(sequence) -> protocol handler`

Never call the guard on unauthenticated sequence numbers. An attacker who can submit unauthenticated values must not be able to advance the session's replay window.

## State and algorithm

State is limited to:
- highest accepted sequence number;
- a 1–64-bit bitmap for the replay window;
- initialization state.

For a non-negative sequence number (s):

1. If the state is uninitialized, accept (s) and mark it seen.
2. If (s) is greater than the highest seen number, advance the window. If the jump is at least the window size, clear prior bits; otherwise shift the bitmap. Mark the new highest value seen.
3. Otherwise calculate the distance from the highest number.
4. If distance is outside the configured window, return TOO_OLD.
5. If the corresponding bit is already set, return DUPLICATE.
6. Otherwise mark the bit and accept the out-of-order message.

The bitmap and highest-sequence update are synchronized to make evaluation atomic across concurrent callers.

## Decisions

| Result | Meaning | Caller action |
|---|---|---|
| ACCEPT | New sequence inside the allowed window | Continue protocol handling |
| DUPLICATE | Sequence was already accepted | Reject; optionally emit minimized event |
| TOO_OLD | Sequence lies outside the replay window | Reject |
| INVALID_SEQUENCE | Sequence is negative/invalid | Reject and record bounded diagnostic |

## Security constraints

- Sequence numbers are scoped to one authenticated session.
- Instantiate a new guard only after a new session is successfully established.
- Do not share one instance across unrelated peers or sessions.
- Do not reset the guard based on an untrusted request.
- The caller must bind the sequence number to authenticated message content/session state.
- Sequence wraparound is not supported. Establish a new authenticated session before exhausting the non-negative Long range.
- This guard is not a substitute for a cryptographic nonce, MAC/signature verification, key separation, or authenticated session establishment.
- Time-based timestamp freshness is deliberately separate: wall-clock timestamps are not safely comparable to local monotonic time without a protocol-specific clock model.

## Privacy

Do not log payloads, keys, or authentication material. If telemetry is needed, record bounded event type, result, policy/window version, and a session-local pseudonymous identifier.

## Tests

JUnit tests are in `app/src/test/java/com/eagle/app/security/ReplayGuardTest.kt`. They cover initial acceptance, duplicate detection, in-window reordering, stale sequences, large advances, negative input, a 64-entry window, Long.MAX_VALUE, and invalid configuration.

## Verification

Implementation and tests are committed, but build/test PASS remains pending until the Gradle test and lint gates actually run.
