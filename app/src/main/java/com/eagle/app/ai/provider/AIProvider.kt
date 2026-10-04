package com.eagle.app.ai.provider

import com.eagle.app.ai.findings.SecurityFinding

interface AIProvider {
    val id: String
    val providerVersion: String
    val capabilities: ProviderCapabilities
    val policy: ProviderPolicy

    suspend fun analyze(request: AnalyzeRequest): AnalyzeResult

    suspend fun proposePatch(request: PatchRequest): PatchProposal?

    suspend fun generateTest(request: TestRequest): TestProposal
}

data class AnalyzeRequest(
    val taskType: TaskType,
    val targetCommit: String,
    val context: String,
    val evidenceIds: List<String> = emptyList()
)

data class AnalyzeResult(
    val findings: List<SecurityFinding>,
    val output: String
)

data class PatchRequest(
    val findingId: String,
    val targetCommit: String,
    val context: String,
    val allowedPaths: Set<String>
)

data class PatchProposal(
    val id: String,
    val findingId: String,
    val patch: String,
    val touchedPaths: Set<String>,
    val cryptoSensitive: Boolean,
    val requiresHumanApproval: Boolean
)

data class TestRequest(
    val findingId: String,
    val targetCommit: String,
    val context: String
)

data class TestProposal(
    val id: String,
    val findingId: String,
    val testDescription: String,
    val testPaths: Set<String>
)
