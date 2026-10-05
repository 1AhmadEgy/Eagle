# MESH-011 — Network Change and Recovery
Status: SPECIFIED / IMPLEMENTATION-GATED

## Events
- interface change
- IP/NAT mapping change
- loss of connectivity
- sleep/wake
- route invalidation

## Required behavior
Re-gather candidates, re-check connectivity, preserve protocol semantics, and recover without trust escalation.

## Acceptance
Network-change and sleep/wake tests prove deterministic recovery.
