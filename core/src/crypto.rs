//! Fail-closed cryptographic and key-management boundary.
//!
//! This module intentionally does not implement cryptographic primitives.
//! Production crypto must be supplied by an explicitly approved, audited provider.
//! The typed handles prevent accidental cross-domain key use at the core boundary.

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyPurpose {
    DeviceIdentity = 1,
    SignedPreKey = 2,
    OneTimePreKey = 3,
    PostQuantumPreKey = 4,
    Session = 5,
    Message = 6,
    StorageWrapping = 7,
    Recovery = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    ProviderUnavailable,
    UnsupportedOperation,
    RevokedKey,
    DestroyedKey,
    InvalidKeyState,
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ProviderUnavailable => "cryptographic provider unavailable",
            Self::UnsupportedOperation => "cryptographic operation unsupported",
            Self::RevokedKey => "key is revoked",
            Self::DestroyedKey => "key has been destroyed",
            Self::InvalidKeyState => "invalid key state",
        })
    }
}

impl std::error::Error for KeyError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyId([u8; 16]);

impl KeyId {
    pub const fn new(bytes: [u8; 16]) -> Self { Self(bytes) }
    pub const fn as_bytes(&self) -> &[u8; 16] { &self.0 }
}

macro_rules! typed_key_handle {
    ($name:ident, $purpose:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(KeyId);

        impl $name {
            pub const fn from_id(id: KeyId) -> Self { Self(id) }
            pub const fn id(&self) -> KeyId { self.0 }
            pub const fn purpose(&self) -> KeyPurpose { $purpose }
        }
    };
}

typed_key_handle!(IdentityKeyHandle, KeyPurpose::DeviceIdentity);
typed_key_handle!(SignedPreKeyHandle, KeyPurpose::SignedPreKey);
typed_key_handle!(OneTimePreKeyHandle, KeyPurpose::OneTimePreKey);
typed_key_handle!(PostQuantumPreKeyHandle, KeyPurpose::PostQuantumPreKey);
typed_key_handle!(SessionKeyHandle, KeyPurpose::Session);
typed_key_handle!(MessageKeyHandle, KeyPurpose::Message);
typed_key_handle!(StorageWrappingKeyHandle, KeyPurpose::StorageWrapping);
typed_key_handle!(RecoveryKeyHandle, KeyPurpose::Recovery);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderCapabilities {
    pub identity_keys_non_exportable: bool,
    pub hardware_protection: bool,
    pub pq_kem: bool,
    pub message_ratchet: bool,
}

impl ProviderCapabilities {
    pub const fn satisfies_required_messaging_profile(&self) -> bool {
        self.identity_keys_non_exportable && self.pq_kem && self.message_ratchet
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl ProviderVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderRevision([u8; 20]);

impl ProviderRevision {
    pub const fn new(bytes: [u8; 20]) -> Self { Self(bytes) }
    pub const fn bytes(&self) -> &[u8; 20] { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderApproval {
    pub version: ProviderVersion,
    pub revision: ProviderRevision,
    pub license_reviewed: bool,
    pub support_reviewed: bool,
    pub platform_reviewed: bool,
    pub conformance_verified: bool,
    pub independent_reviewed: bool,
}

impl ProviderApproval {
    pub const fn rejected() -> Self {
        Self {
            version: ProviderVersion::new(0, 0, 0),
            revision: ProviderRevision::new([0; 20]),
            license_reviewed: false,
            support_reviewed: false,
            platform_reviewed: false,
            conformance_verified: false,
            independent_reviewed: false,
        }
    }

    pub const fn is_production_approved(&self) -> bool {
        (self.version.major != 0 || self.version.minor != 0 || self.version.patch != 0)
            && self.revision.0 != [0; 20]
            && self.license_reviewed
            && self.support_reviewed
            && self.platform_reviewed
            && self.conformance_verified
            && self.independent_reviewed
    }

    pub const fn version(&self) -> ProviderVersion { self.version }
    pub const fn revision(&self) -> ProviderRevision { self.revision }

    pub const fn is_production_approved_with_capabilities(
        &self,
        capabilities: ProviderCapabilities,
    ) -> bool {
        self.is_production_approved() && capabilities.satisfies_required_messaging_profile()
    }
}

pub trait KeyStore {
    type Error;

    fn generate_identity_key(&mut self) -> Result<IdentityKeyHandle, Self::Error>;
    fn generate_signed_pre_key(&mut self) -> Result<SignedPreKeyHandle, Self::Error>;
    fn generate_one_time_pre_key(&mut self) -> Result<OneTimePreKeyHandle, Self::Error>;
    fn generate_post_quantum_pre_key(&mut self) -> Result<PostQuantumPreKeyHandle, Self::Error>;
    fn generate_storage_wrapping_key(&mut self) -> Result<StorageWrappingKeyHandle, Self::Error>;
    fn generate_recovery_key(&mut self) -> Result<RecoveryKeyHandle, Self::Error>;
    fn revoke_identity_key(&mut self, key: IdentityKeyHandle) -> Result<(), Self::Error>;
    fn destroy_storage_wrapping_key(&mut self, key: StorageWrappingKeyHandle)
        -> Result<(), Self::Error>;
}

pub trait MessagingCrypto {
    type Error;

    fn establish_session(
        &mut self,
        local_identity: IdentityKeyHandle,
        remote_identity_public: &[u8],
        signed_pre_key: SignedPreKeyHandle,
        one_time_pre_key: Option<OneTimePreKeyHandle>,
        post_quantum_pre_key: Option<PostQuantumPreKeyHandle>,
    ) -> Result<SessionKeyHandle, Self::Error>;

    fn next_message_key(
        &mut self,
        session: SessionKeyHandle,
    ) -> Result<MessageKeyHandle, Self::Error>;

    fn encrypt_message(
        &mut self,
        message_key: MessageKeyHandle,
        plaintext: &[u8],
        associated_data: &[u8],
    ) -> Result<Vec<u8>, Self::Error>;

    fn decrypt_message(
        &mut self,
        message_key: MessageKeyHandle,
        ciphertext: &[u8],
        associated_data: &[u8],
    ) -> Result<Vec<u8>, Self::Error>;

    fn revoke_session(&mut self, session: SessionKeyHandle) -> Result<(), Self::Error>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct UnavailableCryptoProvider;

impl KeyStore for UnavailableCryptoProvider {
    type Error = KeyError;

    fn generate_identity_key(&mut self) -> Result<IdentityKeyHandle, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }
    fn generate_signed_pre_key(&mut self) -> Result<SignedPreKeyHandle, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }
    fn generate_one_time_pre_key(&mut self) -> Result<OneTimePreKeyHandle, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }
    fn generate_post_quantum_pre_key(&mut self) -> Result<PostQuantumPreKeyHandle, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }
    fn generate_storage_wrapping_key(&mut self) -> Result<StorageWrappingKeyHandle, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }
    fn generate_recovery_key(&mut self) -> Result<RecoveryKeyHandle, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }
    fn revoke_identity_key(&mut self, _key: IdentityKeyHandle) -> Result<(), Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }
    fn destroy_storage_wrapping_key(
        &mut self,
        _key: StorageWrappingKeyHandle,
    ) -> Result<(), Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }
}

