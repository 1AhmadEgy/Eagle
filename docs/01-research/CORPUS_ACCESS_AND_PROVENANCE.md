# Eagle — Corpus Access & Provenance Record

**Purpose:** define how project corpus is consumed during engineering work without silently promoting historical material to canonical authority.

## Corpus layers

### 1. ChatGPT project conversations

The project conversation corpus is treated as historical/project evidence:

- requirements and intent;
- design discussions;
- architecture proposals;
- prior audits;
- role handoffs;
- unresolved questions;
- evidence summaries.

Conversation content does not become an approved technical decision by repetition. Decisions must be traceable to repository ADR/specification records.

### 2. Project file corpus

Documents supplied through the project/file repository are treated according to their provenance:

- source material;
- research;
- requirements;
- architecture;
- protocol/crypto reference;
- threat model;
- test evidence;
- operational evidence;
- superseded/duplicate material.

Where the runtime exposes only a file reference and not its contents, the artifact remains unverified until content can actually be inspected.

### 3. GitHub repository

`1AhmadEgy/Eagle` is the executable repository source of truth for:

- current branches and commits;
- canonical file state;
- implementation;
- tests;
- CI evidence;
- pull requests;
- repository-level provenance.

A historical document is not assumed current merely because it contains more detail.

### 4. External authoritative sources

Standards, RFCs, NIST publications, and official platform-security documentation are used to validate claims and technical constraints.

External research must be linked to the decision/specification that consumes it.

## Authority order

```text
Authoritative standards / official security docs
                ↓
Approved project ADRs
                ↓
Canonical project specifications
                ↓
Verified implementation
                ↓
Executable test evidence / audit
                ↓
Historical project documents
                ↓
Chat discussion / proposal
```

A conflict is recorded and reconciled; it is not silently resolved by preference.

## Identity & Trust corpus used

The Identity & Trust baseline was traced to:

- `archive/chatgpt-historical/ADR_Decision_Pack.md`
- `archive/chatgpt-historical/PRIVATE_MESH_MASTER_PROJECT_REFERENCE.md`
- `archive/chatgpt-historical/Eagle_PrivateMesh_Implementation_Blueprint_v1.0.md`
- `docs/03-architecture/PLATFORMS.md`
- `docs/03-architecture/adr/ADR-0008.md`
- `docs/03-architecture/adr/ADR-0009.md`
- `docs/03-architecture/adr/ADR-0010.md`

Current state observed during execution:

- ADR-0008: proposed / pending review.
- ADR-0009: proposed / pending review.
- ADR-0010: proposed / pending review.
- ADR-005/006/007: proposed / open in the historical decision pack.

## Provenance rule for generated implementation

Every generated security implementation must record:

- source baseline commit;
- applicable decision/ADR;
- specification path;
- tests/evidence;
- unresolved assumptions;
- release-gate state.

No generated artifact may claim adoption of a technology that remains only a candidate/proposal.

## P2P constraint

The project-level P2P-only requirement is recorded as an implementation constraint for this workstream. Historical server/relay alternatives remain research options until the project's transport decision is explicitly approved.

## Security rule

Corpus access must never be used to extract, reproduce, or expose private keys, credentials, recovery secrets, or other sensitive material. The engineering record contains metadata and design conclusions, not secret values.
