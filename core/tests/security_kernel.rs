use eagle_core::{
    authorize, validate_version, Capability, DeliveryGuardError, DeliveryMetadata, Device,
    DeviceTrustState, EncryptedEnvelope, EncryptedRecord, InMemoryOpaqueTransport,
    InMemorySecureStorage, KeyError, KeyId, KeyPurpose, KeyStore, LifecycleError, KeyLifecycle,
    MeshTransport, MessageId, OpaqueFrame, OpaqueId, OpaquePayload, Platform, ReplayError,
    ReplayWindow, SecurityContext, SecurityError, SecureStorage, Session, SessionState,
    StorageError, StorageState, TransportError, UnavailableCryptoProvider,
    CURRENT_PROTOCOL_VERSION, MAX_ID_BYTES,
};

fn authenticated_context() -> SecurityContext {
    let mut ctx = SecurityContext::new(1, CURRENT_PROTOCOL_VERSION).unwrap();
    ctx.begin_authentication().unwrap();
    ctx
}

fn id(value: u8) -> OpaqueId {
    OpaqueId::new(vec![value; 8]).unwrap()
}

#[test]
fn untrusted_kernel_is_not_usable() {
    let ctx = SecurityContext::new(1, 1).unwrap();
    assert_eq!(authorize(&ctx, Capability::Read), Err(SecurityError::Unauthorized));
    assert_eq!(authorize(&ctx, Capability::Write), Err(SecurityError::Unauthorized));
    assert_eq!(
        authorize(&ctx, Capability::Administrative),
        Err(SecurityError::Unauthorized)
    );
}

#[test]
fn device_is_not_authorized_before_trust() {
    let device = Device::new(10, 20, Platform::Android);
    assert_eq!(device.trust_state(), DeviceTrustState::Unknown);
    assert!(device.can_authorize().is_err());
}

#[test]
fn session_cannot_self_elevate_unverified_context() {
    let mut ctx = SecurityContext::new(1, 1).unwrap();
    assert_eq!(
        Session::establish(&mut ctx, 1),
        Err(SecurityError::InvalidSessionTransition)
    );
    assert_eq!(ctx.session_state(), SessionState::Idle);
}

#[test]
fn pending_context_cannot_establish_or_rekey() {
    let mut ctx = authenticated_context();
    assert_eq!(ctx.trust_state(), eagle_core::TrustState::Pending);
    assert!(ctx.establish().is_err());
    assert_eq!(
        Session::begin_rekey(&mut ctx),
        Err(SecurityError::InvalidSessionTransition)
    );
}

#[test]
fn invalid_configuration_fails_closed() {
    assert_eq!(
        SecurityContext::new(0, 1),
        Err(SecurityError::UnsupportedProtocol)
    );
    assert_eq!(
        SecurityContext::new(2, 1),
        Err(SecurityError::UnsupportedProtocol)
    );
    assert_eq!(
        SecurityContext::new(1, 2),
        Err(SecurityError::UnsupportedProtocol)
    );
}

#[test]
fn envelope_and_version_validation_are_bounded() {
    assert_eq!(validate_version(1, 1), Ok(1));
    assert_eq!(validate_version(0, 1), Err(eagle_core::ProtocolError::DowngradeRejected));
    assert_eq!(validate_version(2, 1), Err(eagle_core::ProtocolError::UnsupportedVersion));

    assert_eq!(
        OpaqueId::new(Vec::new()),
        Err(eagle_core::ProtocolError::EmptyIdentifier)
    );
    assert_eq!(
        OpaqueId::new(vec![0; MAX_ID_BYTES + 1]),
        Err(eagle_core::ProtocolError::IdentifierTooLarge)
    );

    let envelope = EncryptedEnvelope::new(
        MessageId::new([1; 16]),
        id(2),
        id(3),
        None,
        vec![0xAA; 32],
        CURRENT_PROTOCOL_VERSION,
        42,
    )
    .unwrap();
    let header = envelope.frame_header().unwrap();
    assert_eq!(header.payload_len(), 32);
    assert_eq!(header.validate_payload_len(32), Ok(()));
    assert_eq!(
        header.validate_payload_len(31),
        Err(eagle_core::ProtocolError::PayloadLengthMismatch)
    );
}

