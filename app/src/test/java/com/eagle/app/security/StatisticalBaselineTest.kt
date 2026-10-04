package com.eagle.app.security

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class StatisticalBaselineTest {

    private val feature = SecurityFeature.REJECTION_RATE_BPS

    @Test
    fun zScoreFlagsLargeDeviation() {
        val s = StatisticalBaseline.zScore(feature, listOf(100, 100, 100, 100), 200, 3_000)
        assertTrue(s.anomalous)
        assertEquals(100_000, s.scoreMilli)
        assertEquals(AnomalyScoreSemantics.ABSOLUTE_Z_SCORE_X1000, s.scoreSemantics)
        assertEquals(feature, s.feature)
    }

    @Test
    fun zScoreDoesNotFlagStableValue() {
        val s = StatisticalBaseline.zScore(feature, listOf(100, 100, 100, 100), 100, 3_000)
        assertFalse(s.anomalous)
        assertEquals(0, s.scoreMilli)
    }

    @Test
    fun ewmaTracksHistoryWithoutFloatingPoint() {
        val s = StatisticalBaseline.ewma(feature, listOf(100, 100, 100), 110, 3_000)
        assertTrue(s.scoreMilli > 0)
        assertFalse(s.anomalous)
        assertEquals(AnomalyScoreSemantics.RELATIVE_DEVIATION_X1000, s.scoreSemantics)
    }

    @Test
    fun medianMadResistsSingleOutlierInHistory() {
        val s = StatisticalBaseline.medianMad(feature, listOf(100, 100, 100, 1000), 1000, 3_000)
        assertTrue(s.anomalous)
        assertEquals(AnomalyScoreSemantics.ABSOLUTE_MAD_UNITS_X1000, s.scoreSemantics)
    }

    @Test(expected = IllegalArgumentException::class)
    fun insufficientHistoryRejected() {
        StatisticalBaseline.zScore(feature, listOf(1, 2), 3)
    }

    @Test(expected = IllegalArgumentException::class)
    fun negativeHistoryRejectedBeforeAbsoluteDifference() {
        StatisticalBaseline.medianMad(feature, listOf(Long.MIN_VALUE, 1, 1), 1)
    }

    @Test
    fun maximumNonNegativeValueIsStable() {
        val max = Long.MAX_VALUE
        val s = StatisticalBaseline.zScore(feature, listOf(max, max, max), max)
        assertFalse(s.anomalous)
        assertEquals(0, s.scoreMilli)
    }

    @Test
    fun featureIdentityIsNotImplicit() {
        val s = StatisticalBaseline.zScore(
            SecurityFeature.REPLAY_FAILURE_RATE_BPS,
            listOf(100, 100, 100),
            200
        )
        assertEquals(SecurityFeature.REPLAY_FAILURE_RATE_BPS, s.feature)
    }

    @Test
    fun vectorLookupUsesExplicitFeature() {
        val vector = FeatureVector(
            schemaVersion = 1,
            windowDurationMillis = 60_000,
            eventCount = 10,
            eventsPerMinuteMicros = 166_000,
            rejectionRateBps = 1_000,
            replayFailureRateBps = 2_000,
            rateLimitRateBps = 3_000,
            sessionTransitionRejectRateBps = 4_000
        )
        assertEquals(166_000L, vector.value(SecurityFeature.EVENTS_PER_MINUTE_MICROS))
        assertEquals(2_000L, vector.value(SecurityFeature.REPLAY_FAILURE_RATE_BPS))
    }
}
