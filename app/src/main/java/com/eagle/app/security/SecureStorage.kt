package com.eagle.app.security

import android.content.Context
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import java.io.File
import java.security.KeyStore
import java.security.SecureRandom
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

/**
 * Small authenticated local secret store.
 *
 * The Android Keystore protects the long-term AES key; application data is stored only
 * as nonce || ciphertext in a private file. No plaintext value is persisted.
 */
class SecureStorage(context: Context, private val fileName: String = "eagle_secure_store.bin") {
    private val appContext = context.applicationContext
    private val file: File = File(appContext.filesDir, fileName)

    fun put(value: ByteArray) {
        require(value.size <= MAX_VALUE_BYTES) { "Value too large" }
        val key = getOrCreateKey()
        val nonce = ByteArray(NONCE_BYTES).also(SecureRandom()::nextBytes)
        val ciphertext = cipher(Cipher.ENCRYPT_MODE, key, nonce).doFinal(value)

        val record = ByteArray(1 + NONCE_BYTES + ciphertext.size)
        record[0] = FORMAT_VERSION
        System.arraycopy(nonce, 0, record, 1, NONCE_BYTES)
        System.arraycopy(ciphertext, 0, record, 1 + NONCE_BYTES, ciphertext.size)

        writeAtomically(record)
        nonce.fill(0)
    }

    fun get(): ByteArray? {
        if (!file.exists()) return null
        val record = file.readBytes()
        require(record.size >= 1 + NONCE_BYTES + TAG_BYTES) { "Corrupt secure storage" }
        require(record[0] == FORMAT_VERSION) { "Unsupported secure storage version" }

        val nonce = record.copyOfRange(1, 1 + NONCE_BYTES)
        val ciphertext = record.copyOfRange(1 + NONCE_BYTES, record.size)
        return try {
            cipher(Cipher.DECRYPT_MODE, getExistingKey(), nonce).doFinal(ciphertext)
        } catch (_: Exception) {
            throw SecurityException("Secure storage authentication failed")
        } finally {
            nonce.fill(0)
        }
    }

    fun delete() {
        file.delete()
    }

    private fun writeAtomically(record: ByteArray) {
        val temp = File(appContext.filesDir, "$fileName.tmp")
        temp.outputStream().use { it.write(record); it.fd.sync() }
        if (!temp.renameTo(file)) {
            temp.delete()
            throw IllegalStateException("Unable to commit secure storage")
        }
    }

    private fun getOrCreateKey(): SecretKey {
        val ks = keyStore()
        val existing = ks.getKey(KEY_ALIAS, null)
        if (existing is SecretKey) return existing

        val generator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, ANDROID_KEYSTORE)
        generator.init(
            KeyGenParameterSpec.Builder(
                KEY_ALIAS,
                KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
            )
                .setKeySize(256)
                .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .setRandomizedEncryptionRequired(true)
                .build()
        )
        return generator.generateKey()
    }

    private fun getExistingKey(): SecretKey {
        return keyStore().getKey(KEY_ALIAS, null) as? SecretKey
            ?: throw SecurityException("Secure storage key is unavailable")
    }

    private fun cipher(mode: Int, key: SecretKey, nonce: ByteArray): Cipher =
        Cipher.getInstance("AES/GCM/NoPadding").apply {
            init(mode, key, GCMParameterSpec(TAG_BITS, nonce))
            updateAAD(AAD)
        }

    private fun keyStore(): KeyStore =
        KeyStore.getInstance(ANDROID_KEYSTORE).apply { load(null) }

    private companion object {
        const val ANDROID_KEYSTORE = "AndroidKeyStore"
        const val KEY_ALIAS = "eagle.secure-storage.v1"
        const val FORMAT_VERSION: Byte = 1
        const val NONCE_BYTES = 12
        const val TAG_BITS = 128
        const val MAX_VALUE_BYTES = 1_048_576
        val AAD = "EAGLE-SECURE-STORAGE-V1".toByteArray(Charsets.UTF_8)
    }
}
