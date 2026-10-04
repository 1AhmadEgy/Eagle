package com.eagle.app.security

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class DeterministicSecurityEngineTest {

    @Test
    fun acceptsFirstAuthenticatedMessageAndConsumesRateBudget() {
        val engine = DeterministicSecurityEngine(
            rateLimiter = TokenBucket(
                capacityMicrotokens = 10,
                refillMicrotokensPerSecond = 10
            )
        )

        val result = engine.evaluateAuthenticatedMessage(
            sequenceNumber = 1,
            costMicrotokens = 4,
            nowMonotonicMillis = 1_000
        )

        assertEquals(
            DeterministicSecurityEngine.Decision.RATE_ALLOWED,
            result.decision
        )
        assertEquals(ReplayGuard.Decision.ACCEPT, result.replayDecision)
        assertTrue(result.rateAllowed == true)
    }

    @Test
    fun rejectsReplayBeforeRateLimiting() {
        val engine = DeterministicSecurityEngine(
            rateLimiter = TokenBucket(
                capacityMicrotokens = 10,
                refillMicrotokensPerSecond = 1
            )
        )

        val first = engine.evaluateAuthenticatedMessage(7, 6, 1_000)
        val replay = engine.evaluateAuthenticatedMessage(7, 6, 1_000)

        assertEquals(
            DeterministicSecurityEngine.Decision.RATE_ALLOWED,
            first.decision
        )
        assertEquals(
            DeterministicSecurityEngine.Decision.REPLAY_REJECTED,
            replay.decision
        )
        assertEquals(ReplayGuard.Decision.DUPLICATE, replay.replayDecision)
        assertFalse(replay.rateAllowed == true)
    }

    @Test
    fun rateLimitDecisionIsDeterministic() {
        val engine = DeterministicSecurityEngine(
            rateLimiter = TokenBucket(
                capacityMicrotokens = 5,
                refillMicrotokensPerSecond = 1
            )
        )

        val first = engine.evaluateAuthenticatedMessage(1, 5, 1_000)
        val second = engine.evaluateAuthenticatedMessage(2, 1, 1_000)

        assertEquals(
            DeterministicSecurityEngine.Decision.RATE_ALLOWED,
            first.decision
        )
        assertEquals(
            DeterministicSecurityEngine.Decision.RATE_LIMITED,
            second.decision
        )
        assertTrue(second.rateAllowed == false)
    }

    @Test
    fun sessionDecisionRemainsIndependentFromMessagePolicy() {
        val engine = DeterministicSecurityEngine(
            rateLimiter = TokenBucket(10, 10)
        )

        val rejected = engine.transition(SessionStateMachine.State.ESTABLISHED)
        val accepted = engine.transition(SessionStateMachine.State.AUTHENTICATING)

        assertEquals(
            DeterministicSecurityEngine.Decision.SESSION_REJECTED,
            rejected.decision
        )
        assertEquals(
            DeterministicSecurityEngine.Decision.SESSION_ACCEPTED,
            accepted.decision
        )
        assertEquals(
            SessionStateMachine.State.AUTHENTICATING,
            accepted.sessionState
        )
    }
}
