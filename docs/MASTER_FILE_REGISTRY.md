# Eagle — Master File Registry

**Status:** Baseline established  
**Repository:** 1AhmadEgy/Eagle  
**Default branch:** main  
**Documentation branch:** docs/project-documentation-baseline

## Purpose

Canonical inventory and provenance ledger for project files, changes, decisions, tests, security evidence, and releases.

## Source-of-truth policy

GitHub is the durable project record. Chat conversations are working sessions only and must not be treated as the canonical archive after migration.

Every retained artifact must have a stable repository path, provenance/source reference, classification, version/status, owner or responsible role, related change/commit/PR, validation evidence where applicable, and security classification.

## Required lifecycle

Inventory → Provenance → Classification → File Audit → Version Comparison → Canonical Reference → Requirements/Gaps Matrix → Correction → Implementation → Tests → Security Audit → Verification → Release Gate.

## Registry states

- DISCOVERED — located but not yet validated.
- VERIFIED — content and provenance checked.
- CANONICAL — authoritative version.
- SUPERSEDED — historical, no longer authoritative.
- QUARANTINED — isolated pending review.
- REJECTED — duplicate, invalid, corrupted, unauthorized, or excluded.
- ARCHIVED — retained for historical/audit reasons.

## Registry schema

| Field | Required | Description |
|---|---:|---|
| Registry ID | Yes | Stable identifier |
| Repository path | Yes | Canonical GitHub path |
| Artifact name | Yes | Human-readable name |
| Type | Yes | Code/config/docs/data/test/etc. |
| Provenance | Yes | Original source and acquisition context |
| Source reference | Yes | Chat/file/commit/PR/external source |
| Version | Yes | Version/date/commit |
| Status | Yes | Registry state |
| Canonical? | Yes | Yes/No |
| Owner | Yes | Responsible role/person |
| Related change | Yes | Commit/PR/issue |
| Validation | Yes | Test/review/evidence |
| Security class | Yes | Public/internal/confidential/secret |
| Notes | No | Exceptions and decisions |

## Current baseline

This file establishes the documentation control plane. It does **not** claim that every historical chat attachment has already been imported. Historical completeness requires explicit collection, provenance recording, duplicate/version comparison, and validation of each available artifact.
