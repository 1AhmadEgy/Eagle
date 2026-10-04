package com.eagle.app.ai.evidence

import com.eagle.app.ai.findings.SecurityFinding
import com.eagle.app.ai.provider.TaskType
import java.time.Instant

data class HumanApproval(
    val approvedBy: String,
    val approvedAt: Instant,
    val decision: Decision,
    val note: String
) {
    enum class Decision {
        APPROVED,
        REJECTED
    }

    init {
        require(approvedBy.isNotBlank()) { "approvedBy must not be blank" }
        require(note.isNotBlank()) { "note must not be blank" }
    }
}

/**
 * Immutable evidence envelope for one AI-assisted verification step.
 *
 * rawOutput must contain only the final provider output required by the evidence policy.
 * Provider chain-of-thought, hidden reasoning, secrets, plaintext message contents, private
 * keys, and credentials must never be persisted here.
 *
 * recordHash and signature are produced/validated by the M2 evidence implementation.
 */
data class EvidenceRecord(
    val id: String,
    val timestamp: Instant,
    val taskType: TaskType,
    val providerId: String,
    val providerVersion: String,
    val codeHash: String,
    val promptHash: String,
    val rawOutput: String,
    val parsedFinding: SecurityFinding?,
    val reproductionTestId: String?,
    val sastResultId: String?,
    val regressionTestIds: List<String>,
    val buildId: String?,
    val humanApproval: HumanApproval?,
    val previousHash: String?,
    val recordHash: String,
    val signature: String? = null
) {
    init {
        require(id.isNotBlank()) { "id must not be blank" }
        require(providerId.isNotBlank()) { "providerId must not be blank" }
        require(providerVersion.isNotBlank()) { "providerVersion must not be blank" }
        require(codeHash.isNotBlank()) { "codeHash must not be blank" }
        require(promptHash.isNotBlank()) { "promptHash must not be blank" }
        require(rawOutput.isNotBlank()) { "rawOutput must not be blank" }
        require(recordHash.isNotBlank()) { "recordHash must not be blank" }
        require(regressionTestIds.none { it.isBlank() }) {
            "regressionTestIds cannot contain blank identifiers"
        }
    }
}
