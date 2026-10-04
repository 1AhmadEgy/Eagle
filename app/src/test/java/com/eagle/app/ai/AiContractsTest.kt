package com.eagle.app.ai

import com.eagle.app.ai.findings.FindingSeverity
import com.eagle.app.ai.findings.FindingStatus
import com.eagle.app.ai.findings.SecurityFinding
import com.eagle.app.ai.provider.AIProvider
import com.eagle.app.ai.provider.AnalyzeRequest
import com.eagle.app.ai.provider.AnalyzeResult
import com.eagle.app.ai.provider.PatchProposal
import com.eagle.app.ai.provider.PatchRequest
import com.eagle.app.ai.provider.ProviderCapabilities
import com.eagle.app.ai.provider.ProviderPolicy
import com.eagle.app.ai.provider.ReasoningDepth
import com.eagle.app.ai.provider.TaskType
import com.eagle.app.ai.provider.TestProposal
import com.eagle.app.ai.provider.TestRequest
import java.time.Instant
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class AiContractsTest {

    @Test
    fun providerContractCarriesCapabilitiesInsteadOfRoutingByName() {
        val policy = ProviderPolicy(
            maxAutoPatchSeverity = FindingSeverity.LOW,
            allowedTaskTypes = setOf(TaskType.CODE_REVIEW, TaskType.TEST_GENERATION)
        )
        val provider = FakeProvider(
            id = "test-provider",
            providerVersion = "1",
            capabilities = ProviderCapabilities(
                toolUse = true,
                longContextTokens = 100_000,
                onPremise = true,
                telemetryFree = true,
                codeOnly = true,
                reasoningDepth = ReasoningDepth.DEEP,
                supportsCryptoReview = false
            ),
            policy = policy
        )
        assertTrue(provider.capabilities.onPremise)
        assertTrue(provider.policy.allowsTask(TaskType.CODE_REVIEW))
        assertFalse(provider.policy.allowsTask(TaskType.CRYPTO_REVIEW))
    }

    @Test
    fun cryptoPatchCannotBeAutomaticallyApproved() {
        val policy = ProviderPolicy(
            maxAutoPatchSeverity = FindingSeverity.CRITICAL,
            requiresHumanForCrypto = true
        )
        assertFalse(policy.allowsAutomaticPatch(FindingSeverity.CRITICAL, true))
        assertTrue(policy.allowsAutomaticPatch(FindingSeverity.LOW, false))
    }

    @Test
    fun cryptoSensitivePatchProposalCannotSkipHumanApproval() {
        try {
            PatchProposal(
                id = "p1",
                findingId = "f1",
                patch = "safe-looking patch",
                touchedPaths = setOf("app/src/main/Security.kt"),
                cryptoSensitive = true,
                requiresHumanApproval = false
            )
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected crypto-sensitive patch to require human approval")
    }

    @Test
    fun requestAndResultPayloadsAreBounded() {
        try {
            AnalyzeRequest(
                taskType = TaskType.CODE_REVIEW,
                targetCommit = "sha",
                context = "x".repeat(32_769)
            )
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected oversized analysis context to be rejected")
    }

    @Test
    fun highSeverityRanksAboveMedium() {
        assertEquals(3, FindingSeverity.HIGH.rank)
        assertTrue(FindingSeverity.CRITICAL.rank > FindingSeverity.HIGH.rank)
    }

    private class FakeProvider(
        override val id: String,
        override val providerVersion: String,
        override val capabilities: ProviderCapabilities,
        override val policy: ProviderPolicy
    ) : AIProvider {
        override suspend fun analyze(request: AnalyzeRequest): AnalyzeResult =
            AnalyzeResult(
                findings = listOf(
                    SecurityFinding(
                        id = "finding-1",
                        taskType = request.taskType,
                        severity = FindingSeverity.MEDIUM,
                        status = FindingStatus.OPEN,
                        title = "test",
                        description = "test finding",
                        affectedPaths = setOf("app/src/main/java/com/eagle/app/security/ReplayGuard.kt"),
                        createdAt = Instant.EPOCH,
                        proposerProviderId = id
                    )
                ),
                output = "final output"
            )

        override suspend fun proposePatch(request: PatchRequest): PatchProposal? = null

        override suspend fun generateTest(request: TestRequest): TestProposal =
            TestProposal(
                id = "test-1",
                findingId = request.findingId,
                testDescription = "reproduce finding",
                testPaths = setOf("app/src/test")
            )
    }
}
