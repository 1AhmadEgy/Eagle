# Eagle / PrivateMesh --- Unified Documentation Consolidation Register

**Baseline date:** 2026-09-30\
**Status:** Draft register for controlled consolidation; not an
implementation or production approval.

## 1. Authority and status

Use the following authority chain for every requirement or claim:

Source → Claim → Requirement → ADR → Threat/Control → Specification →
Implementation → Test → Result → Evidence → Independent Verification →
Release Decision → Artifact → Runtime Monitoring → Drift.

A document's newer date or version does not, by itself, override an
approved ADR or verified evidence. Preserve superseded files in the
historical archive.

## 2. Consolidation decisions

  ----------------------------------------------------------------------------
  ID                Topic             Consolidation rule  Required evidence /
                                                          closure
  ----------------- ----------------- ------------------- --------------------
  DOC-001           Master index      Maintain one        Cross-reference
                                      navigation index;   check against every
                                      indexes inside      included document.
                                      historical packages 
                                      are archival.       

  DOC-002           Architecture      Keep product        Approved
                    authority         constitution and    architecture ADRs
                                      master architecture and consistency
                                      as the architecture review.
                                      baseline;           
                                      subordinate         
                                      specifications      
                                      define component    
                                      behavior.           

  DOC-003           Open decisions    Keep all eight      Decision owner,
                    OI-001--OI-008    explicitly open     alternatives,
                                      until each has an   rationale,
                                      approved ADR and    consequences,
                                      supporting          approval, linked
                                      evidence.           tests.

  DOC-004           Security          Maintain one        Negative tests and
                    invariants        canonical invariant security review
                                      catalogue and link  evidence.
                                      each invariant to   
                                      threat, control,    
                                      implementation, and 
                                      tests.              

  DOC-005           Test and evidence Keep test design    Reproducible run
                                      separate from       logs, artifact
                                      executed test       hashes, environment
                                      records; never      and reviewer.
                                      label planned tests 
                                      as passed.          

  DOC-006           Production        Keep release state  Gate-by-gate signed
                    readiness         BLOCKED until       evidence and formal
                                      source, build,      release decision.
                                      test, independent   
                                      verification, and   
                                      external security   
                                      audit evidence are  
                                      established.        

  DOC-007           Research adoption Research and        Adoption record,
                                      suggested tools     threat/maintenance
                                      remain              assessment, ADR
                                      candidate-only      where architecture
                                      until an explicit   changes.
                                      adoption decision.  

  DOC-008           Team work         Treat team packages Version links, named
                    packages          as assignments      owner, acceptance
                                      derived from        criteria, handoff
                                      canonical           record.
                                      specifications, not 
                                      independent sources 
                                      of truth.           

  DOC-009           Duplicate         Preserve historical Manifest comparison
                    archives          ZIPs and identify   and recorded
                                      canonical/current   supersession links.
                                      package in the      
                                      index; do not       
                                      delete originals    
                                      during              
                                      consolidation.      

  DOC-010           Terminology and   Use a controlled    Terminology sweep
                    status            vocabulary          and status-owner
                                      consistently:       review.
                                      VERIFIED, ACCEPTED, 
                                      PROPOSED,           
                                      UNDER_REVIEW,       
                                      UNKNOWN, DEFERRED,  
                                      REJECTED,           
                                      DEPRECATED; use     
                                      BLOCKED for gate    
                                      state where         
                                      applicable.         
  ----------------------------------------------------------------------------

## 3. Open decision register

  ---------------------------------------------------------------------------------
  ID                Decision area            Current handling  Closure condition
  ----------------- ------------------------ ----------------- --------------------
  OI-001            Exact Signal             Open; do not      Freeze exact
                    implementation/version   assume a version. implementation and
                                                               revision; record
                                                               security and
                                                               compatibility
                                                               review.

  OI-002            PQXDH integration        Open; no implied  Approved profile,
                    profile                  approval.         vectors,
                                                               interoperability
                                                               tests,
                                                               implementation
                                                               evidence.

  OI-003            V1 transport boundary    Open until        Define transport,
                                             explicit ADR.     trust boundary,
                                                               failure behavior,
                                                               and tested
                                                               interface.

  OI-004            Server ciphertext        Open; no silent   Approved
                    retention                selection of      retention/deletion
                                             retention policy. policy and verified
                                                               server behavior.

  OI-005            Identity/device trust    Open until state  State transitions,
                    states                   model is          authorization rules,
                                             approved.         and adversarial
                                                               tests.

  OI-006            Device linking           Open until        One-time pairing
                                             protocol and      lifecycle,
                                             consent model are trusted-device
                                             approved.         approval, replay and
                                                               revocation tests.

  OI-007            Recovery protocol        Open; account     Threat-reviewed
                                             recovery and data recovery flows and
                                             recovery remain   recovery-failure
                                             distinct          tests.
                                             concerns.         

  OI-008            Deletion guarantees      Open; avoid       Layer-specific
                                             absolute deletion deletion semantics,
                                             claims without    retention
                                             proof.            boundaries, and
                                                               verification
                                                               evidence.
  ---------------------------------------------------------------------------------

