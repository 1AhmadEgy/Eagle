package com.eagle.app.security

import org.junit.Assert.assertEquals
import org.junit.Test

class SessionStateMachineTest {

    @Test
    fun followsValidLifecycle() {
        val machine = SessionStateMachine()
        assertEquals(SessionStateMachine.State.NEW, machine.state())
        assertEquals(SessionStateMachine.Decision.ACCEPTED,
            machine.transition(SessionStateMachine.State.AUTHENTICATING))
        assertEquals(SessionStateMachine.Decision.ACCEPTED,
            machine.transition(SessionStateMachine.State.ESTABLISHED))
        assertEquals(SessionStateMachine.Decision.ACCEPTED,
            machine.transition(SessionStateMachine.State.REKEYING))
        assertEquals(SessionStateMachine.Decision.ACCEPTED,
            machine.transition(SessionStateMachine.State.ESTABLISHED))
        assertEquals(SessionStateMachine.Decision.ACCEPTED,
            machine.transition(SessionStateMachine.State.CLOSED))
        assertEquals(SessionStateMachine.State.CLOSED, machine.state())
    }

    @Test
    fun rejectsInvalidTransitionWithoutMutation() {
        val machine = SessionStateMachine()
        assertEquals(SessionStateMachine.Decision.REJECTED,
            machine.transition(SessionStateMachine.State.ESTABLISHED))
        assertEquals(SessionStateMachine.State.NEW, machine.state())
    }

    @Test
    fun allowsEarlyClose() {
        val machine = SessionStateMachine()
        assertEquals(SessionStateMachine.Decision.ACCEPTED,
            machine.transition(SessionStateMachine.State.CLOSED))
        assertEquals(SessionStateMachine.Decision.REJECTED,
            machine.transition(SessionStateMachine.State.AUTHENTICATING))
    }

    @Test
    fun rejectsRepeatedSameState() {
        val machine = SessionStateMachine()
        assertEquals(SessionStateMachine.Decision.ACCEPTED,
            machine.transition(SessionStateMachine.State.AUTHENTICATING))
        assertEquals(SessionStateMachine.Decision.REJECTED,
            machine.transition(SessionStateMachine.State.AUTHENTICATING))
    }

    @Test
    fun rejectsRekeyBeforeEstablishment() {
        val machine = SessionStateMachine()
        assertEquals(SessionStateMachine.Decision.ACCEPTED,
            machine.transition(SessionStateMachine.State.AUTHENTICATING))
        assertEquals(SessionStateMachine.Decision.REJECTED,
            machine.transition(SessionStateMachine.State.REKEYING))
        assertEquals(SessionStateMachine.State.AUTHENTICATING, machine.state())
    }
}
