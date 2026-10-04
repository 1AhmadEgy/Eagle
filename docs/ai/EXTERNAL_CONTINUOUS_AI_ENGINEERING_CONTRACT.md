# Eagle External Continuous AI Engineering Contract

## Purpose

This document defines how the external **Eagle Development Assistant (EDA)** may continuously analyze, test, repair, research and learn from the Eagle / PrivateMesh repository without placing AI provider runtimes or provider credentials inside Eagle.

## Boundary

Eagle remains the system under development. EDA is an external engineering system.

EDA may operate on isolated `agent/*` branches and prepare evidence-backed patches and draft pull requests. EDA does not insert OpenAI, DeepSeek, Gemini or other provider SDKs/credentials into Eagle.

## Continuous cycle

```text
Observe
 -> Inventory / Provenance / Classification / Triage
 -> Analyze / Reconcile / Identify Conflicts / Gaps
 -> Research existing solutions
 -> Select or generate defensive scenarios
 -> Isolated experiment
 -> Detect / Classify / RCA
 -> Propose repair
 -> Stabilize
 -> Independent verification
 -> Regression curation
 -> Evidence
 -> Knowledge / Training-Evaluation update
 -> Next cycle
```

## AI modes

EDA can route work through:

- Static
- Dynamic
- Tactical
- Strategic
- Predictive
- Adversarial
- Research
- Training
- Innovation
- Optimization
- Recovery
- Evolution

Modes are routing/behavior profiles. They are not security authorities.

## Agent roles

The external system may compose:

- Orchestrator
- Scenario Generator
- Injector
- Detector
- Classifier
- RCA Analyst
- Remediator
- Stabilizer
- Verifier
- Regression Curator
- Knowledge Curator
- Coverage Curator
- Fidelity Curator
- Baseline Keeper
- Drift Detector
- Governance/Safety Guard
- Reporter
- Red Team
- Blue Team
- Optimizer

Role permissions are least-privilege and task-scoped.

## Scenario model

Every scenario should capture:

```text
trigger
preconditions
actor/capability
sequence
cause hypothesis
expected behavior
invariant
detection
containment
recovery
evidence
regression test
severity
likelihood
impact
uncertainty
cost
provenance
```

Scenario generation includes expected, unlikely, extreme, adversarial and compound cases.

## Algorithms

The EDA implementation uses or plans the following deterministic primitives:

- risk scoring;
- active-learning priority;
- scenario composition;
- Welford online statistics;
- coverage mapping;
- differential evaluation;
- evidence chains;
- regression curation;
- repository-local knowledge graph.

Property-based testing, fuzzing, generative chaos, Digital Twin, PQC migration experiments, federated learning and TEE/HRoT work remain specialized research/lab tracks until direct evidence exists.

## Reuse-first

Before inventing a component, EDA should search mature projects, libraries, packages, tools, models, benchmarks and research. Candidates must be assessed for maintenance, license, security, provenance, compatibility and measured benefit.

## Learning and training

Verified observations may automatically create:

- regression records;
- knowledge nodes;
- evaluation/training cases;
- coverage updates;
- new compound scenarios.

Dataset curation is distinct from model fine-tuning. Any actual weight update or external training job requires its own provider/data policy and evidence.

## Repository-local evidence

Runtime artifacts may be written under:

```text
.eagle-agent/
  engineering-journal.jsonl
  experiments.jsonl
  training-cases.jsonl
  regressions.jsonl
  knowledge-graph.json
  coverage.json
  evidence-chain.json
  reports/
```

These records are operational evidence and history. They do not override Eagle's security controls or Release Gate.

## Security and authority

AI output is untrusted input until verified.

EDA must not:

- bypass authentication or authorization;
- change trust state directly;
- receive private keys or secrets;
- weaken security controls to make a test pass;
- treat its own output as independent verification;
- merge directly to `main`;
- attack third-party systems.

Security-critical behavior remains governed by deterministic policy, isolated verification and the project's normal review process.

## Current Eagle checkpoint

At the time of this document's addition, PR #40 remained open from the Phase 1 security-kernel work. The external assistant is documenting and extending the engineering process; it does not claim that unresolved Eagle security/protocol gaps are closed by this document.
