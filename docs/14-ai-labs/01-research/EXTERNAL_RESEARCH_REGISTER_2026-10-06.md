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