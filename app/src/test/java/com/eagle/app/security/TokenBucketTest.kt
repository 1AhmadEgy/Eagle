package com.eagle.app.security

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TokenBucketTest {

    @Test
    fun exactCostIsAllowed() {
        val bucket = TokenBucket(10_000_000, 1_000_000)
        assertTrue(bucket.tryConsume(10_000_000, 0))
        assertEquals(0L, bucket.availableTokensMicrotokens(0))
    }

    @Test
    fun insufficientTokensAreLimitedWithoutBalanceChange() {
        val bucket = TokenBucket(10_000_000, 1_000_000, 3_000_000)
        assertFalse(bucket.tryConsume(4_000_000, 0))
        assertEquals(3_000_000L, bucket.availableTokensMicrotokens(0))
    }

    @Test
    fun refillAllowsAfterElapsedTime() {
        val bucket = TokenBucket(10_000_000, 2_000_000, 3_000_000)
        assertTrue(bucket.tryConsume(4_000_000, 1_000))
        assertEquals(1_000_000L, bucket.availableTokensMicrotokens(1_000))
    }

    @Test
    fun refillClampsAtCapacity() {
        val bucket = TokenBucket(10_000_000, 100_000_000)
        assertEquals(10_000_000L, bucket.availableTokensMicrotokens(1_000))
    }

    @Test
    fun backwardClockCannotCreateTokens() {
        val bucket = TokenBucket(10_000_000, 10_000_000, 3_000_000)
        assertTrue(bucket.tryConsume(3_000_000, 1_000))
        assertEquals(0L, bucket.availableTokensMicrotokens(900))
    }

    @Test
    fun fractionalSecondRefillIsRepresentable() {
        val bucket = TokenBucket(10_000_000, 2_000_000, 0)
        assertEquals(0L, bucket.availableTokensMicrotokens(0))
        assertEquals(500_000L, bucket.availableTokensMicrotokens(250))
    }

    @Test
    fun oversizedCostIsRejectedAsInvalidInput() {
        val bucket = TokenBucket(5_000_000, 1_000_000)
        try {
            bucket.tryConsume(5_000_001, 0)
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected invalid cost")
    }

    @Test
    fun overflowScaleDoesNotWrapIntoPositiveTokens() {
        val bucket = TokenBucket(
            capacityMicrotokens = 1_000_000,
            refillMicrotokensPerSecond = Long.MAX_VALUE,
            initialTokensMicrotokens = 0
        )
        assertEquals(0L, bucket.availableTokensMicrotokens(0))
        assertEquals(1_000_000L, bucket.availableTokensMicrotokens(1_000_000))
    }

    @Test
    fun backwardClockDoesNotMoveRefillAnchorBackward() {
        val bucket = TokenBucket(10_000_000, 1_000_000, 0)
        assertEquals(0L, bucket.availableTokensMicrotokens(1_000))
        assertEquals(0L, bucket.availableTokensMicrotokens(900))
        assertEquals(500_000L, bucket.availableTokensMicrotokens(1_500))
    }
}