#[test]
fn replay_and_freshness_fail_closed() {
    let mut window = ReplayWindow::new(4).unwrap();
    assert_eq!(window.observe(1, 10, MessageId::new([10; 16])), Ok(()));
    assert_eq!(
        window.observe(1, 10, MessageId::new([10; 16])),
        Err(ReplayError::Duplicate)
    );
    assert_eq!(
        window.observe(1, 10, MessageId::new([11; 16])),
        Err(ReplayError::SequenceCollision)
    );
    assert_eq!(window.advance_epoch(2), Ok(()));
    assert_eq!(window.epoch(), Some(2));
    assert_eq!(
        window.advance_epoch(1),
        Err(ReplayError::EpochRollback)
    );
}

#[test]
fn inbound_delivery_guard_is_freshness_then_replay() {
    let mut guard =
        eagle_core::InboundReplayGuard::new(4, eagle_core::FreshnessPolicy::new(1_000, 100))
            .unwrap();

    let metadata = DeliveryMetadata::new(1, 1, 10_000, Some(10_100));
    assert_eq!(
        guard.accept(metadata, MessageId::new([1; 16]), 10_050),
        Ok(())
    );
    assert_eq!(
        guard.accept(metadata, MessageId::new([1; 16]), 10_050),
        Err(DeliveryGuardError::Replay(ReplayError::Duplicate))
    );
}

#[test]
fn crypto_provider_never_falls_back_to_fake_keys() {
    let mut provider = UnavailableCryptoProvider;
    assert_eq!(
        provider.generate_identity_key(),
        Err(KeyError::ProviderUnavailable)
    );
    let id = eagle_core::IdentityKeyHandle::from_id(KeyId::new([2; 16]));
    assert_eq!(id.purpose(), KeyPurpose::DeviceIdentity);
}

#[test]
fn key_lifecycle_is_monotonic_and_single_use() {
    let mut lifecycle = KeyLifecycle::default();
    let key = KeyId::new([3; 16]);
    lifecycle
        .register(key, KeyPurpose::OneTimePreKey, 1)
        .unwrap();

    let consumed = lifecycle.consume_one_time_pre_key(key).unwrap();
    assert_eq!(consumed.mutation, eagle_core::KeyMutation::Consume);
    assert_eq!(
        lifecycle.consume_one_time_pre_key(key),
        Err(LifecycleError::Consumed)
    );
}

#[test]
fn storage_is_opaque_and_fail_closed() {
    let record = EncryptedRecord::new(id(1), id(2), 1, vec![0xAA; 16], 7).unwrap();
    let mut store = InMemorySecureStorage::new();
    assert_eq!(store.state(), StorageState::Healthy);
    store.put(record).unwrap();

    assert!(store.get(&id(1), &id(2)).is_ok());
    assert_eq!(store.get(&id(1), &id(9)), Err(StorageError::NotFound));

    let receipt = store.delete(&id(1), &id(2)).unwrap();
    assert_eq!(receipt.record_id(), &id(1));
    assert_eq!(receipt.schema_version(), 1);
    assert_eq!(store.get(&id(1), &id(2)), Err(StorageError::NotFound));

    store.set_recovery_required();
    assert_eq!(store.put(EncryptedRecord::new(id(4), id(2), 1, vec![1], 8).unwrap()),
        Err(StorageError::RecoveryRequired));
}

#[test]
fn opaque_transport_never_accepts_empty_payload() {
    assert_eq!(
        OpaquePayload::new(Vec::new()),
        Err(TransportError::InvalidPayload)
    );

    let payload = OpaquePayload::new(vec![0xAA; 8]).unwrap();
    let frame = OpaqueFrame::new(1, id(1), id(2), payload, 2).unwrap();

    let mut transport = InMemoryOpaqueTransport::default();
    transport.send(frame.clone()).unwrap();
    assert_eq!(transport.receive(), Some(frame));
}

#[test]
fn mesh_frame_hop_limit_is_fail_closed() {
    let mut frame = OpaqueFrame::new(
        1,
        id(1),
        id(2),
        OpaquePayload::new(vec![0xAA; 8]).unwrap(),
        1,
    )
    .unwrap();

    assert_eq!(
        frame.decrement_hop_limit(),
        Err(TransportError::HopLimitExceeded)
    );
}

#[test]
fn storage_state_cannot_recover_from_unavailable_without_explicit_backend_recovery() {
    let mut store = InMemorySecureStorage::new();
    store.set_unavailable();
    assert_eq!(store.state(), StorageState::Unavailable);
    assert_eq!(store.recover(), Err(StorageError::Unavailable));
}
