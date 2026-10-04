package com.eagle.app.ai.verification
import com.eagle.app.ai.findings.SecurityFinding
import com.eagle.app.ai.provider.AIProvider
interface FindingVerifier {
    suspend fun verify(finding: SecurityFinding, targetCommit: String, provider: AIProvider): VerificationOutcome
}
data class VerificationOutcome(val verified: Boolean, val summary: String) {
    init { require(summary.isNotBlank()); require(summary.length <= 4096) }
}
