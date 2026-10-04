package com.eagle.app.ai.findings

import com.eagle.app.ai.provider.TaskType
import java.time.Instant

data class SecurityFinding(
    val id: String,
    val taskType: TaskType,
    val severity: FindingSeverity,
    val status: FindingStatus,
    val title: String,
    val description: String,
    val affectedPaths: Set<String>,
    val createdAt: Instant,
    val proposerProviderId: String
) {
    init {
        require(id.isNotBlank()) { "id must not be blank" }
        require(title.isNotBlank()) { "title must not be blank" }
        require(description.isNotBlank()) { "description must not be blank" }
        require(proposerProviderId.isNotBlank()) { "proposerProviderId must not be blank" }
        require(affectedPaths.none { it.isBlank() }) { "affectedPaths cannot contain blank paths" }
    }
}
