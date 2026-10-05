# Eagle — Key Lifecycle Specification

## Lifecycle states

ACTIVE -> CONSUMED
ACTIVE -> REVOKED
ACTIVE -> DESTROYED
REVOKED -> DESTROYED
CONSUMED -> DESTROYED

No state may return to ACTIVE.

## Required invariants

- generation numbers are strictly monotonic for replacement;
- lifecycle epochs are strictly monotonic;
- generation zero is invalid;
- epoch exhaustion fails closed;
- key purpose cannot change;
- identity/session/message/storage/recovery domains remain distinct;
- One-Time PreKeys are single-use;
- revoked, consumed, and destroyed material is unusable;
- lifecycle metadata contains handles only; private key bytes remain outside the state machine;
- a provider that has not passed approval cannot service production cryptographic operations.

## Rotation

Rotation registers the replacement first, then revokes the predecessor. A failed registration does not revoke the predecessor. A generation rollback is rejected.

## One-Time PreKeys

A One-Time PreKey enters CONSUMED on successful protocol consumption. A consumed key cannot be consumed twice and cannot be used as an active key.

## Recovery

Recovery keys use their own purpose domain and cannot be used as messaging keys.

## Capacity

The current core implementation intentionally uses a bounded metadata table. Exhaustion is a hard failure rather than silent eviction or implicit deletion. A persistent storage-backed implementation must retain the same fail-closed invariant.
