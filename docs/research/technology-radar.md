# Eagle Technology & Research Radar

Date: 2026-10-04
Status: Active evaluation register
Scope: reusable, tested, mature external technology that can accelerate Eagle without weakening Eagle-owned contracts.

## 1. Decision rule

Eagle does not adopt a library merely because it is popular.

Every candidate is evaluated against:

1. maintenance/activity;
2. license compatibility;
3. test and CI evidence;
4. security/provenance history;
5. benchmark or real-world evidence;
6. dependency and build-system cost;
7. lock-in and migration cost;
8. ability to sit behind an Eagle-owned contract;
9. offline/local operation where required;
10. failure behavior and security boundary.

Decisions:

- ADOPT — approved for direct use after a concrete integration slice.
- ADAPTER — usable behind an Eagle-owned interface; external API must not leak into core contracts.
- REFERENCE — architecture/pattern may be copied or studied; dependency is not required.
- PROTOTYPE — integration experiment only.
- WATCH — promising but evidence is insufficient.
- REJECT — unacceptable security, maintenance, licensing, or architectural cost.

## 2. Initial radar

| Area | Candidate | Eagle disposition | Intended use |
|---|---|---|---|
| Agent orchestration | LangGraph | ADAPTER/PROTOTYPE | orchestration outside eagle-core |
| Agent orchestration | Microsoft AutoGen | PROTOTYPE | multi-agent experiments |
| RAG/indexing | LlamaIndex | ADAPTER/PROTOTYPE | retrieval adapter and experiments |
| RAG/indexing | Haystack | PROTOTYPE | retrieval/evaluation pipelines |
| Memory | Mem0 | ADAPTER/PROTOTYPE | optional long-term memory backend |
| Memory | Letta/MemGPT | REFERENCE/PROTOTYPE | stateful memory architecture |
| Temporal/graph memory | Graphiti | ADAPTER/PROTOTYPE | temporal knowledge graph backend |
| Knowledge graph | Neo4j | PROTOTYPE | graph-backed retrieval where justified |
| Vector DB | Qdrant | ADAPTER/PROTOTYPE | vector retrieval backend |
| Vector DB | pgvector | ADAPTER/PROTOTYPE | Postgres-backed vector retrieval |
| Vector DB | Milvus | WATCH/PROTOTYPE | large-scale retrieval option |
| Embeddings | Sentence Transformers | ADAPTER | embedding generation |
| Local inference | llama.cpp | ADAPTER/PROTOTYPE | local GGUF inference |
| Local inference | Ollama | PROTOTYPE | developer/local model runtime |
| Local inference | vLLM | ADAPTER/PROTOTYPE | server-side high-throughput inference |
| Model ecosystem | Hugging Face Transformers | ADAPTER | model loading/training/evaluation integration |
| Reflection | Reflexion | REFERENCE | self-reflection loop design |
| Refinement | Self-Refine | REFERENCE | iterative output refinement |
| Reasoning | ReAct | REFERENCE | tool-use/reasoning policy |
| Planning | Tree of Thoughts | REFERENCE | search/planning experiments |
| Search/planning | LATS | REFERENCE | tree-search agent planning |
| Skill acquisition | Voyager | REFERENCE | reusable skill library pattern |
| Evaluation | SWE-bench Verified | EVALUATE | coding-agent benchmark |
| Evaluation | GAIA | EVALUATE | general assistant benchmark |
| Evaluation | WebArena | EVALUATE | web-agent benchmark |
| Evaluation | OSWorld | EVALUATE | computer-use benchmark |
| Evaluation | tau-bench / tau2-bench | EVALUATE | tool-use/customer-service evaluation |
| Evaluation | BFCL | EVALUATE | function/tool calling evaluation |
| Evaluation | Terminal-Bench | EVALUATE | terminal-agent evaluation |
| Observability | OpenTelemetry | ADOPT/ADAPTER | traces, metrics and evaluation telemetry |
| Supply chain | cargo-audit | ADOPT | Rust dependency vulnerability checks |
| Supply chain | cargo-deny | ADOPT | dependency/license/advisory policy |
| Provenance | GitHub artifact attestations / SLSA | ADOPT | build provenance and release evidence |

## 3. Architecture rule

Third-party frameworks must not become the definition of Eagle's security or domain contracts.

Preferred direction:

Eagle contract -> adapter -> external implementation

Not:

external framework -> Eagle core contract

In particular:

- eagle-core remains framework-independent;
- eagle-core-reference remains non-production;
- model runtimes, vector stores, agent frameworks and memory systems belong above the core boundary;
- cryptographic/security decisions remain Eagle-owned and Rust-owned;
- generated FFI remains a build artifact rather than a second domain implementation.

## 4. Research families to harvest

### Reasoning and tool use

- ReAct
- Reflexion
- Self-Refine
- Tree of Thoughts
- LATS
- Self-Discover
- Chain-of-Verification

Use as algorithms and testable policies before adding framework dependencies.

### Memory

- MemGPT / Letta
- Generative Agents
- Mem0
- Graphiti
- Cognee

Separate short-term state, durable memory, semantic retrieval and temporal knowledge instead of treating all memory as one vector store.

### Skill learning

Voyager's automatic curriculum, executable skill library and feedback loop are relevant as a reference architecture for Eagle's future skill subsystem. This is a research pattern, not an authorization to import Minecraft-specific code.

### Retrieval

Evaluate hybrid retrieval, reranking, metadata filtering, temporal retrieval and graph retrieval. A vector database alone is not considered a complete memory architecture.

## 5. Evidence policy for each candidate

Before ADOPT or ADAPTER, add a candidate record containing:

- upstream repository;
- official documentation;
- license;
- release/version;
- commit or release provenance;
- language/runtime;
- tests and CI;
- security/advisory history;
- benchmark evidence;
- known production use, where independently verifiable;
- dependency footprint;
- integration boundary;
- failure modes;
- rollback plan;
- exact Eagle slice using it.

No candidate becomes an Eagle production dependency from a name-only recommendation.

## 6. Current security/provenance priority

Before broad dependency expansion, Eagle should close the repository supply-chain gates:

1. YAML-aware workflow action pinning verifier;
2. all moving GitHub Action tags removed from production workflows;
3. dependency vulnerability/license policy;
4. reproducible dependency inventory;
5. artifact provenance/attestation for released binaries;
6. SBOM attached to releasable artifacts.

GitHub documents artifact attestations as cryptographically signed provenance claims linking an artifact to its repository, workflow and commit; attestations can also carry SBOMs. They improve provenance but do not by themselves prove that the software is secure.

GitHub dependency review can detect vulnerable dependency changes before merge and can enforce severity/license policies when configured as a required check.

## 7. Research sources

Primary/official sources should be recorded alongside every research item. Secondary comparisons are discovery aids only and are not sufficient for an ADOPT decision.

Initial external research performed on 2026-10-04 covered agent benchmarks, agent frameworks, memory systems, local inference runtimes, retrieval systems and self-improvement research.

## 8. Next harvesting slices

- R1: agent orchestration and state machines
- R2: memory/retrieval backends
- R3: local and hosted model runtimes
- R4: evaluation/benchmark harness
- R5: observability and tracing
- R6: supply-chain/SBOM/provenance
- R7: sandboxing and tool-execution isolation
- R8: security protocol libraries and formal verification tooling

Each slice ends with ADOPT, ADAPTER, REFERENCE, PROTOTYPE, WATCH, or REJECT decisions backed by repository evidence.
