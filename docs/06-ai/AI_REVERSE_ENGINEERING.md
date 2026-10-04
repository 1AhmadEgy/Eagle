# Eagle — AI / ML Reverse Engineering Baseline

Status: CONFIRMED BASELINE / PRODUCT-AI NOT IMPLEMENTED
Branch: ai/reverse-engineering-foundation
Audit baseline: main @ 6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee

## Executive finding
The current repository must not be described as containing a product AI/ML engine.
The verified repository evidence shows two distinct AI surfaces:
1. Development-time AI agents — OpenCode agents driven by DeepSeek in GitHub Actions.
2. Product AI/ML — no verified Android/Kotlin application source, model runtime, trained model, feature pipeline, anomaly detector, risk engine, reputation model, or inference module exists in the audited main baseline.
The README phrase 'Built with AI Studio' and its Gemini wording identify repository starter provenance; they do not prove Gemini is an Eagle runtime security component.

## Verified development-time AI
### CI repair agent
.github/workflows/ai-fix-ci.yml invokes OpenCode with model deepseek/deepseek-v4-flash and agent orchestrator. It uses the DEEPSEEK_API_KEY secret.
### Documentation agent
.github/workflows/documentation-agent.yml invokes the same OpenCode action/model with agent documentation-agent. The agent is explicitly read-only.
### Agent governance
AGENTS.md requires human review before merge, forbids direct pushes to main, forbids weakening tests and security checks, requires bash scripts/ci/verify.sh, and requires missing test categories to remain PENDING.

## Product AI status
| Capability | Status |
|---|---|
| Android/Kotlin product code | NOT IMPLEMENTED in audited main baseline |
| ML inference runtime | NOT IMPLEMENTED |
| Model artifact | NOT IMPLEMENTED |
| Feature extraction pipeline | NOT IMPLEMENTED |
| Anomaly detection | NOT IMPLEMENTED |
| Risk scoring engine | NOT IMPLEMENTED |
| Peer reputation model | NOT IMPLEMENTED |
| Replay detection algorithm | NOT IMPLEMENTED |
| Sequence model | NOT IMPLEMENTED |
| LLM inside product | NOT IMPLEMENTED |
| AI-based security decision authority | PROHIBITED BY DESIGN |

## Security boundary
AI/ML must produce evidence or advisory scores; deterministic security policy owns enforcement decisions.
A model must never directly authorize an identity, release private keys, bypass authentication, select cryptographic primitives, override protocol state validation, disable rate limits, or approve an untrusted peer solely from a model score.
Reference pipeline:
sanitized event -> feature extraction -> model -> bounded signal -> deterministic policy -> action

## Required product-AI interfaces
SecurityEvent, FeatureVector, AnomalySignal, RiskAssessment, ReputationEvidence, PolicyDecision, ModelMetadata, Explanation.
Every model output should carry model identifier, model version, feature-schema version, inference timestamp, score semantics, evidence identifiers, and fail-safe behavior.

## Adoption order
1. deterministic feature extraction
2. robust statistical baselines
3. EWMA / Z-score temporal deviation
4. bounded deterministic risk aggregation
5. supervised/unsupervised ML after data and evaluation exist
6. model runtime integration
7. optional local LLM explanation
8. remote LLM only for explicitly approved, minimized, non-sensitive evidence

## Current blockers
The repository's execution-readiness record states that product implementation is blocked pending authoritative requirements.
Before product AI is implemented, freeze security objectives, event taxonomy, trust boundaries, feature schema, privacy constraints, acceptable error rates, device performance budget, model update/provenance policy, and acceptance tests.

## Evidence
AGENTS.md; .github/workflows/ai-fix-ci.yml; .github/workflows/documentation-agent.yml; .opencode/agents/orchestrator.md; .opencode/agents/documentation-agent.md; docs/EXECUTION-READINESS.md; docs/06-execution/EXECUTION_MATRIX.md; README.md.
Historical/archive material is not treated as proof of current implementation.