## 4. Gate status snapshot

  ------------------------------------------------------------------------------
  Gate              Area              Status from       Next evidence
                                      documentation     
                                      baseline          
  ----------------- ----------------- ----------------- ------------------------
  G0                Source integrity  Partial /         Actual source tree,
                                      documentation     frozen revisions and
                                      ready             dependency set.

  G1                Architecture      Open              Close
                    freeze                              architecture-affecting
                                                        ADRs.

  G2                Crypto freeze     Open / blocked    Exact crypto profile,
                                                        vectors,
                                                        interoperability and
                                                        implementation evidence.

  G3                Mobile security   Open /            Real platform builds,
                                      documentation     keystore/attestation
                                      ready             tests and device-matrix
                                                        results.

  G4                Observability     Documentation     Instrumented app
                    safety            ready / not       evidence and negative
                                      verified          tests for prohibited
                                                        data.

  G5                Supply chain      Documentation     Actual-build SBOM,
                                      ready / not       provenance, signatures
                                      verified          and reproducibility
                                                        evidence.

  G6                Security          Open / blocked    Complete test results
                    verification                        and independent
                                                        verification.

  G7                Production        Blocked           Closure of G1--G6 and
                                                        formal release evidence.
  ------------------------------------------------------------------------------

## 5. Document structure for the unified package

-   `00_CONTROL`: master index, status semantics, provenance, conflict
    register, change log.
-   `01_PRODUCT_REQUIREMENTS`: constitution, scope, requirements and
    acceptance criteria.
-   `02_ARCHITECTURE_SPECIFICATIONS`: architecture and component
    specifications.
-   `03_SECURITY_PRIVACY`: threat model, invariants, abuse cases,
    privacy and deletion.
-   `04_PROTOCOL_CRYPTO`: protocol decisions, crypto profile, vectors
    and conformance.
-   `05_IMPLEMENTATION_CONTRACTS`: API, error model, execution order and
    intake gate.
-   `06_TESTING_EVIDENCE`: test architecture, scenarios, execution
    records and evidence graph.
-   `07_PLATFORM_INTEGRATION`: Android, iOS, networking, storage and
    recovery integration.
-   `08_OPERATIONS_RELEASE`: observability, incident response, rollback,
    supply chain and release gates.
-   `09_TEAM_HANDOFFS`: team assignments, RACI, review and handoff
    checklists.
-   `10_RESEARCH_REFERENCES`: source registry, research digest and
    adoption decisions.
-   `90_ARCHIVE`: immutable historical versions and supersession map.

## 6. Immediate action list

1.  Freeze the candidate baseline and record its manifest/hash without
    overwriting historical packages.
2.  Build a file-level inventory mapping every source document to one
    canonical destination.
3.  Compare overlapping specifications and ADRs; log conflicts rather
    than silently merging them.
4.  Resolve ownership and status for OI-001--OI-008.
5.  Populate the documentation completeness matrix per component.
6.  Validate internal links, document identifiers, terminology, version
    references and manifest entries.
7.  Publish a unified package only after review of the consolidation
    diff.
8.  Keep implementation and production gates blocked until actual
    evidence is supplied and reviewed.

## 7. Completion criteria

Documentation consolidation is complete only when each included document
has a unique identifier, owner, version, status, canonical location,
authority level, linked requirements/ADRs, and review state; duplicate
or superseded files are mapped to the archive; conflicts have explicit
dispositions; and the package manifest matches its contents.

**Important:** This register organizes the work and records the
currently documented state. It does not claim that source code,
executable tests, security verification, or production readiness have
been completed.
