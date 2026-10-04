package com.eagle.app.ai.provider
data class ProviderRequirements(
    val requireToolUse: Boolean = false,
    val requireOnPremise: Boolean = false,
    val requireTelemetryFree: Boolean = false,
    val requireCodeOnly: Boolean = false,
    val requireCryptoReview: Boolean = false,
    val minimumReasoningDepth: ReasoningDepth? = null,
    val minimumLongContextTokens: Int = 0
) {
    init { require(minimumLongContextTokens >= 0) }
    fun matches(provider: AIProvider): Boolean {
        val c = provider.capabilities
        if (requireToolUse && !c.toolUse) return false
        if (requireOnPremise && !c.onPremise) return false
        if (requireTelemetryFree && !c.telemetryFree) return false
        if (requireCodeOnly && !c.codeOnly) return false
        if (requireCryptoReview && !c.supportsCryptoReview) return false
        if (c.longContextTokens < minimumLongContextTokens) return false
        val minimum = minimumReasoningDepth
        if (minimum != null && c.reasoningDepth.ordinal < minimum.ordinal) return false
        return true
    }
}
