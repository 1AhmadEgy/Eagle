package com.eagle.shared.core

import kotlin.test.Test
import kotlin.test.assertTrue

class CoreContractTest {
    @Test
    fun session_handle_is_an_opaque_port_type() {
        assertTrue(SessionHandle::class.java.isInterface)
    }

    @Test
    fun trust_level_has_explicit_lifecycle_states() {
        val states: List<TrustLevel> = listOf(
            TrustLevel.Untrusted,
            TrustLevel.Pending,
            TrustLevel.Trusted,
            TrustLevel.Revoked,
            TrustLevel.Replaced,
        )
        assertTrue(states.size == 5)
    }
}
