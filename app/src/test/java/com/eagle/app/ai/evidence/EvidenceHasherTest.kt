package com.eagle.app.ai.evidence

import java.time.Instant
import com.eagle.app.ai.findings.FindingSeverity
import com.eagle.app.ai.findings.FindingStatus
import com.eagle.app.ai.provider.TaskType
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotEquals
import org.junit.Test

class EvidenceHasherTest {

    @Test
    fun sameRecordProducesSameHash() {
        val record = record()
        val hasher = EvidenceHasher()

        assertEquals(hasher.hashRecord(record), hasher.hashRecord(record))
    }

    @Test
    fun changingBoundFieldsChangesHash() {
        val hasher = EvidenceHasher()
        val base = record()

        val variants = listOf(
            base.copy(id = "evidence-2"),
            base.copy(timestamp = Instant.ofEpochSecond(1)),
            base.copy(providerVersion = "2026-other"),
            base.copy(rawOutput = "changed"),
            base.copy(reproductionTestId = "test-2"),
            base.copy(previousHash = "sha256:previous"),
            base.copy(regressionTestIds = listOf("reg-2")),
            base.copy(
                humanApproval = HumanApproval(
                    approvedBy = "reviewer",
                    approvedAt = Instant.EPOCH,
                    decision = HumanApproval.Decision.APPROVED,
                    note = "changed"
                )
            )
        )

        variants.forEach { changed ->
            assertNotEquals(hasher.hashRecord(base), hasher.hashRecord(changed))
        }
    }

    @Test
    fun changingNestedFindingChangesHash() {
        val hasher = EvidenceHasher()
        val finding = com.eagle.app.ai.findings.SecurityFinding(
            id = "finding-1",
            taskType = TaskType.CODE_REVIEW,
            severity = FindingSeverity.MEDIUM,
            status = FindingStatus.OPEN,
            title = "finding",
            description = "description",
            affectedPaths = setOf("a.kt"),
            createdAt = Instant.EPOCH,
            proposerProviderId = "provider"
        )
        val base = record(parsedFinding = finding)
        val changed = base.copy(parsedFinding = finding.copy(description = "changed"))

        assertNotEquals(hasher.hashRecord(base), hasher.hashRecord(changed))
    }

    @Test
    fun nullAndEmptyAreDistinctValues() {
        val hasher = EvidenceHasher()
        val withoutPrevious = record(previousHash = null)
        val withEmptyPrevious = record(previousHash = "")

        assertNotEquals(
            hasher.hashRecord(withoutPrevious),
            hasher.hashRecord(withEmptyPrevious)
        )
    }

    @Test
    fun withComputedHashBindsTheRecordToItsCanonicalHash() {
        val hasher = EvidenceHasher()
        val hashed = record(recordHash = "placeholder").withComputedHash(hasher)

        assertEquals(hasher.hashRecord(hashed), hashed.recordHash)
        assertEquals(null, hashed.signature)
    }

    private fun record(
        id: String = "evidence-1",
        timestamp: Instant = Instant.EPOCH,
        previousHash: String? = null,
        parsedFinding: com.eagle.app.ai.findings.SecurityFinding? = null,
        regressionTestIds: List<String> = listOf("reg-1"),
        humanApproval: HumanApproval? = null,
        recordHash: String = "placeholder"
    ) = EvidenceRecord(
        id = id,
        timestamp = timestamp,
        taskType = TaskType.CODE_REVIEW,
        providerId = "provider-a",
        providerVersion = "v1",
        codeHash = "sha256:code",
        promptHash = "sha256:prompt",
        rawOutput = "final output",
        parsedFinding = parsedFinding,
        reproductionTestId = "test-1",
        sastResultId = "sast-1",
        regressionTestIds = regressionTestIds,
        buildId = "build-1",
        humanApproval = humanApproval,
        previousHash = previousHash,
        recordHash = recordHash
    )
}
