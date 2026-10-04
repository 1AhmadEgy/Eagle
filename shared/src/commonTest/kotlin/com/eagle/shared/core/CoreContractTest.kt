package com.eagle.shared.core

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

private object TestSessionHandle : SessionHandle

class CoreContractTest {
    @Test
    fun session_handle_is_consumed_only_as_an_opaque_port_type() {
        val handle: SessionHandle = TestSessionHandle
        assertEquals(TestSessionHandle, handle)
        assertTrue(handle is SessionHandle)
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
