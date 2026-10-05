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
 * Authenticated, password-derived recovery envelope.
 *
 * The caller supplies associated data (for example an account/device/epoch binding).
 * The envelope itself never persists secrets.
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
    private const val MAX_SECRET_CHARS = 1_024
    private val random = SecureRandom()

    fun seal(
        payload: ByteArray,
        recoverySecret: CharArray,
        associatedData: ByteArray = ByteArray(0),
    ): String {
        validate(payload, recoverySecret, associatedData)

        val salt = ByteArray(SALT_BYTES).also(random::nextBytes)
        val nonce = ByteArray(NONCE_BYTES).also(random::nextBytes)
        val key = deriveKey(recoverySecret, salt)
        return try {
            val cipher = Cipher.getInstance("AES/GCM/NoPadding")
            cipher.init(Cipher.ENCRYPT_MODE, key, GCMParameterSpec(TAG_BITS, nonce))
            cipher.updateAAD(aad(associatedData))
            val ciphertext = cipher.doFinal(payload)

            ByteBuffer.allocate(1 + 4 + SALT_BYTES + NONCE_BYTES + ciphertext.size).apply {
                put(VERSION)
                putInt(ITERATIONS)
                put(salt)
                put(nonce)
                put(ciphertext)
            }.array().let(Base64.getEncoder()::encodeToString)
        } finally {
            salt.fill(0)
            nonce.fill(0)
        }
    }

    fun open(
        envelope: String,
        recoverySecret: CharArray,
        associatedData: ByteArray = ByteArray(0),
    ): ByteArray {
        validate(ByteArray(0), recoverySecret, associatedData)

        val raw = try {
            Base64.getDecoder().decode(envelope)
        } catch (_: IllegalArgumentException) {
            throw SecurityException("Invalid recovery envelope")
        }
        require(raw.size >= 1 + 4 + SALT_BYTES + NONCE_BYTES + 16) {
            "Invalid recovery envelope"
        }

        val input = ByteBuffer.wrap(raw)
        require(input.get() == VERSION) { "Unsupported recovery envelope version" }

        val iterations = input.int
        require(iterations in 100_000..1_000_000) { "Invalid recovery parameters" }

        val salt = ByteArray(SALT_BYTES).also(input::get)
        val nonce = ByteArray(NONCE_BYTES).also(input::get)
        val ciphertext = ByteArray(input.remaining()).also(input::get)
        require(ciphertext.size <= MAX_PAYLOAD_BYTES + 16) { "Recovery payload too large" }

        val key = deriveKey(recoverySecret, salt, iterations)
        return try {
            Cipher.getInstance("AES/GCM/NoPadding").run {
                init(Cipher.DECRYPT_MODE, key, GCMParameterSpec(TAG_BITS, nonce))
                updateAAD(aad(associatedData))
                try {
                    doFinal(ciphertext)
                } catch (_: Exception) {
                    throw SecurityException("Recovery authentication failed")
                }
            }
        } finally {
            salt.fill(0)
            nonce.fill(0)
        }
    }

    private fun validate(
        payload: ByteArray,
        recoverySecret: CharArray,
        associatedData: ByteArray,
    ) {
        require(payload.size <= MAX_PAYLOAD_BYTES) { "Recovery payload too large" }
        require(recoverySecret.size >= 12) { "Recovery secret is too short" }
        require(recoverySecret.size <= MAX_SECRET_CHARS) { "Recovery secret is too long" }
        require(associatedData.size <= 4096) { "Recovery binding is too large" }
    }

    private fun aad(associatedData: ByteArray): ByteArray =
        ByteBuffer.allocate(4 + PURPOSE.toByteArray(Charsets.UTF_8).size + associatedData.size).apply {
            val purpose = PURPOSE.toByteArray(Charsets.UTF_8)
            putInt(purpose.size)
            put(purpose)
            put(associatedData)
        }.array()

    private fun deriveKey(secret: CharArray, salt: ByteArray, iterations: Int = ITERATIONS): SecretKey {
        val spec = PBEKeySpec(secret, salt, iterations, KEY_BYTES * 8)
        return try {
            val encoded = SecretKeyFactory.getInstance("PBKDF2WithHmacSHA256")
                .generateSecret(spec).encoded
            try {
                SecretKeySpec(encoded, "AES")
            } finally {
                encoded.fill(0)
            }
        } finally {
            spec.clearPassword()
        }
    }
}
