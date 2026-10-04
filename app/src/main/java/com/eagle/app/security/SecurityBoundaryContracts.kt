package com.eagle.app.security

/**
 * Platform-neutral contracts for device identity and peer authentication.
 *
 * These contracts deliberately describe capabilities and data flow, not a specific
 * cryptographic algorithm or wire protocol. Implementations must keep private keys
 * behind the KeyManagementBoundary and must not expose them to UI/application code.
 */

@JvmInline
value class IdentityId(val value: String) {
    init {
        require(value.isNotBlank()) { "identity id must not be blank" }
        require(value.length <= MAX_LENGTH) { "identity id is too long" }
    }

    companion object {
        const val MAX_LENGTH = 128
    }
}

@JvmInline
value class KeyHandle(val value: String) {
    init {
        require(value.isNotBlank()) { "key handle must not be blank" }
        require(value.length <= MAX_LENGTH) { "key handle is too long" }
    }

    companion object {
        const val MAX_LENGTH = 128
    }
}

@JvmInline
value class SecureRecordId(val value: String) {
    init {
        require(value.isNotBlank()) { "record id must not be blank" }
        require(value.length <= MAX_LENGTH) { "record id is too long" }
    }

    companion object {
        const val MAX_LENGTH = 128
    }
}

/**
 * Public-key metadata only. Private key bytes must never appear in this type.
 */
class PublicKeyDescriptor(
    val algorithmId: String,
    encodedKey: ByteArray
) {
    private val encodedKeyCopy = encodedKey.clone()

    init {
        require(algorithmId.isNotBlank()) { "algorithm id must not be blank" }
        require(algorithmId.length <= MAX_ALGORITHM_LENGTH) {
            "algorithm id is too long"
        }
        require(encodedKeyCopy.isNotEmpty()) { "encoded public key must not be empty" }
        require(encodedKeyCopy.size <= MAX_PUBLIC_KEY_BYTES) {
            "encoded public key is too large"
        }
    }

    fun encodedKey(): ByteArray = encodedKeyCopy.clone()

    companion object {
        const val MAX_ALGORITHM_LENGTH = 64
        const val MAX_PUBLIC_KEY_BYTES = 4096
    }
}

enum class IdentityStatus {
    ACTIVE,
    REVOKED,
    RETIRED
}

data class DeviceIdentity(
    val identityId: IdentityId,
    val keyVersion: Int,
    val publicKey: PublicKeyDescriptor,
    val status: IdentityStatus
) {
    init {
        require(keyVersion > 0) { "key version must be positive" }
    }
}

interface IdentityContract {
    fun currentIdentity(): DeviceIdentity?
}

data class AuthenticationContext(
    val contextId: String,
    val expectedSessionState: SessionStateMachine.State
) {
    init {
        require(contextId.isNotBlank()) { "authentication context id must not be blank" }
        require(contextId.length <= 128) { "authentication context id is too long" }
    }
}

/**
 * Opaque proof bytes supplied by the selected authentication/protocol implementation.
 * The contract intentionally does not assign meaning to the bytes.
 */
class AuthenticationProof(encodedProof: ByteArray) {
    private val encodedProofCopy = encodedProof.clone()

    init {
        require(encodedProofCopy.isNotEmpty()) { "authentication proof must not be empty" }
        require(encodedProofCopy.size <= MAX_PROOF_BYTES) {
            "authentication proof is too large"
        }
    }

    fun encoded(): ByteArray = encodedProofCopy.clone()

    companion object {
        const val MAX_PROOF_BYTES = 16 * 1024
    }
}

enum class AuthenticationDecision {
    ACCEPTED,
    REJECTED
}

enum class AuthenticationFailureReason {
    NONE,
    INVALID_IDENTITY,
    INVALID_PROOF,
    REVOKED_IDENTITY,
    CONTEXT_MISMATCH,
    UNSUPPORTED
}

data class AuthenticationResult(
    val decision: AuthenticationDecision,
    val reason: AuthenticationFailureReason = AuthenticationFailureReason.NONE
) {
    init {
        require(
            (decision == AuthenticationDecision.ACCEPTED &&
                reason == AuthenticationFailureReason.NONE) ||
                (decision == AuthenticationDecision.REJECTED &&
                    reason != AuthenticationFailureReason.NONE)
        ) {
            "authentication result decision/reason combination is invalid"
        }
    }
}

data class AuthenticationRequest(
    val peer: DeviceIdentity,
    val proof: AuthenticationProof,
    val context: AuthenticationContext
)

interface AuthenticationContract {
    fun authenticate(request: AuthenticationRequest): AuthenticationResult
}

/**
 * Capability boundaries for cryptography, key lifecycle, and secure local storage.
 *
 * No method exposes private-key bytes. Concrete providers may be backed by Rust,
 * Android/JVM, Apple, desktop OS facilities, or another vetted implementation.
 */

enum class KeyPurpose {
    IDENTITY_AUTHENTICATION,
    PREKEY,
    SESSION
}

enum class KeyStatus {
    ACTIVE,
    REVOKED,
    RETIRED
}

data class KeyMetadata(
    val handle: KeyHandle,
    val purpose: KeyPurpose,
    val version: Int,
    val status: KeyStatus
) {
    init {
        require(version > 0) { "key version must be positive" }
    }
}

interface KeyManagementBoundary {
    fun createIdentityKey(): KeyMetadata

    /**
     * Return public-key material without exposing the private key.
     */
    fun publicKey(handle: KeyHandle): PublicKeyDescriptor

    /**
     * Rotate the active identity key. The old key remains addressable until retired/revoked.
     */
    fun rotateIdentityKey(): KeyMetadata

    fun revoke(handle: KeyHandle): Boolean

    fun retire(handle: KeyHandle): Boolean
}

/**
 * Crypto is intentionally capability-oriented. The selected protocol determines the
 * concrete suite, transcript rules, nonce construction, and ratcheting semantics.
 */
interface CryptoBoundary {
    fun verify(
        publicKey: PublicKeyDescriptor,
        message: ByteArray,
        signature: ByteArray
    ): Boolean

    /**
     * Derive an opaque session-key handle from authenticated key material.
     */
    fun deriveSessionKey(
        localKey: KeyHandle,
        peerPublicKey: PublicKeyDescriptor,
        context: ByteArray
    ): KeyHandle

    fun encrypt(
        sessionKey: KeyHandle,
        plaintext: ByteArray,
        associatedData: ByteArray
    ): ByteArray

    fun decrypt(
        sessionKey: KeyHandle,
        ciphertext: ByteArray,
        associatedData: ByteArray
    ): ByteArray
}

enum class SecureRecordType {
    ENCRYPTED_MESSAGE,
    SESSION_STATE,
    DEVICE_METADATA,
    PROTOCOL_STATE,
    APPLICATION_METADATA
}

interface SecureStorageBoundary {
    /**
     * Persist opaque/encrypted records.
     *
     * Implementations must not treat this interface as a raw private-key repository.
     * Private-key lifecycle belongs to KeyManagementBoundary.
     */
    fun put(
        id: SecureRecordId,
        type: SecureRecordType,
        value: ByteArray
    )

    fun get(id: SecureRecordId, type: SecureRecordType): ByteArray?

    fun delete(id: SecureRecordId, type: SecureRecordType): Boolean
}
