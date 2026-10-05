package com.eagle.app.security

import androidx.test.ext.junit.runners.AndroidJUnit4
import java.security.GeneralSecurityException
import javax.crypto.AEADBadTagException
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class AndroidKeyStoreStorageInstrumentedTest {
    @Test
    fun keystoreRoundTripAndAadBinding() {
        val storage = AndroidKeyStoreStorage()
        val alias = storage.ensureStorageKey(10, 20, 1)
        val plaintext = "eagle-storage-test".encodeToByteArray()
        val aad = "account=10;device=20;epoch=1".encodeToByteArray()

        try {
            val encrypted = storage.encrypt(alias, plaintext, aad)
            val decrypted = storage.decrypt(alias, encrypted, aad)
            assertArrayEquals(plaintext, decrypted)

            var failed = false
            try {
                storage.decrypt(
                    alias,
                    encrypted,
                    "account=10;device=21;epoch=1".encodeToByteArray(),
                )
            } catch (_: AEADBadTagException) {
                failed = true
            } catch (_: GeneralSecurityException) {
                failed = true
            }
            assertTrue("AAD mismatch must fail authentication", failed)
        } finally {
            storage.deleteKey(alias)
        }
    }
}