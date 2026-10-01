# Eagle Security Control & Research Register

## Purpose

This register records security decisions, external guidance used to justify them, implementation status, and remaining gaps. It is intended to prevent repeated rediscovery and to keep security work auditable over time.

## Control: immutable GitHub Action references

**Status:** IMPLEMENTED in the hardened workflows.

GitHub recommends pinning third-party Actions to full-length commit SHAs as the strongest immutable reference, while also auditing the action source. This control was applied to the checkout, artifact upload, gitleaks, and OpenCode action references used by the hardened workflows.

**Implementation:** PR #5 / branch `security/actions-supply-chain-hardening`.

**Primary source:** GitHub Secure use reference:
https://docs.github.com/en/actions/reference/security/secure-use

## Control: explicit least-privilege workflow permissions

**Status:** IMPLEMENTED in the reviewed workflows.

The workflows explicitly declare permissions instead of relying on broad defaults. The privileged repair workflow remains separated behind a trusted `workflow_run` boundary.

**Primary source:** GitHub guidance on secure workflow use:
https://docs.github.com/en/actions/reference/security/secure-use

## Control: artifact provenance

**Status:** PLANNED / conditional.

GitHub artifact attestations create signed provenance claims that bind an artifact to its workflow, repository, commit SHA, event, and OIDC-derived identity. They are appropriate for release artifacts such as binaries, packages, or container images, rather than routine test builds or individual documentation/source files.

**Primary sources:**
- https://docs.github.com/en/actions/concepts/security/artifact-attestations
- https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations

**Eagle action:** Do not add attestation permissions merely for documentation artifacts. Introduce them when Eagle has a defined release artifact and the repository plan supports the required feature.

## Control: SLSA alignment

**Status:** ALIGNMENT TARGET / NOT CLAIMED.

SLSA v1.2 defines concrete requirements for provenance completeness, authenticity, and resistance to tampering, with stronger build isolation/provenance properties at higher levels.

**Primary source:** https://slsa.dev/spec/v1.2/

**Eagle action:** Use SLSA requirements as an engineering target and evidence checklist. Do not claim a formal SLSA level without demonstrating the applicable requirements for an actual build/release path.

## Control: continuous provenance

**Status:** IMPLEMENTED.

The documentation workflow records exact commit/ref/event/actor metadata, resolves Test Lab evidence for the exact SHA, validates a machine-readable provenance contract locally, and uploads the resulting evidence artifact.

**Implementation:** PR #4.

## Control: AI repair isolation

**Status:** IMPLEMENTED with review required.

The repair pipeline uses a privileged `workflow_run` only for the trusted development branch and constrains the OpenCode repair agent from modifying workflows, secrets, credentials, deployment configuration, or pushing to main. The orchestrator is not permitted to edit files.

**Implementation:** PR #3, hardened action references in PR #5, static boundary verification in PR #8.

## Control: Test Lab evidence discipline

**Status:** IMPLEMENTED as a governance rule; product categories remain PENDING until evidence exists.

The CI pipeline records exact-SHA evidence and preserves explicit PENDING states for categories without category-specific tests. A successful policy gate or baseline repository check does not promote product categories to PASS.

**Research/engineering basis:** GitHub provenance guidance and SLSA evidence model; see the deep-research report at `docs/research/deep-research-executive-summary-2026-10-01.md`.

## Security research maintenance rule

When a future change depends on external security guidance:

1. record the source URL and date/relevance;
2. record the exact control derived from it;
3. implement the smallest safe change;
4. verify the changed behavior;
5. update this register and the continuous development ledger;
6. leave unresolved items explicitly marked PENDING/PLANNED rather than converting them to PASS by inference.

## Legal/security boundary

Security records document engineering controls and provenance evidence. They do not determine copyright ownership, legal title, or jurisdiction-specific registration.
