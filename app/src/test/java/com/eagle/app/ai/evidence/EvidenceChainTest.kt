package com.eagle.app.ai.evidence

import com.eagle.app.ai.provider.TaskType
import java.time.Instant
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class EvidenceChainTest {

    private val hasher = EvidenceHasher()

    @Test
    fun validGenesisAndLinkedRecordVerify() {
        val first = record("evidence-1", null)
        val second = record("evidence-2", first.recordHash)

        val result = EvidenceChain(hasher).verifyChain(listOf(first, second))

        assertTrue(result.isValid)
        assertEquals(null, result.firstBrokenIndex)
        assertEquals(null, result.reason)
    }

    @Test
    fun tamperingWithMiddleRecordFailsAtMiddleIndex() {
        val first = record("evidence-1", null)
        val second = record("evidence-2", first.recordHash)
        val third = record("evidence-3", second.recordHash)

        val tampered = second.copy(rawOutput = "tampered")

        val result = EvidenceChain(hasher).verifyChain(listOf(first, tampered, third))

        assertEquals(false, result.isValid)
        assertEquals(1, result.firstBrokenIndex)
        assertEquals("recordHash mismatch at index 1", result.reason)
    }

    @Test
    fun brokenLinkIsReportedAtFirstDivergence() {
        val first = record("evidence-1", null)
        val second = record("evidence-2", "wrong")

        val result = EvidenceChain(hasher).verifyChain(listOf(first, second))

        assertEquals(false, result.isValid)
        assertEquals(1, result.firstBrokenIndex)
        assertEquals("previousHash mismatch at index 1", result.reason)
    }

    @Test
    fun tamperingWithEarlierRecordIsReportedAtEarlierIndex() {
        val first = record("evidence-1", null)
        val second = record("evidence-2", first.recordHash)
        val third = record("evidence-3", second.recordHash)

        val tampered = first.copy(providerVersion = "tampered")

        val result = EvidenceChain(hasher).verifyChain(listOf(tampered, second, third))

        assertEquals(false, result.isValid)
        assertEquals(0, result.firstBrokenIndex)
        assertEquals("recordHash mismatch at index 0", result.reason)
    }

    @Test
    fun emptyChainIsValid() {
        val result = EvidenceChain(hasher).verifyChain(emptyList())

        assertTrue(result.isValid)
    }

    private fun record(id: String, previousHash: String?): EvidenceRecord {
        return EvidenceRecord(
            id = id,
            timestamp = Instant.EPOCH,
            taskType = TaskType.CODE_REVIEW,
            providerId = "provider-a",
            providerVersion = "v1",
            codeHash = "sha256:code",
            promptHash = "sha256:prompt",
            rawOutput = "output-$id",
            parsedFinding = null,
            reproductionTestId = null,
            sastResultId = null,
            regressionTestIds = emptyList(),
            buildId = null,
            humanApproval = null,
            previousHash = previousHash,
            recordHash = "placeholder"
        ).withComputedHash(hasher)
    }
}
