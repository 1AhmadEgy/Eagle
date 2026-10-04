package com.eagle.app.ai.verification
import com.eagle.app.ai.findings.FindingStatus
import com.eagle.app.ai.provider.AnalyzeRequest
import com.eagle.app.ai.provider.ProviderRequirements
import com.eagle.app.ai.provider.TaskRouter
import com.eagle.app.ai.provider.TaskType
data class VerificationPipelineResult(
    val proposerId: String,
    val verifierId: String,
    val findings: List<com.eagle.app.ai.findings.SecurityFinding>,
    val outcomes: List<VerificationOutcome>
) {
    init { require(proposerId.isNotBlank()); require(verifierId.isNotBlank()); require(findings.size == outcomes.size) }
}
class VerificationPipeline(private val router: TaskRouter, private val verifier: FindingVerifier) {
    suspend fun run(request: AnalyzeRequest, verifierRequirements: ProviderRequirements = ProviderRequirements()): VerificationPipelineResult {
        val proposer = router.select(request.taskType) ?: error("no provider satisfies task policy: ${request.taskType}")
        val verifierProvider = router.candidates(TaskType.CODE_REVIEW, verifierRequirements).firstOrNull { it.id != proposer.id }
            ?: error("no independent verifier provider available")
        val result = proposer.analyze(request)
        val outcomes = result.findings.map { finding -> verifier.verify(finding, request.targetCommit, verifierProvider) }
        val normalized = result.findings.zip(outcomes).map { (finding, outcome) ->
            finding.copy(status = if (outcome.verified) FindingStatus.VERIFIED else FindingStatus.DISPUTED)
        }
        return VerificationPipelineResult(proposer.id, verifierProvider.id, normalized, outcomes)
    }
}
