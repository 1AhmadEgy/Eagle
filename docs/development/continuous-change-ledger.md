# Eagle Continuous Development & Repair Ledger

This ledger records significant security, engineering, CI/CD, provenance, and documentation changes made to Eagle.

## Recording rules

- Record the change after implementation, not before.
- Record the exact branch/ref and commit SHA when known.
- Link each change to its pull request when applicable.
- Never record secrets, API keys, tokens, private keys, or sensitive personal data.
- Distinguish verified facts from planned work.
- Do not mark a Test Lab category PASS without category-specific evidence.
- Do not treat Git history as a substitute for legal registration.
- Keep the project ownership intent as `PRIVATE / ALL RIGHTS RESERVED`; do not add an open-source license without explicit owner authorization.

## Change ledger

| 2026-10-04 | Repository consistency / Architecture | Corrected active Android module documentation from androidApp/ to app/; added machine-readable repository state and a deterministic consistency gate; wired it into local verification and CI. | IN_REVIEW | PR #54; validation runs 37210145630 and 37210391735 exposed and were used to correct gate defects; current verification branch implementation/repository-consistency-v1 |
| 2026-10-04 | Protocol/Security Research | Added protocol/component selection matrix covering Signal/libsignal, vodozemac, OpenMLS, Noise, and explicit rejection of a custom cryptographic protocol. | CONFIRMED | docs/01-research/PROTOCOL_SECURITY_SELECTION_MATRIX_V0.1.md; external research captured 2026-10-04 |
| 2026-10-04 | Supply chain | Pinned Test Lab checkout/setup-java/setup-gradle actions to immutable full commit SHAs and extended the repository consistency gate to enforce full-SHA refs for external actions and reusable workflows. | IN_REVIEW | PR #54; current upstream releases verified 2026-10-04; exact pins stored in docs/03-architecture/REPOSITORY_STATE_V1.json |

| Date | Area | Change | Status | Evidence |
|---|---|---|---|---|
| 2026-10-01 | Foundation | Established secure V1 execution/reference baseline and reviewable PR workflow. | CONFIRMED | PR #2, PR #1 |
| 2026-10-01 | CI/Test Lab | Added repository verification, Test Lab status model, guarded OpenCode/DeepSeek repair workflow, and specialized read-only/repair/gatekeeper agents. | CONFIRMED | PR #3 |
| 2026-10-01 | Provenance/Legal | Added continuous private-ownership documentation agent, machine-readable provenance schema, deterministic exact-SHA Test Lab linkage, and offline provenance validation. | CONFIRMED | PR #4 |
| 2026-10-01 | Supply chain | Pinned GitHub Actions to full commit SHAs, upgraded gitleaks action to v3, and replaced mutable OpenCode `@latest` with a reviewed immutable snapshot. | CONFIRMED | PR #5, branch `security/actions-supply-chain-hardening` |
| 2026-10-01 | Documentation | Added this durable development/repair ledger and a security-control register so future repairs and improvements remain traceable. | IN_REVIEW | Branch `docs/continuous-development-audit` |
| 2026-10-01 | Security verification | Added a standard-library static gate that checks full-SHA workflow actions, explicit permissions, and AI-repair deny boundaries before project verification. | VERIFIED | PR #8, commit `84cc0318979cc62b28bbbb6d30a0841d26898c34`; CI run `36926005797` completed successfully |
| 2026-10-01 | Verification evidence | Recorded the successful CI execution for the PR #8 security-policy gate. The successful run confirms the workflow reached a completed/successful conclusion for the tested commit; it does not mark the category-specific Test Lab suite as PASS. | CONFIRMED | CI run `36926005797`, commit `84cc0318979cc62b28bbbb6d30a0841d26898c34` |
| 2026-10-01 | Deep Research | Added a repository-specific Deep Research Executive Summary covering security controls, AI repair boundaries, provenance, SLSA alignment, Test Lab maturity, ownership/visibility discrepancy, risks, and staged execution priorities. | CONFIRMED | Commit `a45a78bae52de9b3faae45c6660ba30c8c4d51d9`; `docs/research/deep-research-executive-summary-2026-10-01.md` |
| 2026-10-01 | Security Research Register | Updated the security/research register with current GitHub provenance guidance, SLSA alignment status, Test Lab evidence discipline, and AI-repair boundary traceability. | CONFIRMED | Commit `b007f0fefdb1225ee42b902ca0bad2dedee42459` |
| 2026-10-02 | Continuity / Provenance | Added chat-deletion continuity snapshot, session artifact manifest, recovery index, and updated ChatGPT attachment intake with independent SHA-256 identities for the two artifacts available in this session. | CONFIRMED | Commits `99022fac2ff5423295a644ee7fcdaf9783f71b7a`, `9dcae335bdc1387a78870ef9e03bb2c1f80496f8`, `1dd1475efb1b917814abcb8bb8cbb6b5386adcde`, `adf4b84e5ac1ac5ffa3c39b83e8de3a05d9a8436`, `b336f8dbda88b6b3e13b5d6d70879ada85dd5040` |

## Current security posture

### Confirmed controls

- Workflow permissions are explicitly declared and minimized where practical.
- Third-party workflow actions in the hardened workflows are referenced by full commit SHA.
- Checkout credential persistence is disabled.
- The privileged `workflow_run` repair workflow is restricted to a trusted push on `implementation/v1-foundation`.
- The AI repair boundary prohibits automatic modification of workflows, secrets, credentials, and deployment configuration.
- The documentation agent is read-only.
- CI/Test Lab evidence is bound to an exact commit SHA before provenance status is derived.
- Provenance validation is performed locally before the documentation artifact is uploaded.
- Automatic merge is not enabled for the repair pipeline.
- The static security-policy gate has a successful CI execution on the PR #8 head commit.
- The Deep Research report and security research register are now part of the repository audit trail.
- Ownership remains documented as `PRIVATE / ALL RIGHTS RESERVED`, while the legal holder remains explicitly `UNCONFIRMED`.

### Known gaps requiring owner-controlled or future work

- Repository visibility is currently PUBLIC while project documentation specifies PRIVATE / ALL RIGHTS RESERVED. This is a repository setting and requires an explicit owner-controlled decision/change.
- The Test Lab currently has a baseline status model, but several categories remain PENDING until category-specific tests are implemented and independently evidenced.
- Artifact attestations should be introduced for actual release artifacts/build outputs when the project has release artifacts.
- Branch protection/rulesets and required reviews should be verified at the repository settings level before treating merge governance as enforced.
- The DeepSeek secret must exist only as a GitHub Actions secret; it must never be committed or emitted in logs.
- Formal SLSA level claims remain unmade until the applicable build and provenance requirements are independently evidenced.

## Evidence policy

Every future material change should add an entry with:

1. exact commit SHA;
2. source and target refs;
3. pull request number/URL when applicable;
4. affected files;
5. verification commands/checks and their results;
6. security impact;
7. remaining limitations or PENDING categories.

A change is not considered fully documented merely because a commit exists. The record should explain what changed, why, how it was verified, and what remains unresolved.
