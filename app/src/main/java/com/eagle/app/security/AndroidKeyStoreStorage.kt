package com.eagle.app.security

import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

class EncryptedStoragePayload(
    iv: ByteArray,
    ciphertext: ByteArray,
) {
    private val ivBytes = iv.copyOf()
    private val ciphertextBytes = ciphertext.copyOf()

    init {
        require(ivBytes.size == AndroidKeyStoreStorage.GCM_IV_BYTES) { "invalid GCM IV length" }
        require(ciphertextBytes.size > AndroidKeyStoreStorage.GCM_TAG_BYTES) {
            "ciphertext must include a GCM authentication tag"
        }
    }

    fun copyIv(): ByteArray = ivBytes.copyOf()

    fun copyCiphertext(): ByteArray = ciphertextBytes.copyOf()
}

class AndroidKeyStoreStorage(
    private val keyStore: KeyStore = KeyStore.getInstance(ANDROID_KEYSTORE).apply { load(null) },
) {
    fun ensureStorageKey(
        accountId: Long,
        deviceId: Long,
        trustEpoch: Long,
        requireStrongBox: Boolean = false,
    ): String {
        val alias = aliasFor(accountId, deviceId, trustEpoch)
        if (keyStore.containsAlias(alias)) {
            return alias
        }

        val generator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, ANDROID_KEYSTORE)
        val builder = KeyGenParameterSpec.Builder(
            alias,
            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT,
        )
            .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
            .setKeySize(AES_KEY_BITS)

        if (requireStrongBox) {
            builder.setIsStrongBoxBacked(true)
        }

        generator.init(builder.build())
        generator.generateKey()
        return alias
    }

    fun encrypt(
        alias: String,
        plaintext: ByteArray,
        associatedData: ByteArray = ByteArray(0),
    ): EncryptedStoragePayload {
        require(plaintext.isNotEmpty()) { "plaintext must not be empty" }

        val cipher = Cipher.getInstance(TRANSFORMATION)
        cipher.init(Cipher.ENCRYPT_MODE, requireKey(alias))
        if (associatedData.isNotEmpty()) {
            cipher.updateAAD(associatedData)
        }

        return EncryptedStoragePayload(
            iv = cipher.iv.copyOf(),
            ciphertext = cipher.doFinal(plaintext),
        )
    }

    fun decrypt(
        alias: String,
        payload: EncryptedStoragePayload,
        associatedData: ByteArray = ByteArray(0),
    ): ByteArray {
        val cipher = Cipher.getInstance(TRANSFORMATION)
        cipher.init(
            Cipher.DECRYPT_MODE,
            requireKey(alias),
            GCMParameterSpec(GCM_TAG_BITS, payload.iv),
        )
        if (associatedData.isNotEmpty()) {
            cipher.updateAAD(associatedData)
        }
        return cipher.doFinal(payload.ciphertext)
    }

    fun deleteKey(alias: String) {
        requireKey(alias)
        keyStore.deleteEntry(alias)
    }

    private fun requireKey(alias: String): SecretKey =
        (keyStore.getKey(alias, null) as? SecretKey)
            ?: throw IllegalStateException("storage key is unavailable")

    companion object {
        private const val ANDROID_KEYSTORE = "AndroidKeyStore"
        private const val TRANSFORMATION = "AES/GCM/NoPadding"
        private const val AES_KEY_BITS = 256
        private const val GCM_TAG_BITS = 128
        internal const val GCM_IV_BYTES = 12
        internal const val GCM_TAG_BYTES = 16

        internal fun aliasFor(accountId: Long, deviceId: Long, trustEpoch: Long): String {
            require(accountId > 0) { "accountId must be positive" }
            require(deviceId > 0) { "deviceId must be positive" }
            require(trustEpoch >= 0) { "trustEpoch must not be negative" }
            return "eagle.storage.$accountId.$deviceId.$trustEpoch"
        }
    }
}
