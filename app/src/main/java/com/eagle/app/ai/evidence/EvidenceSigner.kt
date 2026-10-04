package com.eagle.app.ai.evidence

/**
 * Signs and verifies the canonical evidence record hash.
 *
 * Key management is intentionally outside this interface so Android Keystore,
 * CI-managed keys, or another approved key source can be introduced without
 * changing the evidence contract.
 */
interface EvidenceSigner {
    fun sign(recordHash: String): String
    fun verify(recordHash: String, signature: String): Boolean
}
