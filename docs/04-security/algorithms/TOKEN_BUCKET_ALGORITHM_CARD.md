# Eagle — Token Bucket Rate Limiting Algorithm Card

Status: Implemented in Kotlin; build verification pending
Date: 2026-10-04
Security role: Deterministic abuse-control primitive

## 1. Purpose
Limit security-sensitive operations while permitting controlled short bursts.

Initial application points: authentication attempts; session establishment; peer discovery; message submission; telemetry upload.

The limiter is deterministic. AI/ML may observe limiter events, but may never disable or override the limiter.

## 2. Threat model
Mitigates request/message flooding, repeated authentication attempts, resource-exhaustion pressure, and burst abuse from a single identity, peer, or session.

It does not provide authentication, authorization, replay protection, cryptographic freshness, identity verification, or distributed global quota enforcement unless separately approved.

## 3. State
Each bucket contains capacity, tokens, refillRate, lastRefill, and policyVersion.

Optional partitioning dimensions are identity, peer, session, or operation. Bucket keys must not contain raw secrets.

## 4. Time
Use a monotonic clock, never wall-clock time, for refill calculations.

If the clock moves backward or produces an invalid delta, treat elapsed time as zero and preserve the last-refill high-water mark. A backward reading must not move the anchor backward and later create extra refill. Never create tokens from a negative delta.

## 5. Deterministic algorithm
The first call establishes the monotonic clock anchor and does not create refill time. This prevents an arbitrary first timestamp from minting tokens.

For request cost:
1. Validate policy and cost.
2. Read monotonic time now.
3. Compute elapsed = max(0, now - lastRefill).
4. Refill using the documented fixed-point/integer representation.
5. Clamp tokens to capacity.
6. If tokens >= cost, subtract cost and allow.
7. Otherwise reject/throttle without increasing the balance.
8. Advance lastRefill only when now is later than the stored high-water mark.
9. Emit only privacy-minimized telemetry.

Continuous reference equation:
tokens = min(capacity, tokens + elapsed_seconds * refillRate)

Decision:
allow iff tokens >= cost

Portable deterministic implementations should represent fractional tokens using a documented integer fixed-point unit or another explicitly tested representation.

## 6. Overflow and numeric safety
Use checked arithmetic.

Potential overflow sites include elapsed time multiplied by refill rate, fixed-point scaling, and cumulative counters.

Overflow must fail closed or saturate safely; it must never wrap into a large positive token balance.

## 7. Policy semantics
Recommended initial constraints:
- capacity > 0
- refillRate > 0
- 0 < cost <= capacity
- one bucket per selected policy key
- bounded bucket lifetime/storage
- explicit eviction behavior
- no unbounded attacker-controlled key cardinality

A request costing more than capacity is invalid policy/input, not normal depletion.

## 8. Failure behavior
| Condition | Required behavior |
|---|---|
| Negative elapsed | Treat as zero |
| Arithmetic overflow | Fail closed / safe saturation |
| Invalid capacity/rate | Reject policy configuration |
| Cost <= 0 | Reject input |
| Cost > capacity | Reject input |
| Bucket absent | Initialize at documented starting balance |
| Memory/storage pressure | Apply bounded eviction; never bypass limiter |
| Telemetry failure | Limiter decision remains authoritative |

## 9. Security boundary
request -> deterministic limiter -> allow/deny -> operation

Separately:
limiter event -> SecurityEvent -> FeatureVector -> optional risk/anomaly signal

The second path cannot feed back into the first without an explicitly approved deterministic policy change.

## 10. Privacy
Telemetry may contain event type, policy version, operation class, result, bounded counters, and a pseudonymous local bucket identifier where required.

Never log tokens, private keys, authentication secrets, raw message bodies, or unnecessary personal data.

## 11. Performance
The core operation should be O(1) in time and O(1) per active bucket in state.

No numeric latency claim is approved until the actual runtime is established and benchmarked.

## 12. Testing
Required vectors are in docs/04-security/algorithms/TOKEN_BUCKET_TEST_VECTORS.json.

Minimum coverage: empty bucket; exact-cost request; insufficient tokens; burst up to capacity; refill; capacity clamp; fractional refill; negative clock delta; overflow boundary; invalid policy; oversized cost; bucket isolation; bounded eviction.

## 13. Implementation gate
The Kotlin implementation exists, but production adoption remains gated on passing build/test/lint verification and integration review.

Promotion to Implemented requires an authoritative requirement, threat-model mapping, actual runtime/module boundary, passing vectors, concurrency tests where applicable, performance evidence, memory/cardinality evidence, code review, and ADR approval.
