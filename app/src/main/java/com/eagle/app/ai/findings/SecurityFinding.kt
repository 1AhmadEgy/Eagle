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
        require(id.length <= MAX_ID_LENGTH) { "id is too long" }
        require(title.isNotBlank()) { "title must not be blank" }
        require(title.length <= MAX_TITLE_CHARS) { "title is too long" }
        require(description.isNotBlank()) { "description must not be blank" }
        require(description.length <= MAX_DESCRIPTION_CHARS) { "description is too long" }
        require(affectedPaths.size <= MAX_PATHS) { "too many affected paths" }
        require(affectedPaths.none { it.isBlank() || it.length > MAX_PATH_LENGTH }) {
            "affectedPaths contains an invalid path"
        }
        require(proposerProviderId.isNotBlank()) { "proposerProviderId must not be blank" }
        require(proposerProviderId.length <= MAX_ID_LENGTH) {
            "proposerProviderId is too long"
        }
    }

    companion object {
        const val MAX_ID_LENGTH = 128
        const val MAX_TITLE_CHARS = 256
        const val MAX_DESCRIPTION_CHARS = 16_384
        const val MAX_PATHS = 128
        const val MAX_PATH_LENGTH = 512
    }
}
