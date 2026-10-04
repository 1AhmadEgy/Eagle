# Eagle — Algorithm-by-Algorithm Reverse Engineering

Status: VERIFIED DESIGN BASELINE — PRODUCT IMPLEMENTATION NOT YET AUTHORIZED

This document separates algorithms evidenced by the repository from algorithms proposed for future implementation.

## 1. Authentication

Current evidence: no product authentication implementation is present in the audited main baseline.

Required design:
- deterministic protocol validation
- challenge/response or authenticated session establishment
- explicit nonce/freshness rules
- bounded retry/rate limiting
- fail closed

AI role: none in the authority path.

Candidate reuse:
- use a mature, reviewed cryptographic protocol/library rather than implementing cryptography.
- Android Keystore should be evaluated for device-bound key protection once the Android stack is defined.

Tests:
- positive/negative vectors
- replay
- nonce reuse
- downgrade
- malformed messages
- key rotation
- rate-limit exhaustion

## 2. Identity

Current evidence: no product identity implementation is present in the audited main baseline.

Required design:
Identity -> authenticated public-key binding -> device/session state -> authorization policy.

AI role: none.

Important invariant:
A model score must never establish identity.

## 3. Peer trust

Current evidence: no peer-trust implementation is present.

Proposed first implementation:
Use explicit deterministic evidence rather than an opaque model.

Example normalized evidence:
- authentication validity
- protocol compliance
- recent failures
- successful authenticated interactions
- freshness
- local administrative policy

A future reputation score may aggregate evidence, but it must remain bounded and auditable.

## 4. Session state

Current evidence: deterministic session state machine is implemented in Kotlin.

Implemented structure:
NEW -> AUTHENTICATING -> ESTABLISHED -> REKEYING -> ESTABLISHED
Any non-closed state may transition to CLOSED; CLOSED is terminal.

Invalid transitions must be rejected deterministically.

AI role: telemetry/anomaly evidence only.

Tests:
- every valid transition
- every invalid transition
- duplicate transition
- timeout
- concurrent close/rekey
- restart recovery

## 5. Replay detection

Current implementation: `ReplayGuard` provides a per-session bounded sliding window (1–64 sequence positions). Required deterministic controls:
- unique session/connection context
- nonce/sequence number
- freshness window where applicable
- duplicate detection
- authenticated state binding

AI is unnecessary for primary replay prevention.

A model can later identify unusual replay-like patterns, but it must never replace cryptographic freshness. `ReplayGuard` is not message authentication and must only be called after authenticated session binding.

## 6. Rate limiting

Current implementation: `TokenBucket` is implemented as a deterministic Kotlin primitive; build verification remains pending.

First implementation should be deterministic:
- token bucket or leaky bucket
- per-identity/per-peer/per-session dimensions
- bounded memory
- monotonic time
- fail-safe behavior

AI role: detect abuse patterns and recommend investigation thresholds; never disable the limiter.

## 7. Telemetry

Define a privacy-minimized SecurityEvent before any ML model.

Minimum conceptual fields:
- event type
- monotonic timestamp
- local context identifier
- peer pseudonymous identifier
- protocol state
- outcome
- bounded counters
- schema version

Never put:
- private keys
- raw secrets
- message plaintext
- unnecessary personal data

## 8. Feature extraction

Current implementation: `SecurityFeatureExtractor` converts bounded ordered `SecurityEvent` windows into integer-scaled `FeatureVector` values. Build verification remains pending.

Feature extraction must be deterministic and versioned.

Example:
FeatureVector {
  schemaVersion,
  eventRate,
  failureRate,
  retryRate,
  protocolViolationRate,
  peerHistorySummary,
  sessionAge,
  freshnessDeviation
}

Every feature requires:
- definition
- units
- range
- missing-value behavior
- privacy classification
- test vectors

## 9. Baseline risk scoring

Start with a transparent deterministic score.

For example:

risk = clamp(
  w1*failureRate +
  w2*retryRate +
  w3*protocolViolationRate +
  w4*freshnessDeviation,
  0,
  100
)

This is a design placeholder, not an approved production formula.

Weights must be derived from requirements and evaluation evidence rather than arbitrary intuition.

Output:
RiskAssessment(score, reasons, schemaVersion, policyVersion)

## 10. Statistical anomaly detection

### Z-score

z = (x - mean) / sigma

Use only after establishing a baseline dataset.

### EWMA

s_t = alpha*x_t + (1-alpha)*s_(t-1)

Useful for drift/change detection with very low runtime cost.

### Robust alternatives

Median/MAD should be evaluated where distributions are heavy-tailed or contain legitimate outliers.

## 11. Isolation Forest

Purpose: nonlinear unsupervised outlier detection.

Training:
offline.

Inference:
potentially on-device after model export and runtime validation.

Prerequisites:
- representative feature dataset
- contamination/threshold policy
- false-positive evaluation
- model provenance
- resource benchmark

It must produce an AnomalySignal, not an authorization decision.

## 12. One-Class SVM

Purpose: model a normal operating region.

Risks:
- feature scaling sensitivity
- hyperparameter sensitivity
- difficult threshold interpretation
- potentially higher runtime/memory cost than simple statistics

Use only if it demonstrates material improvement over the baseline.

## 13. Clustering

Purpose:
- discover peer/event populations
- detect population shifts
- support offline investigation

It should not directly determine authorization.

## 14. Sequence models

LSTM/Transformer models are deferred.

They require:
- ordered event schema
- sequence window definition
- training/evaluation dataset
- leakage controls
- adversarial evaluation
- model versioning
- Android performance measurements

They are not justified before simpler baselines are measured.

## 15. LLM explanation

Pipeline:

RiskAssessment + minimized evidence -> ExplanationModel -> human-readable explanation

The LLM receives structured evidence, not raw private state.

The LLM cannot:
- authenticate
- authorize
- handle keys
- alter protocol state
- override deterministic policy
- suppress security findings

## 16. Decision table

| Algorithm | First choice? | Reason |
|---|---|---|
| Deterministic state machine | YES | Security correctness |
| Token bucket | YES | Transparent abuse control |
| Z-score | Candidate | Cheap baseline |
| EWMA | Candidate | Cheap temporal baseline |
| Median/MAD | Candidate | Robust statistics |
| Isolation Forest | Later | Requires data |
| One-Class SVM | Later | More tuning/complexity |
| Clustering | Later | Exploratory evidence |
| LSTM/Transformer | Deferred | High data/compute burden |
| LLM | Explanation only | Non-deterministic |

## 17. Implementation gate

No algorithm moves from Candidate to Implemented until:
1. requirement exists;
2. threat model impact is reviewed;
3. inputs are defined;
4. output contract is defined;
5. test vectors exist;
6. performance budget is known;
7. privacy classification exists;
8. mature implementation has been evaluated;
9. rollback/fallback is defined;
10. ADR is approved.

## 18. Current repository truth

The repository is currently a foundation rather than an implemented Android security product. Therefore this document intentionally records **design and verification status**, not fictitious implementation status.
