# MESH-010 — Retry and Backoff
Status: SPECIFIED / IMPLEMENTATION-GATED

## Policy
Failures are classified as transient, recoverable, or terminal.

## Rules
- Retries are bounded.
- Backoff is deterministic within a defined jitter policy.
- Terminal errors do not spin.
- Retry state is observable without payload leakage.

## Acceptance
Tests cover retry exhaustion, cancellation, backoff bounds, and concurrent-failure storms.
