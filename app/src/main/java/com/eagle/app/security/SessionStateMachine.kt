package com.eagle.app.security

/**
 * Deterministic lifecycle for one authenticated transport/session context.
 *
 * Security-sensitive protocol code should perform authentication/key validation before
 * entering ESTABLISHED. This state machine does not authenticate peers by itself.
 */
class SessionStateMachine {

    enum class State {
        NEW,
        AUTHENTICATING,
        ESTABLISHED,
        REKEYING,
        CLOSED
    }

    enum class Decision {
        ACCEPTED,
        REJECTED
    }

    private var currentState = State.NEW

    @Synchronized
    fun state(): State = currentState

    /**
     * Allowed transitions:
     * NEW -> AUTHENTICATING | CLOSED
     * AUTHENTICATING -> ESTABLISHED | CLOSED
     * ESTABLISHED -> REKEYING | CLOSED
     * REKEYING -> ESTABLISHED | CLOSED
     * CLOSED -> nowhere
     */
    @Synchronized
    fun transition(target: State): Decision {
        if (target == currentState) return Decision.REJECTED
        if (!isAllowed(currentState, target)) return Decision.REJECTED
        currentState = target
        return Decision.ACCEPTED
    }

    private fun isAllowed(from: State, to: State): Boolean = when (from) {
        State.NEW -> to == State.AUTHENTICATING || to == State.CLOSED
        State.AUTHENTICATING -> to == State.ESTABLISHED || to == State.CLOSED
        State.ESTABLISHED -> to == State.REKEYING || to == State.CLOSED
        State.REKEYING -> to == State.ESTABLISHED || to == State.CLOSED
        State.CLOSED -> false
    }
}
