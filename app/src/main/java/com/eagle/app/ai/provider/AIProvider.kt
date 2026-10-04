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
) {
    init {
        require(targetCommit.isNotBlank())
        require(targetCommit.length <= MAX_COMMIT_LENGTH)
        require(context.isNotBlank())
        require(context.length <= MAX_CONTEXT_CHARS)
        require(evidenceIds.size <= MAX_EVIDENCE_IDS)
        require(evidenceIds.none { it.isBlank() || it.length > MAX_ID_LENGTH })
    }
}

data class AnalyzeResult(
    val findings: List<SecurityFinding>,
    val output: String
) {
    init {
        require(findings.size <= MAX_FINDINGS)
        require(output.isNotBlank())
        require(output.length <= MAX_OUTPUT_CHARS)
    }
}

data class PatchRequest(
    val findingId: String,
    val targetCommit: String,
    val context: String,
    val allowedPaths: Set<String>
) {
    init {
        require(findingId.isNotBlank())
        require(findingId.length <= MAX_ID_LENGTH)
        require(targetCommit.isNotBlank())
        require(targetCommit.length <= MAX_COMMIT_LENGTH)
        require(context.isNotBlank())
        require(context.length <= MAX_CONTEXT_CHARS)
        require(allowedPaths.isNotEmpty())
        require(allowedPaths.size <= MAX_PATHS)
        require(allowedPaths.none { it.isBlank() || it.length > MAX_PATH_LENGTH })
    }
}

data class PatchProposal(
    val id: String,
    val findingId: String,
    val patch: String,
    val touchedPaths: Set<String>,
    val cryptoSensitive: Boolean,
    val requiresHumanApproval: Boolean
) {
    init {
        require(id.isNotBlank())
        require(findingId.isNotBlank())
        require(patch.isNotBlank())
        require(patch.length <= MAX_PATCH_CHARS)
        require(touchedPaths.isNotEmpty())
        require(touchedPaths.size <= MAX_PATHS)
        require(touchedPaths.none { it.isBlank() || it.length > MAX_PATH_LENGTH })
        if (cryptoSensitive) {
            require(requiresHumanApproval) {
                "crypto-sensitive patches require human approval"
            }
        }
    }
}

data class TestRequest(
    val findingId: String,
    val targetCommit: String,
    val context: String
) {
    init {
        require(findingId.isNotBlank())
        require(findingId.length <= MAX_ID_LENGTH)
        require(targetCommit.isNotBlank())
        require(targetCommit.length <= MAX_COMMIT_LENGTH)
        require(context.isNotBlank())
        require(context.length <= MAX_CONTEXT_CHARS)
    }
}

data class TestProposal(
    val id: String,
    val findingId: String,
    val testDescription: String,
    val testPaths: Set<String>
) {
    init {
        require(id.isNotBlank())
        require(findingId.isNotBlank())
        require(testDescription.isNotBlank())
        require(testDescription.length <= MAX_TEST_DESCRIPTION_CHARS)
        require(testPaths.isNotEmpty())
        require(testPaths.size <= MAX_PATHS)
        require(testPaths.none { it.isBlank() || it.length > MAX_PATH_LENGTH })
    }
}

private const val MAX_COMMIT_LENGTH = 128
private const val MAX_ID_LENGTH = 128
private const val MAX_CONTEXT_CHARS = 32_768
private const val MAX_OUTPUT_CHARS = 65_536
private const val MAX_PATCH_CHARS = 65_536
private const val MAX_TEST_DESCRIPTION_CHARS = 4_096
private const val MAX_EVIDENCE_IDS = 128
private const val MAX_FINDINGS = 128
private const val MAX_PATHS = 128
private const val MAX_PATH_LENGTH = 512
