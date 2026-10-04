package com.eagle.app.security

/**
 * Per-session sliding-window replay guard for authenticated sequence numbers.
 *
 * Call only after the message has been cryptographically authenticated and bound
 * to this session. Create a fresh instance for each newly established session.
 * This class does not authenticate peers or validate wall-clock timestamps.
 */
class ReplayGuard(windowSize: Int = 64) {
    private val windowMask: Long
    private val configuredWindowSize = windowSize

    private var initialized = false
    private var highestSeenSequence = 0L
    private var seenBitmap = 0L

    init {
        require(windowSize in 1..64) { "windowSize must be between 1 and 64" }
        windowMask = if (windowSize == 64) -1L else (1L shl windowSize) - 1L
    }

    enum class Decision {
        ACCEPT,
        DUPLICATE,
        TOO_OLD,
        INVALID_SEQUENCE
    }

    /**
     * Records an authenticated sequence number once.
     *
     * Sequence numbers must be non-negative and scoped to one authenticated session.
     * State updates are atomic with respect to concurrent callers.
     */
    @Synchronized
    fun evaluate(sequenceNumber: Long): Decision {
        if (sequenceNumber < 0L) return Decision.INVALID_SEQUENCE

        if (!initialized) {
            initialized = true
            highestSeenSequence = sequenceNumber
            seenBitmap = 1L
            return Decision.ACCEPT
        }

        if (sequenceNumber > highestSeenSequence) {
            val advance = sequenceNumber - highestSeenSequence
            seenBitmap = if (advance >= configuredWindowSize.toLong()) {
                1L
            } else {
                ((seenBitmap shl advance.toInt()) or 1L) and windowMask
            }
            highestSeenSequence = sequenceNumber
            return Decision.ACCEPT
        }

        val distance = highestSeenSequence - sequenceNumber
        if (distance >= configuredWindowSize.toLong()) return Decision.TOO_OLD

        val bit = 1L shl distance.toInt()
        if ((seenBitmap and bit) != 0L) return Decision.DUPLICATE

        seenBitmap = seenBitmap or bit
        return Decision.ACCEPT
    }
}
