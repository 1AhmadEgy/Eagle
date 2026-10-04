package com.eagle.app.ai

import com.eagle.app.ai.evidence.EvidenceRecord
import com.eagle.app.ai.evidence.HumanApproval
import com.eagle.app.ai.findings.FindingSeverity
import com.eagle.app.ai.findings.FindingStatus
import com.eagle.app.ai.findings.SecurityFinding
import com.eagle.app.ai.provider.TaskType
import java.time.Instant
import org.junit.Assert.assertEquals
import org.junit.Test

class EvidenceRecordTest {

    @Test
    fun storesTheMinimumVerificationBindings() {
        val finding = SecurityFinding(
            id = "finding-1",
            taskType = TaskType.CRYPTO_REVIEW,
            severity = FindingSeverity.HIGH,
            status = FindingStatus.PENDING_VERIFICATION,
            title = "Replay window evidence gap",
            description = "Verification evidence required",
            affectedPaths = setOf("app/src/main/java/com/eagle/app/security/ReplayGuard.kt"),
            createdAt = Instant.EPOCH,
            proposerProviderId = "provider-a"
        )
        val approval = HumanApproval(
            approvedBy = "human-reviewer",
            approvedAt = Instant.EPOCH,
            decision = HumanApproval.Decision.APPROVED,
            note = "Approved test-only evidence"
        )
        val record = EvidenceRecord(
            id = "evidence-1",
            timestamp = Instant.EPOCH,
            taskType = TaskType.CODE_REVIEW,
            providerId = "provider-a",
            providerVersion = "2026-test",
            codeHash = "sha256:code",
            promptHash = "sha256:prompt",
            rawOutput = "final redacted output",
            parsedFinding = finding,
            reproductionTestId = "test-1",
            sastResultId = "sast-1",
            regressionTestIds = listOf("reg-1"),
            buildId = "build-1",
            humanApproval = approval,
            previousHash = null,
            recordHash = "sha256:record"
        )

        assertEquals("sha256:code", record.codeHash)
        assertEquals("sha256:prompt", record.promptHash)
        assertEquals("test-1", record.reproductionTestId)
        assertEquals("sast-1", record.sastResultId)
        assertEquals("build-1", record.buildId)
        assertEquals(HumanApproval.Decision.APPROVED, record.humanApproval?.decision)
    }

    @Test
    fun evidenceOutputIsBounded() {
        try {
            EvidenceRecord(
                id = "evidence-1",
                timestamp = Instant.EPOCH,
                taskType = TaskType.CODE_REVIEW,
                providerId = "provider-a",
                providerVersion = "2026-test",
                codeHash = "sha256:code",
                promptHash = "sha256:prompt",
                rawOutput = "x".repeat(65_537),
                parsedFinding = null,
                reproductionTestId = null,
                sastResultId = null,
                regressionTestIds = emptyList(),
                buildId = null,
                humanApproval = null,
                previousHash = null,
                recordHash = "sha256:record"
            )
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected oversized evidence output to be rejected")
    }

    @Test
    fun findingPayloadIsBounded() {
        try {
            SecurityFinding(
                id = "finding-1",
                taskType = TaskType.CODE_REVIEW,
                severity = FindingSeverity.LOW,
                status = FindingStatus.OPEN,
                title = "x".repeat(257),
                description = "description",
                affectedPaths = emptySet(),
                createdAt = Instant.EPOCH,
                proposerProviderId = "provider-a"
            )
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected oversized finding title to be rejected")
    }
}
