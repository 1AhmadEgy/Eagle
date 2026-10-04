package com.eagle.app.ai.evidence

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class EvidenceSignerTest {

    @Test
    fun signatureVerifiesForOriginalRecordHash() {
        val signer = InMemoryTestSigner()
        val hash = "0123456789abcdef"

        val signature = signer.sign(hash)

        assertTrue(signer.verify(hash, signature))
    }

    @Test
    fun signatureFailsAfterRecordHashChanges() {
        val signer = InMemoryTestSigner()
        val signature = signer.sign("hash-a")

        assertFalse(signer.verify("hash-b", signature))
    }

    @Test
    fun repeatedSigningOfSameHashIsDeterministicForEd25519() {
        val signer = InMemoryTestSigner()

        assertTrue(signer.sign("hash-a") == signer.sign("hash-a"))
    }
}
