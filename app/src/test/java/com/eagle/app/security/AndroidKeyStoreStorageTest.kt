package com.eagle.app.security

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Test

class AndroidKeyStoreStorageTest {
    @Test
    fun keyHandlePreservesScope() {
        val storage = AndroidKeyStoreStorage()
        val key = StorageKeyHandle.create(10, 20, 3)

        assertEquals(10, key.accountId)
        assertEquals(20, key.deviceId)
        assertEquals(3, key.trustEpoch)
    }

    @Test
    fun aliasIsBoundToAccountDeviceAndEpoch() {
        assertEquals(
            "eagle.storage.10.20.3",
            AndroidKeyStoreStorage.aliasFor(10, 20, 3),
        )
    }

    @Test
    fun aliasRejectsInvalidScope() {
        assertThrows(IllegalArgumentException::class.java) {
            AndroidKeyStoreStorage.aliasFor(0, 20, 1)
        }
        assertThrows(IllegalArgumentException::class.java) {
            AndroidKeyStoreStorage.aliasFor(10, 0, 1)
        }
        assertThrows(IllegalArgumentException::class.java) {
            AndroidKeyStoreStorage.aliasFor(10, 20, -1)
        }
    }

    @Test
    fun payloadDefensivelyCopiesOutputArrays() {
        val iv = ByteArray(AndroidKeyStoreStorage.GCM_IV_BYTES) { 1 }
        val ciphertext = ByteArray(AndroidKeyStoreStorage.GCM_TAG_BYTES + 1) { 2 }
        val payload = EncryptedStoragePayload(iv, ciphertext)

        iv[0] = 9
        ciphertext[0] = 9

        assertEquals(1, payload.copyIv()[0].toInt())
        assertEquals(2, payload.copyCiphertext()[0].toInt())
        assertArrayEquals(ByteArray(AndroidKeyStoreStorage.GCM_IV_BYTES) { 1 }, payload.copyIv())
    }

    @Test
    fun payloadRejectsInvalidIvOrMissingTag() {
        assertThrows(IllegalArgumentException::class.java) {
            EncryptedStoragePayload(ByteArray(11), ByteArray(AndroidKeyStoreStorage.GCM_TAG_BYTES + 1))
        }
        assertThrows(IllegalArgumentException::class.java) {
            EncryptedStoragePayload(
                ByteArray(AndroidKeyStoreStorage.GCM_IV_BYTES),
                ByteArray(AndroidKeyStoreStorage.GCM_TAG_BYTES),
            )
        }
    }
}
