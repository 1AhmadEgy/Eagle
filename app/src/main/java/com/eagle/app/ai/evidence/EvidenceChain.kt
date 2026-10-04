package com.eagle.app.ai.evidence

data class ChainVerificationResult(
    val isValid: Boolean,
    val firstBrokenIndex: Int?,
    val reason: String?
)

/**
 * Ordered logical evidence chain.
 *
 * Persistence is deliberately left to a later storage adapter. The chain contract
 * is independent from JSON, Room, or another storage technology.
 */
class EvidenceChain(
    private val hasher: EvidenceHasher = EvidenceHasher()
) {

    fun verifyChain(records: List<EvidenceRecord>): ChainVerificationResult {
        var expectedPrevious: String? = null

        records.forEachIndexed { index, record ->
            if (record.previousHash != expectedPrevious) {
                return ChainVerificationResult(
                    isValid = false,
                    firstBrokenIndex = index,
                    reason = "previousHash mismatch at index $index"
                )
            }

            val recomputed = hasher.hashRecord(record)
            if (recomputed != record.recordHash) {
                return ChainVerificationResult(
                    isValid = false,
                    firstBrokenIndex = index,
                    reason = "recordHash mismatch at index $index"
                )
            }

            expectedPrevious = record.recordHash
        }

        return ChainVerificationResult(
            isValid = true,
            firstBrokenIndex = null,
            reason = null
        )
    }
}
