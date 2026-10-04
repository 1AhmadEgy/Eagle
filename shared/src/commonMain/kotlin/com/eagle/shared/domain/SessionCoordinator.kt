package com.eagle.shared.domain

import com.eagle.shared.core.EagleCore
import com.eagle.shared.core.EagleCoreException
import com.eagle.shared.core.SessionHandle
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

public class SessionCoordinator(
    private val core: EagleCore,
    private val scope: CoroutineScope,
) {
    private val _state = MutableStateFlow<SessionState>(SessionState.Idle)
    public val state: StateFlow<SessionState> = _state.asStateFlow()

    public fun connect(peerId: String) {
        scope.launch {
            _state.value = SessionState.Connecting(peerId)

            try {
                core.beginSession(peerId).fold(
                    onSuccess = { handle ->
                        _state.value = SessionState.Active(peerId, handle)
                    },
                    onFailure = { error ->
                        _state.value = error.toState(peerId)
                    },
                )
            } catch (error: Throwable) {
                _state.value = error.toState(peerId)
            }
        }
    }

    public fun disconnect() {
        val current = _state.value
        if (current !is SessionState.Active) return

        scope.launch {
            core.endSession(current.handle)
                .onSuccess { _state.value = SessionState.Idle }
                .onFailure { _state.value = SessionState.Failed(it) }
        }
    }

    private fun Throwable.toState(peerId: String): SessionState =
        when (this) {
            EagleCoreException.DeviceNotTrusted -> SessionState.NeedsTrust(peerId)
            else -> SessionState.Failed(this)
        }
}

public sealed interface SessionState {
    public data object Idle : SessionState

    public data class Connecting(
        val peerId: String,
    ) : SessionState

    public data class NeedsTrust(
        val peerId: String,
    ) : SessionState

    public data class Active(
        val peerId: String,
        val handle: SessionHandle,
    ) : SessionState

    public data class Failed(
        val error: Throwable,
    ) : SessionState
}
