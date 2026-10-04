package com.eagle.app.security

import org.junit.Assert.assertEquals
import org.junit.Test

class SecurityEventTest {

    @Test
    fun acceptsPrivacyMinimizedEvent() {
        val event = SecurityEvent(
            schemaVersion = 1,
            type = SecurityEvent.Type.REPLAY_DUPLICATE,
            monotonicTimestampMillis = 10_000,
            contextId = "ctx-1",
            peerPseudonym = "peer-a",
            protocolState = SessionStateMachine.State.ESTABLISHED,
            outcome = SecurityEvent.Outcome.REJECTED
        )
        assertEquals(SecurityEvent.Type.REPLAY_DUPLICATE, event.type)
        assertEquals(SecurityEvent.Outcome.REJECTED, event.outcome)
    }

    @Test
    fun rejectsInvalidSchemaVersion() {
        try {
            SecurityEvent(0, SecurityEvent.Type.REPLAY_ACCEPTED, 0, "ctx", null, null,
                SecurityEvent.Outcome.ACCEPTED)
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected invalid schema version")
    }

    @Test
    fun rejectsNegativeTimestamp() {
        try {
            SecurityEvent(1, SecurityEvent.Type.REPLAY_ACCEPTED, -1, "ctx", null, null,
                SecurityEvent.Outcome.ACCEPTED)
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected invalid timestamp")
    }

    @Test
    fun rejectsOversizedContextIdentifier() {
        try {
            SecurityEvent(1, SecurityEvent.Type.REPLAY_ACCEPTED, 0, "x".repeat(129), null, null,
                SecurityEvent.Outcome.ACCEPTED)
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected oversized context identifier")
    }

    @Test
    fun rejectsOversizedBoundedCounter() {
        try {
            SecurityEvent(1, SecurityEvent.Type.REPLAY_ACCEPTED, 0, "ctx", null, null,
                SecurityEvent.Outcome.ACCEPTED, 1_000_001)
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected oversized counter")
    }
}
