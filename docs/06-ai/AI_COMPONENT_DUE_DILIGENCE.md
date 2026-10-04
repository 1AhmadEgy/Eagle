# Eagle — AI / ML Component Due Diligence

Status: RESEARCH BASELINE — NO COMPONENT APPROVED FOR PRODUCTION

## Selection rule
Requirement -> architecture fit -> security history -> exact version -> license -> dependency review -> test evidence -> operational fit -> exit strategy -> owner approval.

## Candidate matrix
| Component | Best role | Android/on-device | Recommendation |
|---|---|---|---|
| ONNX Runtime Mobile | General model inference | Yes | Strong candidate once a model exists |
| ExecuTorch | PyTorch model inference | Yes | Strong candidate for PyTorch-origin models |
| LiteRT / Google AI Edge | Google on-device inference | Yes | Candidate; compare after requirements |
| Tribuo | JVM ML, anomaly detection, evaluation | Requires Android experiment | Strong research/evaluation candidate |
| scikit-learn | Offline training and benchmarking | No | Strong offline reference, not Android runtime |
| Custom Kotlin | Small deterministic statistics | Yes | Appropriate for transparent formulas |

## Algorithm candidates
### Z-score
z = (x - mean) / standard_deviation
Cheap and auditable; requires robust handling of low variance and baseline drift.
### EWMA
s_t = alpha*x_t + (1-alpha)*s_(t-1)
Good for temporal drift with tiny state and constant-time updates.
### Weighted anomaly score
A = sum(w_i * normalized_deviation_i)
Weights must be versioned and tested; do not call them learned without training evidence.
### Isolation Forest
Useful for nonlinear outliers after a representative feature dataset exists. Training can remain offline and a compact model may later be deployed through an approved runtime.
### One-Class SVM
Useful when normal behavior is well characterized. Sensitive to feature scaling and hyperparameters; calibration and adversarial evaluation are mandatory.
### Clustering
K-Means/HDBSCAN can discover populations but must produce evidence rather than direct security decisions.
### Sequence models
LSTM/Transformer-style models are deferred until event sequences, datasets, evaluation, provenance, resource budgets, and adversarial tests exist.
### LLM explanation
An LLM may explain an already-computed structured assessment. It must not become the authority for cryptography, authentication, authorization, key handling, or protocol transitions.

## Mature implementation evidence
ONNX Runtime Mobile: official documentation supports Android Java/Kotlin deployment and on-device inference, with CPU, XNNPACK and NNAPI execution options. Models must fit device storage and memory.
Source: https://onnxruntime.ai/docs/tutorials/mobile/
ExecuTorch: official Android documentation provides Java/Kotlin bindings through an AAR, Maven Central distribution, XNNPACK CPU backend and additional Android backends. Stable releases are preferred over snapshots.
Source: https://docs.pytorch.org/executorch/stable/using-executorch-android.html
Tribuo: Oracle Labs' Java ML library provides anomaly detection, clustering, evaluation, provenance/model-card support and ONNX model support. Its anomaly infrastructure includes one-class SVM through LibSVM/LibLinear.
Source: https://tribuo.org/learn/4.3/docs/
scikit-learn: mature reference implementations include Isolation Forest and One-Class SVM and are appropriate for offline experiments and evaluation, not as the Android runtime.
Sources: https://scikit-learn.org/stable/auto_examples/ensemble/plot_isolation_forest.html and https://scikit-learn.org/stable/modules/generated/sklearn.svm.OneClassSVM.html

## Preferred initial stack
1. No ML dependency in the product yet.
2. Implement transparent deterministic feature contracts first.
3. Use Python/scikit-learn for offline evaluation where useful.
4. Select ONNX Runtime Mobile or ExecuTorch only after a real model exists and the target Android budget is known.
5. Consider Tribuo for JVM-side evaluation/training utilities, not as an automatic mobile dependency.
6. Keep LLMs outside the enforcement boundary.

## Required model acceptance evidence
Dataset identity/hash; feature schema version; training code/version; model artifact hash; exact runtime version; license/provenance; task-appropriate metrics; false-positive/false-negative analysis; adversarial tests; latency; peak memory; battery/energy impact; rollback path; deterministic fallback.

## Non-approval rule
This document is a research register, not authorization to add a dependency. Approval requires an ADR plus reproducible verification evidence.