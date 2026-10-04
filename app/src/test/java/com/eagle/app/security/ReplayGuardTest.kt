package com.eagle.app.security

import org.junit.Assert.assertEquals
import org.junit.Test

class ReplayGuardTest {

    @Test
    fun firstSequenceIsAcceptedAndRepeatIsRejected() {
        val guard = ReplayGuard()
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(0))
        assertEquals(ReplayGuard.Decision.DUPLICATE, guard.evaluate(0))
    }

    @Test
    fun acceptsOutOfOrderSequenceWithinWindowOnce() {
        val guard = ReplayGuard(windowSize = 8)
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(10))
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(8))
        assertEquals(ReplayGuard.Decision.DUPLICATE, guard.evaluate(8))
    }

    @Test
    fun rejectsSequenceOutsideWindow() {
        val guard = ReplayGuard(windowSize = 4)
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(10))
        assertEquals(ReplayGuard.Decision.TOO_OLD, guard.evaluate(6))
    }

    @Test
    fun largeAdvanceClearsPriorWindow() {
        val guard = ReplayGuard(windowSize = 4)
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(1))
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(100))
        assertEquals(ReplayGuard.Decision.TOO_OLD, guard.evaluate(1))
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(99))
    }

    @Test
    fun negativeSequenceIsInvalidAndDoesNotMutateState() {
        val guard = ReplayGuard()
        assertEquals(ReplayGuard.Decision.INVALID_SEQUENCE, guard.evaluate(-1))
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(4))
        assertEquals(ReplayGuard.Decision.DUPLICATE, guard.evaluate(4))
    }

    @Test
    fun supportsFullSixtyFourEntryWindow() {
        val guard = ReplayGuard(windowSize = 64)
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(100))
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(37))
        assertEquals(ReplayGuard.Decision.TOO_OLD, guard.evaluate(36))
    }

    @Test
    fun handlesMaximumLongSequenceWithoutArithmeticWraparound() {
        val guard = ReplayGuard(windowSize = 8)
        assertEquals(ReplayGuard.Decision.ACCEPT, guard.evaluate(Long.MAX_VALUE))
        assertEquals(ReplayGuard.Decision.DUPLICATE, guard.evaluate(Long.MAX_VALUE))
        assertEquals(ReplayGuard.Decision.TOO_OLD, guard.evaluate(0))
    }

    @Test
    fun validatesWindowSize() {
        try {
            ReplayGuard(windowSize = 0)
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected invalid window size")
    }
}
