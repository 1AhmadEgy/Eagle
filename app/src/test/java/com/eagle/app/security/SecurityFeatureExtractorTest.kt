package com.eagle.app.security

import org.junit.Assert.assertEquals
import org.junit.Test

class SecurityFeatureExtractorTest {

    private fun event(
        type: SecurityEvent.Type,
        outcome: SecurityEvent.Outcome,
        timestamp: Long
    ) = SecurityEvent(1, type, timestamp, "ctx", "peer", SessionStateMachine.State.ESTABLISHED, outcome)

    @Test
    fun extractsDeterministicRates() {
        val events = listOf(
            event(SecurityEvent.Type.REPLAY_ACCEPTED, SecurityEvent.Outcome.ACCEPTED, 1_000),
            event(SecurityEvent.Type.REPLAY_DUPLICATE, SecurityEvent.Outcome.REJECTED, 2_000),
            event(SecurityEvent.Type.RATE_LIMIT_LIMITED, SecurityEvent.Outcome.LIMITED, 3_000),
            event(SecurityEvent.Type.SESSION_TRANSITION_REJECTED, SecurityEvent.Outcome.REJECTED, 4_000)
        )

        val vector = SecurityFeatureExtractor.extract(events, 0, 60_000)

        assertEquals(4, vector.eventCount)
        assertEquals(4_000_000L, vector.eventsPerMinuteMicros)
        assertEquals(7_500, vector.rejectionRateBps)
        assertEquals(2_500, vector.replayFailureRateBps)
        assertEquals(2_500, vector.rateLimitRateBps)
        assertEquals(2_500, vector.sessionTransitionRejectRateBps)
    }

    @Test
    fun emptyWindowProducesZeroRates() {
        val vector = SecurityFeatureExtractor.extract(emptyList(), 0, 60_000)
        assertEquals(0, vector.eventCount)
        assertEquals(0L, vector.eventsPerMinuteMicros)
        assertEquals(0, vector.rejectionRateBps)
    }

    @Test
    fun rejectsUnorderedEvents() {
        val events = listOf(
            event(SecurityEvent.Type.REPLAY_ACCEPTED, SecurityEvent.Outcome.ACCEPTED, 2_000),
            event(SecurityEvent.Type.REPLAY_ACCEPTED, SecurityEvent.Outcome.ACCEPTED, 1_000)
        )
        try {
            SecurityFeatureExtractor.extract(events, 0, 60_000)
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected unordered events to be rejected")
    }

    @Test
    fun rejectsOutsideWindowEvent() {
        val events = listOf(event(SecurityEvent.Type.REPLAY_ACCEPTED, SecurityEvent.Outcome.ACCEPTED, 61_000))
        try {
            SecurityFeatureExtractor.extract(events, 0, 60_000)
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected out-of-window event to be rejected")
    }

    @Test
    fun rejectsOversizedEventWindow() {
        val events = (0..SecurityFeatureExtractor.MAX_EVENTS_PER_WINDOW).map {
            event(SecurityEvent.Type.REPLAY_ACCEPTED, SecurityEvent.Outcome.ACCEPTED, it.toLong())
        }
        try {
            SecurityFeatureExtractor.extract(events, 0, 60_000)
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected oversized event window to be rejected")
    }
}
