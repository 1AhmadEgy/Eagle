package com.eagle.app.security

/**
 * Platform-neutral orchestration of the verified security primitives.
 *
 * This is deliberately an orchestration layer, not an authentication or cryptographic
 * implementation. Callers are responsible for authenticating/binding the session before
 * replay evaluation, as required by ReplayGuard.
 */
class DeterministicSecurityEngine(
    private val session: SessionStateMachine = SessionStateMachine(),
    private val replayGuard: ReplayGuard = ReplayGuard(),
    private val rateLimiter: TokenBucket
) {
    enum class Decision {
        SESSION_ACCEPTED,
        SESSION_REJECTED,
        REPLAY_ACCEPTED,
        REPLAY_REJECTED,
        RATE_ALLOWED,
        RATE_LIMITED
    }

    data class Result(
        val decision: Decision,
        val sessionState: SessionStateMachine.State,
        val replayDecision: ReplayGuard.Decision? = null,
        val rateAllowed: Boolean? = null
    )

    fun transition(target: SessionStateMachine.State): Result {
        val decision = session.transition(target)
        return Result(
            decision = if (decision == SessionStateMachine.Decision.ACCEPTED) {
                Decision.SESSION_ACCEPTED
            } else {
                Decision.SESSION_REJECTED
            },
            sessionState = session.state()
        )
    }

    /**
     * Evaluates a message after cryptographic authentication and session binding.
     */
    fun evaluateAuthenticatedMessage(
        sequenceNumber: Long,
        costMicrotokens: Long,
        nowMonotonicMillis: Long
    ): Result {
        val replay = replayGuard.evaluate(sequenceNumber)
        if (replay != ReplayGuard.Decision.ACCEPT) {
            return Result(
                decision = Decision.REPLAY_REJECTED,
                sessionState = session.state(),
                replayDecision = replay
            )
        }

        val allowed = rateLimiter.tryConsume(costMicrotokens, nowMonotonicMillis)
        return Result(
            decision = if (allowed) Decision.RATE_ALLOWED else Decision.RATE_LIMITED,
            sessionState = session.state(),
            replayDecision = replay,
            rateAllowed = allowed
        )
    }
}
