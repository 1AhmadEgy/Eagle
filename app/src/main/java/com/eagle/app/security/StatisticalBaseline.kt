package com.eagle.app.security

import java.math.BigInteger
import kotlin.math.max
import kotlin.math.min

enum class SecurityFeature {
    EVENTS_PER_MINUTE_MICROS,
    REJECTION_RATE_BPS,
    REPLAY_FAILURE_RATE_BPS,
    RATE_LIMIT_RATE_BPS,
    SESSION_TRANSITION_REJECT_RATE_BPS
}

enum class AnomalyMethod { Z_SCORE, EWMA, MEDIAN_MAD }

data class AnomalySignal(
    val schemaVersion: Int,
    val feature: SecurityFeature,
    val method: AnomalyMethod,
    val scoreMilli: Int,
    val thresholdMilli: Int,
    val anomalous: Boolean,
    val evidence: String
) {
    init {
        require(schemaVersion > 0)
        require(scoreMilli in 0..100_000)
        require(thresholdMilli in 0..100_000)
        require(evidence.length <= 256)
    }
}

object StatisticalBaseline {
    const val SCHEMA_VERSION = 1
    const val DEFAULT_THRESHOLD_MILLI = 3_000
    const val EWMA_ALPHA_MILLI = 200
    const val MIN_HISTORY = 3

    fun zScore(history: List<Long>, current: Long, thresholdMilli: Int = DEFAULT_THRESHOLD_MILLI): AnomalySignal {
        requireValid(history, current, thresholdMilli)
        val sum = history.fold(BigInteger.ZERO) { acc, value -> acc.add(BigInteger.valueOf(value)) }
        val mean = sum.divide(BigInteger.valueOf(history.size.toLong()))
        val varianceNumerator = history.fold(BigInteger.ZERO) { acc, value ->
            val d = BigInteger.valueOf(value).subtract(mean)
            acc.add(d.multiply(d))
        }
        val variance = varianceNumerator.divide(BigInteger.valueOf(history.size.toLong()))
        val stddev = integerSqrt(variance)
        val deviation = BigInteger.valueOf(current).subtract(mean).abs()
        val score = if (stddev == BigInteger.ZERO) {
            if (deviation == BigInteger.ZERO) 0 else 100_000
        } else {
            deviation.multiply(BigInteger.valueOf(1_000L))
                .divide(stddev)
                .min(BigInteger.valueOf(100_000L))
                .toInt()
        }
        return signal(SecurityFeature.REJECTION_RATE_BPS, AnomalyMethod.Z_SCORE, score, thresholdMilli,
            "integer z-score; threshold=" + thresholdMilli + "/1000")
    }

    fun ewma(history: List<Long>, current: Long, thresholdMilli: Int = DEFAULT_THRESHOLD_MILLI): AnomalySignal {
        requireValid(history, current, thresholdMilli)
        var baseline = history.first()
        for (value in history.drop(1)) {
            val delta = BigInteger.valueOf(value).subtract(BigInteger.valueOf(baseline))
            baseline = BigInteger.valueOf(baseline).add(delta.multiply(BigInteger.valueOf(EWMA_ALPHA_MILLI.toLong()))
                .divide(BigInteger.valueOf(1_000L))).longValueExact()
        }
        val deviation = absDiff(current, baseline)
        val scale = max(1L, absDiff(baseline, 0L))
        val score = min(100_000L, deviation.toBigInteger()
            .multiply(BigInteger.valueOf(1_000L))
            .divide(BigInteger.valueOf(scale))
            .toLong()).toInt()
        return signal(SecurityFeature.REJECTION_RATE_BPS, AnomalyMethod.EWMA, score, thresholdMilli,
            "EWMA alpha=" + EWMA_ALPHA_MILLI + "/1000")
    }

    fun medianMad(history: List<Long>, current: Long, thresholdMilli: Int = DEFAULT_THRESHOLD_MILLI): AnomalySignal {
        requireValid(history, current, thresholdMilli)
        val sorted = history.sorted()
        val median = sorted[sorted.size / 2]
        val deviations = sorted.map { absDiff(it, median) }.sorted()
        val mad = deviations[deviations.size / 2]
        val score = if (mad == 0L) {
            if (current == median) 0 else 100_000
        } else {
            min(100_000L, absDiff(current, median).toBigInteger()
                .multiply(BigInteger.valueOf(1_000L))
                .divide(BigInteger.valueOf(mad))
                .toLong()).toInt()
        }
        return signal(SecurityFeature.REJECTION_RATE_BPS, AnomalyMethod.MEDIAN_MAD, score, thresholdMilli,
            "median/MAD robust deviation; threshold=" + thresholdMilli + "/1000")
    }

    private fun requireValid(history: List<Long>, current: Long, thresholdMilli: Int) {
        require(history.size >= MIN_HISTORY) { "at least " + MIN_HISTORY + " baseline samples are required" }
        require(history.size <= 10_000) { "baseline history is too large" }
        require(current >= 0)
        require(thresholdMilli in 0..100_000)
    }

    private fun signal(feature: SecurityFeature, method: AnomalyMethod, score: Int, threshold: Int, evidence: String) =
        AnomalySignal(SCHEMA_VERSION, feature, method, score, threshold, score >= threshold, evidence)

    private fun absDiff(a: Long, b: Long): Long = if (a >= b) a - b else b - a

    private fun integerSqrt(value: BigInteger): BigInteger {
        require(value.signum() >= 0)
        if (value == BigInteger.ZERO) return BigInteger.ZERO
        var x = BigInteger.ONE.shiftLeft((value.bitLength() + 1) / 2)
        while (true) {
            val next = x.add(value.divide(x)).shiftRight(1)
            if (next >= x) return x
            x = next
        }
    }
}
