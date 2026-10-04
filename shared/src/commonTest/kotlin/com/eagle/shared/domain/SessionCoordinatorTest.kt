package com.eagle.shared.domain

import com.eagle.shared.core.DeviceIdentity
import com.eagle.shared.core.EagleCore
import com.eagle.shared.core.EagleCoreException
import com.eagle.shared.core.SessionHandle
import com.eagle.shared.core.TrustLevel
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.UnconfinedTestDispatcher
import kotlinx.coroutines.test.runTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs

private object FakeSessionHandle : SessionHandle

private class FakeEagleCore(
    private val beginResult: Result<SessionHandle>,
) : EagleCore {
    override suspend fun registerDevice(alias: String): Result<DeviceIdentity> =
        Result.failure(EagleCoreException.ContractNotReady("register_device"))

    override suspend fun beginSession(peerId: String): Result<SessionHandle> = beginResult

    override suspend fun endSession(handle: SessionHandle): Result<Unit> = Result.success(Unit)

    override suspend fun trustStatus(deviceId: String): Result<TrustLevel> =
        Result.success(TrustLevel.Untrusted)
}

@OptIn(ExperimentalCoroutinesApi::class)
class SessionCoordinatorTest {
    @Test
    fun connect_reaches_active_state_when_core_succeeds() = runTest(UnconfinedTestDispatcher()) {
        val coordinator = SessionCoordinator(
            core = FakeEagleCore(Result.success(FakeSessionHandle)),
            scope = this,
        )

        coordinator.connect("peer-1")

        val state = coordinator.state.value
        assertIs<SessionState.Active>(state)
        assertEquals("peer-1", state.peerId)
        assertEquals(FakeSessionHandle, state.handle)
    }

    @Test
    fun connect_requires_trust_when_core_rejects_device() = runTest(UnconfinedTestDispatcher()) {
        val coordinator = SessionCoordinator(
            core = FakeEagleCore(Result.failure(EagleCoreException.DeviceNotTrusted)),
            scope = this,
        )

        coordinator.connect("peer-1")

        assertEquals(SessionState.NeedsTrust("peer-1"), coordinator.state.value)
    }

    @Test
    fun disconnect_returns_to_idle_after_core_closes_session() = runTest(UnconfinedTestDispatcher()) {
        val coordinator = SessionCoordinator(
            core = FakeEagleCore(Result.success(FakeSessionHandle)),
            scope = this,
        )

        coordinator.connect("peer-1")
        coordinator.disconnect()

        assertEquals(SessionState.Idle, coordinator.state.value)
    }
}
