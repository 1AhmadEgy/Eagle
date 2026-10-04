package com.eagle.app.ai

import com.eagle.app.ai.findings.FindingSeverity
import com.eagle.app.ai.findings.FindingStatus
import com.eagle.app.ai.findings.SecurityFinding
import com.eagle.app.ai.provider.*
import com.eagle.app.ai.verification.FindingVerifier
import com.eagle.app.ai.verification.VerificationOutcome
import com.eagle.app.ai.verification.VerificationPipeline
import java.time.Instant
import kotlin.coroutines.Continuation
import kotlin.coroutines.EmptyCoroutineContext
import kotlin.coroutines.startCoroutine
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AiRoutingAndVerificationTest {
    @Test
    fun routerUsesCapabilitiesAndPolicyNotProviderName() {
        val weak = fake("preferred-name", ReasoningDepth.SURFACE, crypto = false, tasks = setOf(TaskType.CRYPTO_REVIEW))
        val strong = fake("other-name", ReasoningDepth.DEEP, crypto = true, tasks = setOf(TaskType.CRYPTO_REVIEW))
        val router = TaskRouter(ProviderRegistry(listOf(weak, strong)))

        val selected = router.select(
            TaskType.CRYPTO_REVIEW,
            ProviderRequirements(requireCryptoReview = true, minimumReasoningDepth = ReasoningDepth.STANDARD)
        )

        assertEquals("other-name", selected?.id)
    }

    @Test
    fun policyCanExcludeProviderEvenWhenCapabilitiesMatch() {
        val provider = fake("blocked", ReasoningDepth.DEEP, crypto = true, tasks = emptySet())
        val router = TaskRouter(ProviderRegistry(listOf(provider)))

        assertTrue(router.select(TaskType.CRYPTO_REVIEW) == null)
    }

    @Test
    fun verificationUsesIndependentProviderAndNormalizesStatus() {
        val proposer = fake("proposer", ReasoningDepth.STANDARD, crypto = false, tasks = setOf(TaskType.CODE_REVIEW))
        val verifierProvider = fake("verifier", ReasoningDepth.DEEP, crypto = true, tasks = setOf(TaskType.CODE_REVIEW))
        val finding = SecurityFinding(
            id = "finding-1",
            taskType = TaskType.CODE_REVIEW,
            severity = FindingSeverity.HIGH,
            status = FindingStatus.PENDING_VERIFICATION,
            title = "test finding",
            description = "description",
            affectedPaths = setOf("src/A.kt"),
            createdAt = Instant.EPOCH,
            proposerProviderId = proposer.id
        )
        val verifier = object : FindingVerifier {
            override suspend fun verify(
                finding: SecurityFinding,
                targetCommit: String,
                provider: AIProvider
            ): VerificationOutcome {
                assertEquals("verifier", provider.id)
                assertEquals("abc123", targetCommit)
                return VerificationOutcome(true, "reproduced")
            }
        }

        val result = runSuspend {
            VerificationPipeline(TaskRouter(ProviderRegistry(listOf(proposer, verifierProvider))), verifier)
                .run(AnalyzeRequest(TaskType.CODE_REVIEW, "abc123", "context"))
        }

        assertEquals("proposer", result.proposerId)
        assertEquals("verifier", result.verifierId)
        assertEquals(FindingStatus.VERIFIED, result.findings.single().status)
    }

    private fun fake(
        id: String,
        depth: ReasoningDepth,
        crypto: Boolean,
        tasks: Set<TaskType>
    ): AIProvider = object : AIProvider {
        override val id = id
        override val providerVersion = "test"
        override val capabilities = ProviderCapabilities(
            toolUse = true,
            longContextTokens = 16_384,
            onPremise = true,
            telemetryFree = true,
            codeOnly = true,
            reasoningDepth = depth,
            supportsCryptoReview = crypto
        )
        override val policy = ProviderPolicy(
            maxAutoPatchSeverity = null,
            requiresHumanForCrypto = true,
            allowedTaskTypes = tasks
        )

        override suspend fun analyze(request: AnalyzeRequest) =
            AnalyzeResult(
                findings = listOf(
                    SecurityFinding(
                        id = "finding-1",
                        taskType = request.taskType,
                        severity = FindingSeverity.HIGH,
                        status = FindingStatus.PENDING_VERIFICATION,
                        title = "test finding",
                        description = "description",
                        affectedPaths = setOf("src/A.kt"),
                        createdAt = Instant.EPOCH,
                        proposerProviderId = id
                    )
                ),
                output = "analysis"
            )

        override suspend fun proposePatch(request: PatchRequest): PatchProposal? = null

        override suspend fun generateTest(request: TestRequest) =
            TestProposal("test-1", request.findingId, "test", setOf("src/A.kt"))
    }

    private fun <T> runSuspend(block: suspend () -> T): T {
        var value: T? = null
        var failure: Throwable? = null
        block.startCoroutine(object : Continuation<T> {
            override val context = EmptyCoroutineContext
            override fun resumeWith(result: Result<T>) {
                result.onSuccess { value = it }.onFailure { failure = it }
            }
        })
        failure?.let { throw it }
        return checkNotNull(value)
    }
}
