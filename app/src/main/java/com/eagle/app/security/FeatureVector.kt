package com.eagle.app.security

import java.math.BigInteger

/**
 * Deterministic feature extraction over a bounded, ordered SecurityEvent window.
 *
 * Rates are represented without floating-point:
 * - eventsPerMinuteMicros: events/minute multiplied by 1,000,000
 * - rate fields: basis points (0..10,000)
 */
data class FeatureVector(
    val schemaVersion: Int,
    val windowDurationMillis: Long,
    val eventCount: Int,
    val eventsPerMinuteMicros: Long,
    val rejectionRateBps: Int,
    val replayFailureRateBps: Int,
    val rateLimitRateBps: Int,
    val sessionTransitionRejectRateBps: Int
) {
    init {
        require(schemaVersion > 0) { "schemaVersion must be positive" }
        require(windowDurationMillis > 0) { "window duration must be positive" }
        require(eventCount in 0..SecurityFeatureExtractor.MAX_EVENTS_PER_WINDOW)
        require(eventsPerMinuteMicros >= 0)
        require(rejectionRateBps in 0..10_000)
        require(replayFailureRateBps in 0..10_000)
        require(rateLimitRateBps in 0..10_000)
        require(sessionTransitionRejectRateBps in 0..10_000)
    }
}

object SecurityFeatureExtractor {
    const val SCHEMA_VERSION = 1
    const val MAX_EVENTS_PER_WINDOW = 10_000
    private const val BASIS_POINTS = 10_000L
    private const val MICROS_PER_MINUTE = 60_000_000L

    fun extract(
        events: List<SecurityEvent>,
        windowStartMillis: Long,
        windowEndMillis: Long
    ): FeatureVector {
        require(windowStartMillis >= 0) { "window start must be non-negative" }
        require(windowEndMillis > windowStartMillis) { "window must have positive duration" }
        require(events.size <= MAX_EVENTS_PER_WINDOW) { "event window is too large" }

        var previousTimestamp = windowStartMillis
        for (event in events) {
            require(event.schemaVersion == SCHEMA_VERSION) { "unsupported event schema version" }
            require(event.monotonicTimestampMillis in windowStartMillis..windowEndMillis) {
                "event timestamp is outside the feature window"
            }
            require(event.monotonicTimestampMillis >= previousTimestamp) {
                "events must be ordered by monotonic timestamp"
            }
            previousTimestamp = event.monotonicTimestampMillis
        }

        val count = events.size
        val rejected = events.count { it.outcome != SecurityEvent.Outcome.ACCEPTED }
        val replayFailures = events.count {
            it.type == SecurityEvent.Type.REPLAY_DUPLICATE ||
                it.type == SecurityEvent.Type.REPLAY_TOO_OLD ||
                it.type == SecurityEvent.Type.REPLAY_INVALID_SEQUENCE
        }
        val rateLimited = events.count {
            it.type == SecurityEvent.Type.RATE_LIMIT_LIMITED
        }
        val sessionRejects = events.count {
            it.type == SecurityEvent.Type.SESSION_TRANSITION_REJECTED
        }

        return FeatureVector(
            schemaVersion = SCHEMA_VERSION,
            windowDurationMillis = windowEndMillis - windowStartMillis,
            eventCount = count,
            eventsPerMinuteMicros = scaledRate(count.toLong(), MICROS_PER_MINUTE, windowEndMillis - windowStartMillis),
            rejectionRateBps = scaledRate(rejected.toLong(), BASIS_POINTS, count).toInt(),
            replayFailureRateBps = scaledRate(replayFailures.toLong(), BASIS_POINTS, count).toInt(),
            rateLimitRateBps = scaledRate(rateLimited.toLong(), BASIS_POINTS, count).toInt(),
            sessionTransitionRejectRateBps = scaledRate(sessionRejects.toLong(), BASIS_POINTS, count).toInt()
        )
    }

    private fun scaledRate(numerator: Long, scale: Long, denominator: Long): Long {
        if (numerator == 0L || denominator == 0L) return 0L
        return BigInteger.valueOf(numerator)
            .multiply(BigInteger.valueOf(scale))
            .divide(BigInteger.valueOf(denominator))
            .coerceAtMost(BigInteger.valueOf(Long.MAX_VALUE))
            .longValueExact()
    }

    private fun BigInteger.coerceAtMost(max: BigInteger): BigInteger =
        if (this > max) max else this
}
