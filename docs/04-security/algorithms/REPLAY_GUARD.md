# Eagle — Replay Guard Algorithm Card

Status: Implemented in Kotlin; build verification pending
Implementation: app/src/main/java/com/eagle/app/security/ReplayGuard.kt

## Algorithm

The guard maintains:
- highest sequence number observed;
- a bitmap of recently observed sequence numbers;
- a configurable window of 1–64 entries.

For a new sequence value:
- if no sequence was seen, accept and initialize the high-water mark;
- if the value is greater than the high-water mark, advance the window and accept;
- if the value is within the window and its bit is already set, reject as duplicate;
- if the value is within the window and unseen, set its bit and accept;
- if the value is older than the window, reject as too old;
- negative sequence values are invalid.

The bounded bitmap makes replay state O(1) for a single session and avoids an unbounded received-sequence set.

## Trust boundary

The caller must authenticate the message and bind it to the correct session before evaluating the sequence number.

The guard does not generate nonces, verify signatures/MACs, establish sessions, or authenticate identities.

## Ordering policy

The current policy allows bounded out-of-order delivery within the configured window. A protocol requiring strict ordering can use window size 1.

## Wraparound

Negative values are invalid and sequence wraparound is not supported. A protocol must establish a new authenticated session before exhausting the non-negative sequence space.

## Tests

JUnit tests cover first-use acceptance, duplicates, bounded out-of-order delivery, old packets, large window advances, negative input, 64-entry window, maximum Long value, and invalid configuration.

## Security events

The caller may emit a privacy-minimized SecurityEvent for duplicate, too-old, or invalid sequence decisions. Raw message contents, secrets, and challenge material must not be logged.
