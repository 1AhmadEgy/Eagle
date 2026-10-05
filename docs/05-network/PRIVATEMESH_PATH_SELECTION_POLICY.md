# PrivateMesh — Path Selection Policy

Status: Proposed / implementation-gated.

## Path classes
1. Direct host path.
2. Direct server-reflexive path established through NAT traversal.
3. Relayed path.
4. Future approved transport path.

## Selection order
Prefer the highest-priority healthy path that satisfies:
- peer target is correct;
- protocol compatibility is valid;
- security policy is satisfied;
- resource limits are within bounds;
- connectivity is currently healthy.

Default preference:
Direct > direct-reflexive > relay.

A relay may win temporarily when direct connectivity is unavailable or unhealthy.

## Hysteresis
Do not flap between paths on single transient failures. Path migration requires a bounded stability/health condition.

## Relay recovery
When relay is active:
1. continue carrying opaque frames;
2. gather/check direct candidates according to policy;
3. establish a healthy direct path;
4. migrate without changing E2E semantics;
5. retire relay after confirmation.

## Failure rule
No path fallback may:
- expose plaintext;
- request private keys;
- weaken protocol version/security policy;
- silently change trust state.
