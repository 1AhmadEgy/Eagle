# Self-Model and Introspection — Execution Slice v1

## Purpose

This slice introduces the first executable foundation for functional self-awareness inside Eagle's engineering core.

It provides:

- an explicit operational self-model;
- deterministic cognitive-state tracking;
- bounded episodic event memory;
- goals, capabilities and known limitations;
- introspection snapshots that are safe to inspect and compare;
- regression tests for the observe → reason → act → verify loop.

## Explicit boundary

This is **functional self-modeling**, not a claim of consciousness.

The self-model is deliberately non-authoritative:

- it cannot authenticate a principal;
- it cannot change trust state;
- it cannot authorize a capability;
- it cannot access cryptographic keys;
- it cannot execute tools;
- it cannot modify release/security policy;
- it cannot declare its own output independently verified.

Security authority remains in deterministic policy and the normal Eagle review/release gates.

## Why the first slice is deterministic

The first implementation intentionally contains no LLM SDK, model weights, vector database, network service, MCP runtime or provider credential.

That makes the self-model:

- reproducible;
- unit-testable;
- portable;
- inspectable;
- safe to place behind the existing AI/MCP policy boundary.

A later adapter may populate the model from an external EDA/LLM runtime, but the adapter must remain untrusted input and cannot obtain authority from the model.

## State model

The initial state set is:

`Idle → Observing → Reasoning → Acting → Verifying`

with `Recovering` available for failure handling.

Transitions are intentionally simple in v1. Higher-level transition policy belongs in the orchestrator/evidence layer rather than this data model.

## Memory model

Memory is a bounded event buffer.

Each event has:

- monotonic sequence number;
- event kind;
- sanitized summary;
- confidence score.

The buffer drops the oldest entry when the capacity is exceeded.

No secret/key storage is intended or permitted in this structure.

## Introspection

`SelfModel::introspect()` returns an immutable snapshot containing:

- identity label/version;
- current cognitive state;
- confidence;
- goals;
- capabilities;
- limitations;
- memory count;
- latest memory event.

Because the snapshot is copied, consumers cannot mutate the underlying model through the snapshot.

## Next slices

1. Self-observation events bound to deterministic repository evidence.
2. Episodic/semantic memory split with explicit retention rules.
3. Reflection/evaluation layer that compares predictions with outcomes.
4. Goal planner that proposes actions but remains subject to authorization.
5. Tool/MCP adapter with schema validation, sandboxing and audit.
6. External model adapters (OpenAI/DeepSeek/Gemini/etc.) kept outside Eagle's security core.
7. Knowledge graph and regression learning backed by evidence IDs.
8. Differential model evaluation and drift detection.

Every later slice must preserve the rule:

`AI cognition → policy → authorization → sandbox → execution → audit`.
