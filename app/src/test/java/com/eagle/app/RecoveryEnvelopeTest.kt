package com.eagle.app

import com.eagle.app.security.RecoveryEnvelope
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertThrows
import org.junit.Test

class RecoveryEnvelopeTest {
    @Test
    fun roundTripRestoresExactPayload() {
        val payload = "identity-state-v1".toByteArray()
        val secret = "correct horse battery staple".toCharArray()

        val envelope = RecoveryEnvelope.seal(payload, secret)
        val restored = RecoveryEnvelope.open(envelope, secret)

        assertArrayEquals(payload, restored)
    }

    @Test
    fun wrongSecretFailsAuthentication() {
        val envelope = RecoveryEnvelope.seal(
            "sensitive-state".toByteArray(),
            "a sufficiently long recovery secret".toCharArray()
        )

        assertThrows(SecurityException::class.java) {
            RecoveryEnvelope.open(envelope, "a different recovery secret".toCharArray())
        }
    }

    @Test
    fun tamperingFailsAuthentication() {
        val envelope = RecoveryEnvelope.seal(
            "sensitive-state".toByteArray(),
            "a sufficiently long recovery secret".toCharArray()
        )
        val raw = java.util.Base64.getDecoder().decode(envelope)
        raw[raw.lastIndex] = (raw.last().toInt() xor 1).toByte()
        val tampered = java.util.Base64.getEncoder().encodeToString(raw)

        assertThrows(SecurityException::class.java) {
            RecoveryEnvelope.open(tampered, "a sufficiently long recovery secret".toCharArray())
        }
    }

    @Test
    fun unsupportedVersionIsRejected() {
        val envelope = RecoveryEnvelope.seal(
            "state".toByteArray(),
            "a sufficiently long recovery secret".toCharArray()
        )
        val raw = java.util.Base64.getDecoder().decode(envelope)
        raw[0] = 2

        assertThrows(IllegalArgumentException::class.java) {
            RecoveryEnvelope.open(
                java.util.Base64.getEncoder().encodeToString(raw),
                "a sufficiently long recovery secret".toCharArray()
            )
        }
    }
}
