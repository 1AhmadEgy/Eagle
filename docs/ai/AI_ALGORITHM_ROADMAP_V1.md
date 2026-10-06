# Eagle AI Algorithms Roadmap V1 — Proposed

## Goal

Build deterministic, testable algorithms around the AI layer so that probabilistic model output is never the only source of truth.

## Algorithm families

### 1. Intent normalization

Transform free-form user input into a bounded intent schema.

```text
raw input
   -> normalization
   -> intent candidates
   -> constraint filtering
   -> ranked intent
```

The algorithm must preserve ambiguity instead of inventing missing security-critical facts.

### 2. Search and retrieval

V1 retrieval should be local and bounded:

- scope by conversation or allowed dataset;
- filter by time, sender, and content type;
- rank by relevance and recency;
- retain provenance;
- enforce a maximum result count.

A later semantic index may be evaluated separately. Embeddings must not be treated as automatically non-sensitive.

### 3. Ranking

Candidate plans, search results, and interpretations can be ranked using deterministic tie-breakers.

Recommended order:

```text
policy validity
  -> evidence quality
  -> exactness / relevance
  -> recency
  -> cost
  -> model preference
```

The model's preference must never outrank a policy rejection.

### 4. Classification

Use classification for non-authoritative tasks such as:

- intent category;
- content type;
- operational state;
- anomaly class;
- confidence bucket.

Security decisions remain outside the classifier.

### 5. Anomaly detection

Initial targets:

- repeated failed requests;
- abnormal retry rates;
- malformed protocol/application inputs;
- unexpected state transitions;
- resource-use spikes.

Output:

```json
{
  "signal": "retry_anomaly",
  "score": 91,
  "evidence_ids": ["ev-123"],
  "recommended_review": true
}
```

The algorithm reports a signal; it does not revoke identities, change trust, or modify security policy.

### 6. Planning

Use a hybrid strategy:

```text
LLM proposal
    +
deterministic constraints
    +
capability graph
    +
resource limits
    +
policy checks
    -> executable candidate plan
```

Planning algorithms should support bounded search and deterministic replay for tests.

### 7. Graph reasoning

Later versions may model:

- people / devices / conversations;
- capabilities;
- evidence;
- dependencies;
- synchronization relationships.

Graph edges must have provenance and should not imply authority by themselves.

### 8. Synchronization algorithms

Network and mesh optimization are later-stage work. AI may recommend a strategy, but protocol/mesh contracts remain authoritative.

Candidates for later evaluation:

- bounded priority queues;
- conflict classification;
- deterministic merge policies;
- backoff and retry scheduling;
- topology-aware route scoring.

### 9. Resource optimization on Android

All AI execution should respect:

- latency budget;
- memory budget;
- battery state;
- thermal state where available;
- network availability;
- foreground/background lifecycle.

The scheduler should prefer cancellation and degradation over unbounded execution.

## Algorithm quality requirements

Each algorithm should define:

- input domain;
- output domain;
- invariants;
- deterministic mode, where practical;
- complexity expectations;
- failure behavior;
- test vectors;
- evidence/provenance requirements.

## Initial implementation order

```text
A-001 Intent normalization
  ->
A-002 Local retrieval
  ->
A-003 Deterministic ranking
  ->
A-004 Risk / anomaly scoring
  ->
A-005 Bounded planner
  ->
A-006 Evaluation / reflection
  ->
A-007 Graph reasoning
  ->
A-008 Sync / mesh optimization
```

AI model integration should be layered over these contracts rather than embedded inside them.
