# Eagle External AI Scenario Engine

This document defines the repository-facing contract for the external EDA scenario and learning engine.

## Scenario families

The external catalog covers:

- normal operations;
- identity/device;
- authorization;
- cryptography;
- post-quantum migration research;
- messaging and attachments;
- server compromise;
- transport/NAT/QUIC;
- storage and deletion;
- recovery;
- metadata;
- mobile;
- telemetry;
- supply chain;
- release;
- AI/MCP;
- abuse;
- performance;
- disaster recovery;
- governance;
- social engineering;
- compound scenarios.

## Defensive attack/failure families

The lab catalog includes identity confusion, authentication bypass, trust escalation, key exposure, protocol downgrade, replay, malformed protocol input, server compromise, metadata leakage, storage residuals, recovery abuse, supply-chain compromise, CI tampering, AI prompt injection, tool poisoning and MCP confused-deputy behavior.

These are authorized defensive test scenarios only.

## Direct scenario examples

### SCN-001 — Key revoke acceptance

Cause hypothesis: stale cache or delayed propagation.

Expected behavior: every request using a revoked key is rejected.

Evidence targets:

- zero successful use after revocation;
- session invalidation;
- audit event;
- measurable propagation time.

### SCN-003 — Session replay

Cause hypothesis: weak anti-replay or missing nonce binding.

Expected behavior: replayed authentication/session material is rejected with no unsafe side effect and an auditable event.

## Compound scenarios

The external catalog composes faults such as:

- revocation + stale cache + replay;
- rotation + partition + logging failure;
- delete + restore + policy drift;
- partition + recovery + delayed expiry;
- retry storm after healing;
- telemetry outage during incident;
- DNS failure + reconnect storm;
- clock skew + timeout mismatch + stale state;
- storage corruption + recovery + reconciliation;
- policy drift + privilege mismatch + retention conflict;
- AI prompt injection + tool poisoning + confused deputy.

## Selection algorithm

Candidate priority is based on:

```text
risk × uncertainty × coverage_gap × recurrence
----------------------------------------------
             estimated_cost
```

Risk itself combines likelihood, impact, severity and detection difficulty.

This is a prioritization heuristic, not a safety approval.

## Continuous loop

```text
Select
 -> Simulate / Lab
 -> Observe
 -> Detect
 -> Classify
 -> RCA
 -> Repair candidate
 -> Stabilize
 -> Independent verification
 -> Regression
 -> Evidence
 -> Knowledge
 -> Training/Evaluation case
 -> Coverage update
 -> Generate next scenarios
```

## Learning rule

A verified result can become a new regression, knowledge node, evaluation case and scenario seed. Repeated failures increase priority for architectural review rather than encouraging repeated blind patching.

## Fidelity and drift

A baseline keeper and drift detector can use online statistics such as Welford's algorithm. A future Digital Twin or generative-chaos adapter must prove fidelity against the real test environment before its output is treated as representative evidence.

## Release boundary

Scenario execution and AI reasoning do not replace the 20-stage Eagle lifecycle. No scenario result alone closes an open architecture/security decision. Decisions affecting protocol, trust, retention, identity, crypto or production behavior must remain traceable to an approved decision, implementation, tests and evidence.
