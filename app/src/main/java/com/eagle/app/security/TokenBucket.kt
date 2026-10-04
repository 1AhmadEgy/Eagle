package com.eagle.app.security

import java.math.BigInteger

/**
 * Deterministic token-bucket limiter.
 *
 * Time is supplied by the caller in monotonic milliseconds.
 * Tokens are represented as integer microtokens to avoid floating-point decisions.
 */
class TokenBucket(
    private val capacityMicrotokens: Long,
    private val refillMicrotokensPerSecond: Long,
    initialTokensMicrotokens: Long = capacityMicrotokens
) {
    private var tokensMicrotokens: Long = initialTokensMicrotokens
    private var lastRefillMillis: Long? = null

    init {
        require(capacityMicrotokens > 0) { "capacity must be positive" }
        require(refillMicrotokensPerSecond > 0) { "refill rate must be positive" }
        require(initialTokensMicrotokens in 0..capacityMicrotokens) {
            "initial tokens must be within capacity"
        }
    }

    @Synchronized
    fun tryConsume(costMicrotokens: Long, nowMonotonicMillis: Long): Boolean {
        require(costMicrotokens > 0) { "cost must be positive" }
        require(costMicrotokens <= capacityMicrotokens) { "cost exceeds capacity" }

        val previous = lastRefillMillis
        if (previous == null) {
            lastRefillMillis = nowMonotonicMillis
        } else {
            refill(elapsedMillis(nowMonotonicMillis, previous))
            if (nowMonotonicMillis > previous) {
                lastRefillMillis = nowMonotonicMillis
            }
        }

        if (tokensMicrotokens < costMicrotokens) return false
        tokensMicrotokens -= costMicrotokens
        return true
    }

    @Synchronized
    fun availableTokensMicrotokens(nowMonotonicMillis: Long): Long {
        val previous = lastRefillMillis
        if (previous == null) {
            lastRefillMillis = nowMonotonicMillis
        } else {
            refill(elapsedMillis(nowMonotonicMillis, previous))
            if (nowMonotonicMillis > previous) {
                lastRefillMillis = nowMonotonicMillis
            }
        }
        return tokensMicrotokens
    }

    private fun elapsedMillis(now: Long, previous: Long): Long {
        if (now <= previous) return 0L
        return BigInteger.valueOf(now)
            .subtract(BigInteger.valueOf(previous))
            .min(BigInteger.valueOf(Long.MAX_VALUE))
            .longValueExact()
    }

    private fun refill(elapsedMillis: Long) {
        if (elapsedMillis == 0L || tokensMicrotokens == capacityMicrotokens) return

        val product = BigInteger.valueOf(elapsedMillis)
            .multiply(BigInteger.valueOf(refillMicrotokensPerSecond))

        val added = product.divide(BigInteger.valueOf(1000L))
        if (added.signum() <= 0) return

        val current = BigInteger.valueOf(tokensMicrotokens)
        val capacity = BigInteger.valueOf(capacityMicrotokens)
        tokensMicrotokens = current.add(added).min(capacity).longValueExact()
    }
}
