# Archive Knowledge Integration

The external EDA imports supplied internal ZIP collections recursively and deduplicates content by cryptographic digest before building a knowledge index.

The imported material contributes:

- scenario families and compound scenarios;
- AI modes and agent roles;
- experiment/lab patterns;
- failure and recovery patterns;
- coverage and regression curation;
- research/reuse guidance;
- learning and training/evaluation patterns;
- advanced research tracks such as generative chaos, Digital Twin, PQC, federated learning and TEE/HRoT.

Imported material is treated as untrusted engineering input. It does not override Eagle architecture decisions, security invariants, repository policy, tests or Release Gate.

The canonical implementation is the external EDA package; Eagle contains only repository-facing contracts and approved project documentation.
