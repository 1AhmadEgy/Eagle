package com.eagle.app.ai.provider

import com.eagle.app.ai.findings.FindingSeverity

data class ProviderPolicy(
    val maxAutoPatchSeverity: FindingSeverity? = null,
    val requiresHumanForCrypto: Boolean = true,
    val allowedTaskTypes: Set<TaskType> = emptySet()
) {
    fun allowsTask(taskType: TaskType): Boolean =
        taskType in allowedTaskTypes

    fun allowsAutomaticPatch(
        severity: FindingSeverity,
        cryptoSensitive: Boolean
    ): Boolean {
        if (cryptoSensitive && requiresHumanForCrypto) return false
        val maximum = maxAutoPatchSeverity ?: return false
        return severity.rank <= maximum.rank
    }
}
