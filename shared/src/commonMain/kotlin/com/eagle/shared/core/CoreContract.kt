package com.eagle.shared.core

/**
 * Application-facing port. Implementations live in platform source sets so
 * generated UniFFI bindings never leak into commonMain.
 */
public interface EagleCore {
    public suspend fun registerDevice(alias: String): Result<DeviceIdentity>
    public suspend fun beginSession(peerId: String): Result<SessionHandle>
    public suspend fun endSession(handle: SessionHandle): Result<Unit>
    public suspend fun trustStatus(deviceId: String): Result<TrustLevel>
}

public data class DeviceIdentity(
    public val deviceId: String,
    public val publicKey: ByteArray,
    public val trustLevel: TrustLevel,
)

public sealed interface TrustLevel {
    public data object Untrusted : TrustLevel
    public data object Pending : TrustLevel
    public data object Trusted : TrustLevel
    public data object Revoked : TrustLevel
    public data object Replaced : TrustLevel
}

/**
 * Opaque to application code. Implementations may wrap a generated UniFFI
 * object, but no session internals or key material are exposed here.
 */
public interface SessionHandle

public sealed class EagleCoreException(
    message: String,
) : Exception(message) {
    public data object DeviceNotTrusted : EagleCoreException("device not trusted")
    public data object HandshakeFailed : EagleCoreException("handshake failed")
    public data class CryptoFailure(
        val reason: String,
    ) : EagleCoreException("crypto error")

    public data class InvalidArgument(
        val reason: String,
    ) : EagleCoreException("invalid argument")

    public data class ContractNotReady(
        val operation: String,
    ) : EagleCoreException("contract operation is not implemented yet: $operation")
}
