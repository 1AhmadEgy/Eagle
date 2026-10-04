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
        require(approvedBy.length <= MAX_ACTOR_LENGTH) { "approvedBy is too long" }
        require(note.length <= MAX_NOTE_CHARS) { "note is too long" }
    }
}

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
        require(id.length <= MAX_ID_LENGTH) { "id is too long" }
        require(providerId.isNotBlank()) { "providerId must not be blank" }
        require(providerId.length <= MAX_ID_LENGTH) { "providerId is too long" }
        require(providerVersion.isNotBlank()) { "providerVersion must not be blank" }
        require(providerVersion.length <= MAX_VERSION_LENGTH) { "providerVersion is too long" }
        require(codeHash.isNotBlank() && codeHash.length <= MAX_HASH_CHARS) {
            "codeHash is invalid"
        }
        require(promptHash.isNotBlank() && promptHash.length <= MAX_HASH_CHARS) {
            "promptHash is invalid"
        }
        require(rawOutput.isNotBlank()) { "rawOutput must not be blank" }
        require(rawOutput.length <= MAX_RAW_OUTPUT_CHARS) { "rawOutput is too large" }
        require(recordHash.isNotBlank() && recordHash.length <= MAX_HASH_CHARS) {
            "recordHash is invalid"
        }
        require(regressionTestIds.size <= MAX_REGRESSION_TESTS) {
            "too many regression test identifiers"
        }
        require(regressionTestIds.none { it.isBlank() || it.length > MAX_ID_LENGTH }) {
            "invalid regression test identifier"
        }
        require(buildId == null || buildId.length <= MAX_ID_LENGTH)
        require(previousHash == null || previousHash.length <= MAX_HASH_CHARS)
        require(signature == null || signature.length <= MAX_SIGNATURE_CHARS)
    }

    companion object {
        const val MAX_ID_LENGTH = 128
        const val MAX_VERSION_LENGTH = 128
        const val MAX_HASH_CHARS = 256
        const val MAX_RAW_OUTPUT_CHARS = 65_536
        const val MAX_REGRESSION_TESTS = 128
        const val MAX_SIGNATURE_CHARS = 512
        const val MAX_ACTOR_LENGTH = 128
        const val MAX_NOTE_CHARS = 4_096
    }
}
