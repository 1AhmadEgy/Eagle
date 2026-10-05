package com.eagle.app

import com.eagle.app.security.RecoveryEnvelope
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertThrows
import org.junit.Test

class RecoveryEnvelopeTest {
    private val secret = "correct horse battery staple".toCharArray()

    @Test
    fun roundTripRestoresExactPayload() {
        val payload = "identity-state-v1".toByteArray()
        val binding = "account-42|device-7|epoch-9".toByteArray()

        val envelope = RecoveryEnvelope.seal(payload, secret, binding)
        val restored = RecoveryEnvelope.open(envelope, secret, binding)

        assertArrayEquals(payload, restored)
    }

    @Test
    fun wrongSecretFailsAuthentication() {
        val envelope = RecoveryEnvelope.seal(
            "sensitive-state".toByteArray(),
            secret
        )

        assertThrows(SecurityException::class.java) {
            RecoveryEnvelope.open(
                envelope,
                "a different recovery secret".toCharArray()
            )
        }
    }

    @Test
    fun wrongContextFailsAuthentication() {
        val envelope = RecoveryEnvelope.seal(
            "sensitive-state".toByteArray(),
            secret,
            "account-a|device-a|epoch-1".toByteArray()
        )

        assertThrows(SecurityException::class.java) {
            RecoveryEnvelope.open(
                envelope,
                secret,
                "account-a|device-a|epoch-2".toByteArray()
            )
        }
    }

    @Test
    fun tamperingFailsAuthentication() {
        val envelope = RecoveryEnvelope.seal(
            "sensitive-state".toByteArray(),
            secret
        )
        val raw = java.util.Base64.getDecoder().decode(envelope)
        raw[raw.lastIndex] = (raw.last().toInt() xor 1).toByte()
        val tampered = java.util.Base64.getEncoder().encodeToString(raw)

        assertThrows(SecurityException::class.java) {
            RecoveryEnvelope.open(tampered, secret)
        }
    }

    @Test
    fun unsupportedVersionIsRejected() {
        val envelope = RecoveryEnvelope.seal("state".toByteArray(), secret)
        val raw = java.util.Base64.getDecoder().decode(envelope)
        raw[0] = 2

        assertThrows(IllegalArgumentException::class.java) {
            RecoveryEnvelope.open(
                java.util.Base64.getEncoder().encodeToString(raw),
                secret
            )
        }
    }

    @Test
    fun oversizedSecretIsRejected() {
        assertThrows(IllegalArgumentException::class.java) {
            RecoveryEnvelope.seal(
                "state".toByteArray(),
                CharArray(1_025) { 'x' }
            )
        }
    }
}
