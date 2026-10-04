package com.eagle.app.ai

import com.eagle.app.ai.findings.FindingSeverity
import com.eagle.app.ai.provider.ProviderCapabilities
import com.eagle.app.ai.provider.ProviderPolicy
import com.eagle.app.ai.provider.ReasoningDepth
import com.eagle.app.ai.provider.TaskType
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ProviderPolicyTest {

    @Test
    fun cryptoRequiresHumanEvenWhenMaximumSeverityAllowsAutomaticPatch() {
        val policy = ProviderPolicy(
            maxAutoPatchSeverity = FindingSeverity.CRITICAL,
            requiresHumanForCrypto = true,
            allowedTaskTypes = setOf(TaskType.CRYPTO_REVIEW)
        )

        assertTrue(policy.allowsTask(TaskType.CRYPTO_REVIEW))
        assertFalse(policy.allowsAutomaticPatch(FindingSeverity.CRITICAL, true))
    }

    @Test
    fun automaticPatchCanBeLimitedToLowSeverityNonCryptoWork() {
        val policy = ProviderPolicy(
            maxAutoPatchSeverity = FindingSeverity.LOW,
            requiresHumanForCrypto = true
        )

        assertTrue(policy.allowsAutomaticPatch(FindingSeverity.LOW, false))
        assertFalse(policy.allowsAutomaticPatch(FindingSeverity.MEDIUM, false))
    }

    @Test
    fun capabilitiesAreValidated() {
        ProviderCapabilities(
            toolUse = false,
            longContextTokens = 0,
            onPremise = false,
            telemetryFree = true,
            codeOnly = true,
            reasoningDepth = ReasoningDepth.STANDARD,
            supportsCryptoReview = true
        )
    }
}
