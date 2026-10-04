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

enum class AnomalyScoreSemantics {
    ABSOLUTE_Z_SCORE_X1000,
    RELATIVE_DEVIATION_X1000,
    ABSOLUTE_MAD_UNITS_X1000
}

data class AnomalySignal(
    val schemaVersion: Int,
    val feature: SecurityFeature,
    val method: AnomalyMethod,
    val scoreSemantics: AnomalyScoreSemantics,
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
    const val MAX_HISTORY = 10_000

    fun zScore(
        feature: SecurityFeature,
        history: List<Long>,
        current: Long,
        thresholdMilli: Int = DEFAULT_THRESHOLD_MILLI
    ): AnomalySignal {
        requireValid(history, current, thresholdMilli)
        val size = BigInteger.valueOf(history.size.toLong())
        val sum = history.fold(BigInteger.ZERO) { acc, value ->
            acc.add(BigInteger.valueOf(value))
        }
        val mean = sum.divide(size)
        val varianceNumerator = history.fold(BigInteger.ZERO) { acc, value ->
            val d = BigInteger.valueOf(value).subtract(mean)
            acc.add(d.multiply(d))
        }
        val variance = varianceNumerator.divide(size)
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

        return signal(
            feature = feature,
            method = AnomalyMethod.Z_SCORE,
            semantics = AnomalyScoreSemantics.ABSOLUTE_Z_SCORE_X1000,
            score = score,
            threshold = thresholdMilli,
            evidence = "integer z-score; threshold=" + thresholdMilli + "/1000"
        )
    }

    fun ewma(
        feature: SecurityFeature,
        history: List<Long>,
        current: Long,
        thresholdMilli: Int = DEFAULT_THRESHOLD_MILLI
    ): AnomalySignal {
        requireValid(history, current, thresholdMilli)
        var baseline = history.first()
        for (value in history.drop(1)) {
            val delta = BigInteger.valueOf(value).subtract(BigInteger.valueOf(baseline))
            baseline = BigInteger.valueOf(baseline)
                .add(
                    delta.multiply(BigInteger.valueOf(EWMA_ALPHA_MILLI.toLong()))
                        .divide(BigInteger.valueOf(1_000L))
                )
                .longValueExact()
        }

        val deviation = absDiff(current, baseline)
        val scale = max(1L, baseline)
        val score = min(
            100_000L,
            BigInteger.valueOf(deviation)
                .multiply(BigInteger.valueOf(1_000L))
                .divide(BigInteger.valueOf(scale))
                .longValueExact()
        ).toInt()

        return signal(
            feature = feature,
            method = AnomalyMethod.EWMA,
            semantics = AnomalyScoreSemantics.RELATIVE_DEVIATION_X1000,
            score = score,
            threshold = thresholdMilli,
            evidence = "EWMA alpha=" + EWMA_ALPHA_MILLI + "/1000; relative-deviation score"
        )
    }

    fun medianMad(
        feature: SecurityFeature,
        history: List<Long>,
        current: Long,
        thresholdMilli: Int = DEFAULT_THRESHOLD_MILLI
    ): AnomalySignal {
        requireValid(history, current, thresholdMilli)
        val sorted = history.sorted()
        val median = sorted[sorted.size / 2]
        val deviations = sorted.map { absDiff(it, median) }.sorted()
        val mad = deviations[deviations.size / 2]

        val score = if (mad == 0L) {
            if (current == median) 0 else 100_000
        } else {
            min(
                100_000L,
                BigInteger.valueOf(absDiff(current, median))
                    .multiply(BigInteger.valueOf(1_000L))
                    .divide(BigInteger.valueOf(mad))
                    .longValueExact()
            ).toInt()
        }

        return signal(
            feature = feature,
            method = AnomalyMethod.MEDIAN_MAD,
            semantics = AnomalyScoreSemantics.ABSOLUTE_MAD_UNITS_X1000,
            score = score,
            threshold = thresholdMilli,
            evidence = "median/MAD robust deviation; threshold=" + thresholdMilli + "/1000"
        )
    }

    private fun requireValid(history: List<Long>, current: Long, thresholdMilli: Int) {
        require(history.size >= MIN_HISTORY) {
            "at least " + MIN_HISTORY + " baseline samples are required"
        }
        require(history.size <= MAX_HISTORY) { "baseline history is too large" }
        require(history.all { it >= 0 }) { "baseline samples must be non-negative" }
        require(current >= 0) { "current value must be non-negative" }
        require(thresholdMilli in 0..100_000)
    }

    private fun signal(
        feature: SecurityFeature,
        method: AnomalyMethod,
        semantics: AnomalyScoreSemantics,
        score: Int,
        threshold: Int,
        evidence: String
    ) = AnomalySignal(
        schemaVersion = SCHEMA_VERSION,
        feature = feature,
        method = method,
        scoreSemantics = semantics,
        scoreMilli = score,
        thresholdMilli = threshold,
        anomalous = score >= threshold,
        evidence = evidence
    )

    private fun absDiff(a: Long, b: Long): Long {
        require(a >= 0 && b >= 0) { "values must be non-negative" }
        return if (a >= b) a - b else b - a
    }

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
