package com.eagle.app.security

import org.junit.Assert.*
import org.junit.Test

class StatisticalBaselineTest {
    @Test fun zScoreFlagsLargeDeviation() {
        val s = StatisticalBaseline.zScore(listOf(100, 100, 100, 100), 200, 3_000)
        assertTrue(s.anomalous)
        assertEquals(100_000, s.scoreMilli)
    }

    @Test fun zScoreDoesNotFlagStableValue() {
        val s = StatisticalBaseline.zScore(listOf(100, 100, 100, 100), 100, 3_000)
        assertFalse(s.anomalous)
        assertEquals(0, s.scoreMilli)
    }

    @Test fun ewmaTracksHistoryWithoutFloatingPoint() {
        val s = StatisticalBaseline.ewma(listOf(100, 100, 100), 110, 3_000)
        assertTrue(s.scoreMilli > 0)
        assertFalse(s.anomalous)
    }

    @Test fun medianMadResistsSingleOutlierInHistory() {
        val s = StatisticalBaseline.medianMad(listOf(100, 100, 100, 1000), 1000, 3_000)
        assertTrue(s.anomalous)
    }

    @Test(expected = IllegalArgumentException::class)
    fun insufficientHistoryRejected() {
        StatisticalBaseline.zScore(listOf(1, 2), 3)
    }

    @Test fun signalIsAdvisoryOnly() {
        val s = StatisticalBaseline.zScore(listOf(10, 10, 10), 100)
        assertEquals(AnomalyMethod.Z_SCORE, s.method)
        assertEquals(SecurityFeature.REJECTION_RATE_BPS, s.feature)
    }
}
