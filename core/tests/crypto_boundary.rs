use eagle_core::{
    IdentityKeyHandle, KeyError, KeyId, KeyPurpose, KeyStore, MessageKeyHandle, MessagingCrypto,
    RecoveryKeyHandle, UnavailableCryptoProvider,
};

#[test]
fn key_domains_are_explicit_and_distinct() {
    let id = KeyId::new([1; 16]);
    let identity = IdentityKeyHandle::from_id(id);
    let message = MessageKeyHandle::from_id(id);
    let recovery = RecoveryKeyHandle::from_id(id);

    assert_eq!(identity.purpose(), KeyPurpose::DeviceIdentity);
    assert_eq!(message.purpose(), KeyPurpose::Message);
    assert_eq!(recovery.purpose(), KeyPurpose::Recovery);
    assert_eq!(identity.id(), message.id());
}

#[test]
fn default_provider_never_falls_back_to_plaintext_or_fake_keys() {
    let mut provider = UnavailableCryptoProvider;

    assert_eq!(
        provider.generate_identity_key(),
        Err(KeyError::ProviderUnavailable)
    );
    assert_eq!(
        provider.generate_storage_wrapping_key(),
        Err(KeyError::ProviderUnavailable)
    );
    assert_eq!(
        provider.generate_recovery_key(),
        Err(KeyError::ProviderUnavailable)
    );
}

#[test]
fn messaging_operations_fail_closed_without_approved_provider() {
    let mut provider = UnavailableCryptoProvider;
    let id = IdentityKeyHandle::from_id(KeyId::new([2; 16]));
    let signed = eagle_core::SignedPreKeyHandle::from_id(KeyId::new([3; 16]));

    assert_eq!(
        provider.establish_session(id, &[9; 32], signed, None, None),
        Err(KeyError::ProviderUnavailable)
    );
}
