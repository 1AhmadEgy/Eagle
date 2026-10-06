# Eagle External Research Register — 2026-10-06
Status: Proposed / Documentation Only

## Authority boundary
This register records external evidence and potential Eagle impact. It does not approve an ADR, requirement, implementation, dependency, model, workflow permission, or release decision.

Evidence → Impact assessment → Requirement candidate → ADR if architectural → Test/Gate → Human Review → Accepted.

## Sources
| ID | Source | Topic | Eagle relevance | Status |
|---|---|---|---|---|
| EXT-001 | GitHub Actions Secure Use Reference | least privilege, secrets, untrusted code | CI/AI Labs | Verified |
| EXT-002 | GitHub GITHUB_TOKEN | token lifecycle and scope | agent/workflow authority | Verified |
| EXT-003 | GitHub Artifact Attestations | signed build provenance | release/provenance | Verified |
| EXT-004 | GitHub Reusable Workflows | deterministic shared logic | Test Labs | Verified |
| EXT-005 | GitHub Compromised Runners | runner compromise | CI threat model | Verified |
| EXT-006 | OpenSSF Scorecard | supply-chain posture | CI/dependencies | Collected |
| EXT-007 | SLSA | build provenance | release evidence | Collected |
| EXT-008 | OWASP GenAI Top 10 2025 | prompt injection, disclosure, supply chain, agency | AI Labs | Verified |
| EXT-009 | OWASP Excessive Agency | permissions/autonomy | AI authority boundary | Verified |
| EXT-010 | NIST AI RMF | AI risk management | AI Labs | Verified |
| EXT-011 | NIST GenAI Profile | GAI security/privacy | AI Labs | Verified |
| EXT-012 | Signal PQXDH | authenticated PQ key agreement | crypto ADR research | Verified |
| EXT-013 | Signal Double Ratchet | ratcheting/security properties | crypto ADR research | Verified |
| EXT-014 | NIST FIPS 203 | ML-KEM standard | PQ research | Verified |
| EXT-015 | Kotlin Multiplatform expect/actual | platform boundary | KMP architecture | Verified |
| EXT-016 | Kotlin/Native memory manager | concurrency/memory model | KMP/FFI | Verified |

## Key findings

### EXT-F-001 — Least privilege
GitHub recommends minimum GITHUB_TOKEN permissions and read-only contents by default, increasing permissions only where required.
Impact: audit every AI/CI workflow permission against its exact required action. Capability is not authority.

### EXT-F-002 — Provenance is not a security verdict
GitHub artifact attestations cryptographically link an artifact to build provenance. GitHub explicitly warns that an attestation does not itself guarantee artifact security.
Impact: provenance verification and security verification remain separate release evidence.

### EXT-F-003 — Reusable workflows
GitHub documents reusable workflows as centralized repeatable logic; combined with attestations they can support stronger build provenance.
Impact: candidate future Test Lab normalization. No implementation approval.

### EXT-F-004 — Compromised runners
GitHub documents that compromised runners can expose secrets/tokens and that risk varies by event trigger.
Impact: runner compromise and event-trigger analysis belong in CI/AI Labs threat coverage.

### EXT-F-005 — Excessive AI agency
OWASP identifies excessive functionality, excessive permissions, and excessive autonomy as root causes; hallucination and prompt injection can trigger harmful actions.
Impact: reinforces the separation between AI tool capability and security authority.

### EXT-F-006 — AI evaluation lifecycle
NIST AI RMF and the Generative AI Profile frame trustworthy AI as risk management across design, development, deployment, use, and evaluation, including security and privacy.
Impact: candidate framework for AI Labs evaluation documentation; not an Eagle requirement until reviewed.

### EXT-F-007 — Signal protocols remain research inputs
Signal specifications document PQXDH and Double Ratchet properties, but Eagle has not accepted them as its production protocol.
Impact: preserve as external evidence for cryptographic ADR/conformance work.

### EXT-F-008 — ML-KEM standardization
NIST FIPS 203 standardizes ML-KEM and defines ML-KEM-512/768/1024 parameter sets.
Impact: useful PQ migration evidence; does not select an Eagle parameter set or provider.

### EXT-F-009 — KMP platform boundaries
Kotlin expect/actual requires corresponding platform implementations and is useful for explicit platform capability mapping.
Impact: candidate evidence for platform analysis; it does not establish Eagle's final architecture.

## Explicit non-decisions
- No selection of Signal/PQXDH/Double Ratchet/MLS/Noise.
- No ML-KEM parameter or provider selection.
- No reusable-workflow architecture approval.
- No AI model/provider selection.
- No database/storage technology selection.
- No release policy change.

## Review triggers
Re-review when GitHub Actions security changes, a CI/security incident occurs, a crypto provider/protocol is proposed, an AI tool/model is proposed, platform architecture changes, or release provenance requirements change.
## Additional 2026 research

### EXT-017 — NIST TEVV-Athlon
NIST's August 2026 public draft describes a structured framework for customized AI Test, Evaluation, Verification and Validation (TEVV), including agentic systems. It emphasizes producing evidence from defined events/tools and measurement blocks.
Impact: useful research input for an Eagle AI Lab evaluation contract. The document was still a public draft on 2026-10-06; it is not an Eagle standard.

### EXT-018 — GitHub Agentic Workflows
GitHub documents agentic repository workflows as public preview and describes read-only defaults, declared safe outputs, isolated execution, threat detection, and human review in the loop.
Impact: strong external reference for comparing Eagle's existing AI repair restrictions and the AILAB-05..07 authority model. No adoption decision.

### EXT-019 — GitHub Actions execution protections
GitHub documents repository/organization/enterprise workflow execution protections that can restrict who may trigger workflows and which events are permitted. GitHub also states that a default public-repository policy will block pull_request_target on November 2, 2026.
Impact: relevant to AILAB-22..24 and the distinction between repository policy and GitHub configuration evidence.

### EXT-020 — OWASP DonkAI
OWASP DonkAI is a deliberately vulnerable, rule-based hands-on lab covering the OWASP 2025 GenAI Top 10. It is described as deterministic, offline-friendly, and reproducible, with explicit warnings to isolate the lab.
Impact: useful external model for Eagle's future AI security scenario lab design, especially reproducibility and isolated attack exercises. It is not a dependency or runtime component.

### EXT-F-010 — TEVV artifacts should be documented for repeatability
NIST AIRC guidance states that test sets, metrics, tools, processes and materials used for TEVV should be documented to support repeatability and consistency.
Impact: strengthens Eagle's existing requirement that AI findings remain evidence-backed and reproducible rather than relying on model assertions.

### EXT-F-011 — Agentic workflow safety is layered
GitHub's current agentic-workflow documentation describes multiple controls: read-only defaults, safe outputs, secret isolation, threat detection, firewalled execution and role-based access.
Impact: candidate comparison framework for Eagle AI Labs. Eagle must preserve its own stricter authority boundary and does not inherit GitHub's model as an accepted design.

### EXT-F-012 — Workflow execution policy is distinct from workflow YAML
GitHub documents execution protections as repository/organization/enterprise policy that can restrict actors/events independently of a workflow file.
Impact: reinforces Eagle's evidence distinction between repository policy, GitHub configuration evidence, and observed API permissions.