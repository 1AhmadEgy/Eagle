package com.eagle.app.ai.provider
class TaskRouter(private val registry: ProviderRegistry) {
    fun candidates(taskType: TaskType, requirements: ProviderRequirements = ProviderRequirements()): List<AIProvider> =
        registry.all()
            .filter { it.policy.allowsTask(taskType) }
            .filter { requirements.matches(it) }
            .sortedWith(compareByDescending<AIProvider> { capabilityScore(it, requirements) }.thenBy { it.id })
    fun select(taskType: TaskType, requirements: ProviderRequirements = ProviderRequirements()): AIProvider? =
        candidates(taskType, requirements).firstOrNull()
    private fun capabilityScore(provider: AIProvider, requirements: ProviderRequirements): Int {
        val c = provider.capabilities
        var score = c.reasoningDepth.ordinal + minOf(c.longContextTokens / 8192, 8)
        if (c.toolUse) score += 1
        if (c.onPremise) score += 1
        if (c.telemetryFree) score += 1
        if (c.codeOnly) score += 1
        if (c.supportsCryptoReview) score += 2
        if (requirements.requireToolUse && c.toolUse) score += 4
        if (requirements.requireOnPremise && c.onPremise) score += 4
        if (requirements.requireTelemetryFree && c.telemetryFree) score += 4
        if (requirements.requireCodeOnly && c.codeOnly) score += 4
        if (requirements.requireCryptoReview && c.supportsCryptoReview) score += 6
        return score
    }
}
