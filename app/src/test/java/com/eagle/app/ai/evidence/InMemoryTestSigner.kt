package com.eagle.app.ai.evidence

import java.security.KeyFactory
import java.security.PrivateKey
import java.security.Signature
import java.security.spec.PKCS8EncodedKeySpec
import java.security.spec.X509EncodedKeySpec
import java.util.Base64

/**
 * JVM-only test signer.
 *
 * Test key material is intentionally embedded in test sources and must never be
 * reused as a production key.
 */
class InMemoryTestSigner : EvidenceSigner {

    private val privateKey: PrivateKey
    private val publicKey: java.security.PublicKey

    init {
        val factory = KeyFactory.getInstance("Ed25519")
        privateKey = factory.generatePrivate(
            PKCS8EncodedKeySpec(Base64.getDecoder().decode(TEST_PRIVATE_KEY))
        )
        publicKey = factory.generatePublic(
            X509EncodedKeySpec(Base64.getDecoder().decode(TEST_PUBLIC_KEY))
        )
    }

    override fun sign(recordHash: String): String {
        val signature = Signature.getInstance("Ed25519")
        signature.initSign(privateKey)
        signature.update(recordHash.toByteArray(Charsets.US_ASCII))
        return Base64.getEncoder().encodeToString(signature.sign())
    }

    override fun verify(recordHash: String, signature: String): Boolean {
        return try {
            val verifier = Signature.getInstance("Ed25519")
            verifier.initVerify(publicKey)
            verifier.update(recordHash.toByteArray(Charsets.US_ASCII))
            verifier.verify(Base64.getDecoder().decode(signature))
        } catch (_: Exception) {
            false
        }
    }

    private companion object {
        const val TEST_PRIVATE_KEY =
            "MC4CAQAwBQYDK2VwBCIEIAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8g"
        const val TEST_PUBLIC_KEY =
            "MCowBQYDK2VwAyEAebVWLo/mVPlAeLES6KmLp5AfhTrmlb7X4OORC60ElmQ="
    }
}
