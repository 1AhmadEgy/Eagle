package com.eagle.app.ai.provider

data class ProviderCapabilities(
    val toolUse: Boolean,
    val longContextTokens: Int,
    val onPremise: Boolean,
    val telemetryFree: Boolean,
    val codeOnly: Boolean,
    val reasoningDepth: ReasoningDepth,
    val supportsCryptoReview: Boolean
) {
    init {
        require(longContextTokens >= 0) { "longContextTokens must be non-negative" }
    }
}
