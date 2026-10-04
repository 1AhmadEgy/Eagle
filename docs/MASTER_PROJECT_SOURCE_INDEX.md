# Eagle — Master Project Source & Integration Index

**Owner / Project authority:** Ahmad Ragab  
**Repository:** 1AhmadEgy/Eagle  
**Integration date:** 2026-10-02  
**Latest audit update:** 2026-10-04

## Purpose

Central coordination reference for project documentation, architecture, security, engineering workflow, research, execution, verification, application/test artifacts, historical inputs, and external component due diligence.

## Integrated source sets

| Source set | Source | Count | Role |
|---|---|---:|---|
| Engineering/security/documentation automation | testlab/category-evidence | 31 | Engineering and governance layer |
| Project reference foundation | docs/project-reference-foundation | 17 | Architecture, requirements, research, decisions, readiness |
| Android/test bootstrap | testlab/bootstrap | 16 | Application skeleton, tests, TestLab and pre-push gate |
| Historical uploaded repository materials | Git commit 812389dc9a19c52ca8089397c96a09d46957336b | 44 | Preserved historical evidence |
| 2026-10-04 evidence audit | docs/14-audit | 3 new audit records | Fact check, architecture/reuse audit, component due diligence |

## Canonical coordination map

- docs/00-reference — project reference
- docs/01-decisions — decisions
- docs/01-research — research and due diligence
- docs/02-source-register — source registry
- docs/03-architecture — architecture
- docs/04-security — security baseline
- docs/05-engineering — engineering workflow
- docs/06-execution — execution matrices
- docs/07-verification — verification
- docs/08-status — status
- docs/09-requirements — traceability
- docs/10-history — historical register
- docs/11-gaps — gaps
- docs/12-readiness — readiness
- docs/13-execution — continuation plan
- docs/14-audit — evidence, fact checking, reuse and component due diligence
- docs/legal — legal and provenance controls
- docs/provenance — machine-readable provenance
- docs/research — deep-research outputs
- docs/testing — testing gates
- archive/chatgpt-historical — preserved historical inputs when separately materialized

## 2026-10-04 audit artifacts

- docs/14-audit/REPOSITORY_FACT_CHECK_2026-10-04.md
- docs/14-audit/EVIDENCE_ARCHITECTURE_REUSE_AUDIT_2026-10-04.md
- docs/14-audit/COMPONENT_DUE_DILIGENCE_2026-10-04.md
- docs/14-audit/REUSE_FIRST_COMPONENT_REGISTER.md

These documents are the canonical record for the current repository fact check and the first reuse-first component survey.

## Merge policy

Overlapping engineering files are represented by the integrated testlab/category-evidence version. Unique reference-foundation and Android/test artifacts are retained. Historical materials remain traceable to their original Git commit/blob. New audit artifacts are linked from the master index rather than silently replacing historical records.

## Security

Historical ZIP/PDF/DOCX files are preserved but not trusted or executed automatically. Content inspection, independent SHA-256 hashing, duplicate detection, and security review are required before promoting an artifact into runtime or canonical implementation use.

## ChatGPT attachment boundary

The connected repository can prove files committed to GitHub. It does not provide a complete cross-conversation inventory of ChatGPT-only attachments that never entered GitHub. Such items remain pending intake rather than being invented or misattributed.

## Owner / team use

Ahmad Ragab is the coordination authority for the master source. All members should use this index, the architecture/reference documents, provenance register, verification registers, and the 2026-10-04 audit artifacts as the shared coordination baseline.
