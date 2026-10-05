use eagle_core::{
    KeyId, KeyLifecycle, KeyMutation, KeyPurpose, LifecycleError, LifecycleState,
};

fn id(value: u8) -> KeyId {
    KeyId::new([value; 16])
}

#[test]
fn identity_key_cannot_be_repurposed_as_message_key() {
    let mut lifecycle = KeyLifecycle::default();
    lifecycle
        .register(id(1), KeyPurpose::DeviceIdentity, 1)
        .unwrap();

    assert_eq!(
        lifecycle.require_active(id(1), KeyPurpose::Message),
        Err(LifecycleError::PurposeMismatch)
    );
}

#[test]
fn rotation_is_forward_only() {
    let mut lifecycle = KeyLifecycle::default();
    lifecycle
        .register(id(1), KeyPurpose::Session, 10)
        .unwrap();

    let event = lifecycle.rotate(id(1), id(2), 11).unwrap();
    assert_eq!(event.mutation, KeyMutation::Register);
    assert_eq!(event.purpose, KeyPurpose::Session);
    assert_eq!(lifecycle.get(id(1)).unwrap().state(), LifecycleState::Revoked);
    assert_eq!(lifecycle.get(id(2)).unwrap().generation(), 11);
}

#[test]
fn revoked_and_destroyed_keys_are_not_usable() {
    let mut lifecycle = KeyLifecycle::default();
    lifecycle
        .register(id(3), KeyPurpose::StorageWrapping, 1)
        .unwrap();

    lifecycle.revoke(id(3)).unwrap();
    assert_eq!(
        lifecycle.require_active(id(3), KeyPurpose::StorageWrapping),
        Err(LifecycleError::Revoked)
    );

    lifecycle.destroy(id(3)).unwrap();
    assert_eq!(
        lifecycle.require_active(id(3), KeyPurpose::StorageWrapping),
        Err(LifecycleError::Destroyed)
    );
}
