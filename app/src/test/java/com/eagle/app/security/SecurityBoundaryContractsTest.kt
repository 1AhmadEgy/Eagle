package com.eagle.app.security

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotSame
import org.junit.Assert.assertTrue
import org.junit.Test

class SecurityBoundaryContractsTest {

    @Test
    fun publicKeyDescriptorDefensivelyCopiesKeyMaterial() {
        val original = byteArrayOf(1, 2, 3)
        val descriptor = PublicKeyDescriptor("test-key", original)

        original[0] = 9
        assertArrayEquals(byteArrayOf(1, 2, 3), descriptor.encodedKey())

        val returned = descriptor.encodedKey()
        assertNotSame(returned, descriptor.encodedKey())
        returned[1] = 8
        assertArrayEquals(byteArrayOf(1, 2, 3), descriptor.encodedKey())
    }

    @Test
    fun authenticationResultEnforcesDecisionReasonConsistency() {
        assertEquals(
            AuthenticationFailureReason.NONE,
            AuthenticationResult(AuthenticationDecision.ACCEPTED).reason
        )
        assertEquals(
            AuthenticationFailureReason.INVALID_PROOF,
            AuthenticationResult(
                AuthenticationDecision.REJECTED,
                AuthenticationFailureReason.INVALID_PROOF
            ).reason
        )

        try {
            AuthenticationResult(
                AuthenticationDecision.ACCEPTED,
                AuthenticationFailureReason.INVALID_PROOF
            )
        } catch (_: IllegalArgumentException) {
            return
        }
        throw AssertionError("Expected invalid accepted/reason combination")
    }

    @Test
    fun identityContractContainsOnlyPublicIdentityMaterial() {
        val identity = DeviceIdentity(
            identityId = IdentityId("device-1"),
            keyVersion = 1,
            publicKey = PublicKeyDescriptor("test-key", byteArrayOf(4, 5, 6)),
            status = IdentityStatus.ACTIVE
        )

        assertEquals(IdentityStatus.ACTIVE, identity.status)
        assertEquals("device-1", identity.identityId.value)
        assertTrue(identity.publicKey.encodedKey().isNotEmpty())
    }

    @Test
    fun keyAndStorageBoundariesUseOpaqueReferences() {
        val key = KeyHandle("key-1")
        val record = SecureRecordId("record-1")

        assertEquals("key-1", key.value)
        assertEquals("record-1", record.value)
        assertEquals(KeyPurpose.IDENTITY_AUTHENTICATION, KeyPurpose.valueOf("IDENTITY_AUTHENTICATION"))
        assertEquals(SecureRecordType.PROTOCOL_STATE, SecureRecordType.valueOf("PROTOCOL_STATE"))
    }
}
