package com.eagle.app.security

import java.nio.ByteBuffer
import java.security.SecureRandom
import java.util.Base64
import javax.crypto.Cipher
import javax.crypto.SecretKey
import javax.crypto.SecretKeyFactory
import javax.crypto.spec.GCMParameterSpec
import javax.crypto.spec.PBEKeySpec
import javax.crypto.spec.SecretKeySpec

/**
 * Password-based recovery envelope for explicitly supplied recovery state.
 *
 * The envelope is intentionally independent from Android UI and storage APIs so it can
 * be verified with JVM unit tests. It never persists secrets by itself.
 */
object RecoveryEnvelope {
    private const val VERSION: Byte = 1
    private const val PURPOSE = "EAGLE-RECOVERY-V1"
    private const val SALT_BYTES = 32
    private const val NONCE_BYTES = 12
    private const val KEY_BYTES = 32
    private const val TAG_BITS = 128
    private const val ITERATIONS = 600_000
    private const val MAX_PAYLOAD_BYTES = 1_048_576
    private val random = SecureRandom()

    fun seal(payload: ByteArray, recoverySecret: CharArray): String {
        require(payload.size <= MAX_PAYLOAD_BYTES) { "Recovery payload too large" }
        require(recoverySecret.size >= 12) { "Recovery secret is too short" }

        val salt = ByteArray(SALT_BYTES).also(random::nextBytes)
        val nonce = ByteArray(NONCE_BYTES).also(random::nextBytes)
        val key = deriveKey(recoverySecret, salt)

        return try {
            val cipher = Cipher.getInstance("AES/GCM/NoPadding")
            cipher.init(Cipher.ENCRYPT_MODE, key, GCMParameterSpec(TAG_BITS, nonce))
            cipher.updateAAD(PURPOSE.toByteArray(Charsets.UTF_8))
            val ciphertext = cipher.doFinal(payload)

            val out = ByteBuffer.allocate(
                1 + 4 + SALT_BYTES + NONCE_BYTES + ciphertext.size
            )
            out.put(VERSION)
            out.putInt(ITERATIONS)
            out.put(salt)
            out.put(nonce)
            out.put(ciphertext)
            Base64.getEncoder().encodeToString(out.array())
        } finally {
            key.encoded.fill(0)
        }
    }

    fun open(envelope: String, recoverySecret: CharArray): ByteArray {
        require(recoverySecret.size >= 12) { "Recovery secret is too short" }

        val raw = try {
            Base64.getDecoder().decode(envelope)
        } catch (_: IllegalArgumentException) {
            throw SecurityException("Invalid recovery envelope")
        }

        require(raw.size >= 1 + 4 + SALT_BYTES + NONCE_BYTES + 16) {
            "Invalid recovery envelope"
        }

        val input = ByteBuffer.wrap(raw)
        val version = input.get()
        require(version == VERSION) { "Unsupported recovery envelope version" }

        val iterations = input.int
        require(iterations in 100_000..1_000_000) { "Invalid recovery parameters" }

        val salt = ByteArray(SALT_BYTES).also(input::get)
        val nonce = ByteArray(NONCE_BYTES).also(input::get)
        val ciphertext = ByteArray(input.remaining()).also(input::get)
        require(ciphertext.size <= MAX_PAYLOAD_BYTES + 16) { "Recovery payload too large" }

        val key = deriveKey(recoverySecret, salt, iterations)
        return try {
            val cipher = Cipher.getInstance("AES/GCM/NoPadding")
            cipher.init(Cipher.DECRYPT_MODE, key, GCMParameterSpec(TAG_BITS, nonce))
            cipher.updateAAD(PURPOSE.toByteArray(Charsets.UTF_8))
            try {
                cipher.doFinal(ciphertext)
            } catch (_: Exception) {
                throw SecurityException("Recovery authentication failed")
            }
        } finally {
            key.encoded.fill(0)
            salt.fill(0)
            nonce.fill(0)
        }
    }

    private fun deriveKey(secret: CharArray, salt: ByteArray, iterations: Int = ITERATIONS): SecretKey {
        val spec = PBEKeySpec(secret, salt, iterations, KEY_BYTES * 8)
        return try {
            val factory = SecretKeyFactory.getInstance("PBKDF2WithHmacSHA256")
            SecretKeySpec(factory.generateSecret(spec).encoded, "AES")
        } finally {
            spec.clearPassword()
        }
    }
}