impl MessagingCrypto for UnavailableCryptoProvider {
    type Error = KeyError;

    fn establish_session(
        &mut self,
        _local_identity: IdentityKeyHandle,
        _remote_identity_public: &[u8],
        _signed_pre_key: SignedPreKeyHandle,
        _one_time_pre_key: Option<OneTimePreKeyHandle>,
        _post_quantum_pre_key: Option<PostQuantumPreKeyHandle>,
    ) -> Result<SessionKeyHandle, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }

    fn next_message_key(
        &mut self,
        _session: SessionKeyHandle,
    ) -> Result<MessageKeyHandle, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }

    fn encrypt_message(
        &mut self,
        _message_key: MessageKeyHandle,
        _plaintext: &[u8],
        _associated_data: &[u8],
    ) -> Result<Vec<u8>, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }

    fn decrypt_message(
        &mut self,
        _message_key: MessageKeyHandle,
        _ciphertext: &[u8],
        _associated_data: &[u8],
    ) -> Result<Vec<u8>, Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }

    fn revoke_session(&mut self, _session: SessionKeyHandle) -> Result<(), Self::Error> {
        Err(KeyError::ProviderUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domains_remain_distinct() {
        let id = KeyId::new([7; 16]);
        let identity = IdentityKeyHandle::from_id(id);
        let message = MessageKeyHandle::from_id(id);
        assert_eq!(identity.id(), message.id());
        assert_ne!(identity.purpose(), message.purpose());
    }

    #[test]
    fn provider_approval_is_fail_closed() {
        let rejected = ProviderApproval::rejected();
        assert!(!rejected.is_production_approved());

        let approved = ProviderApproval {
            version: ProviderVersion::new(1, 2, 3),
            revision: ProviderRevision::new([0xAB; 20]),
            license_reviewed: true,
            support_reviewed: true,
            platform_reviewed: true,
            conformance_verified: true,
            independent_reviewed: true,
        };
        assert!(approved.is_production_approved());
        assert!(!approved.is_production_approved_with_capabilities(ProviderCapabilities {
            identity_keys_non_exportable: true,
            hardware_protection: true,
            pq_kem: false,
            message_ratchet: true,
        }));
        assert!(approved.is_production_approved_with_capabilities(ProviderCapabilities {
            identity_keys_non_exportable: true,
            hardware_protection: false,
            pq_kem: true,
            message_ratchet: true,
        }));
    }

    #[test]
    fn unavailable_provider_is_fail_closed() {
        let mut provider = UnavailableCryptoProvider;
        assert_eq!(
            provider.generate_identity_key(),
            Err(KeyError::ProviderUnavailable)
        );
        assert_eq!(
            provider.generate_recovery_key(),
            Err(KeyError::ProviderUnavailable)
        );
    }
}
