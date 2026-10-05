use eagle_core::{
    authorize, validate_version, Capability, Device, DeviceTrustState, EncryptedEnvelope,
    MessageId, OpaqueId, Platform, SecurityContext, SecurityError, Session, SessionState,
    CURRENT_PROTOCOL_VERSION, MAX_ID_BYTES,
    FreshnessError, FreshnessPolicy, ReplayError, ReplayWindow,
};

fn trusted_context() -> SecurityContext {
    let mut ctx = SecurityContext::new(1, CURRENT_PROTOCOL_VERSION).unwrap();
    ctx.begin_authentication().unwrap();
    ctx
}

#[test]
fn untrusted_kernel_is_not_usable() {
    let ctx = SecurityContext::new(1, 1).unwrap();
    assert_eq!(
        authorize(&ctx, Capability::Read),
        Err(SecurityError::Unauthorized)
    );
    assert_eq!(
        authorize(&ctx, Capability::Write),
        Err(SecurityError::Unauthorized)
    );
    assert_eq!(
        authorize(&ctx, Capability::Administrative),
        Err(SecurityError::Unauthorized)
    );
}

#[test]
fn device_is_not_authorized_before_trust() {
    let device = Device::new(10, 20, Platform::Android);
    assert_eq!(device.account(), 10);
    assert_eq!(device.device(), 20);
    assert_eq!(device.platform(), Platform::Android);
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
    let mut ctx = trusted_context();
    assert_eq!(ctx.trust_state(), eagle_core::TrustState::Pending);

    // The actual verifier remains an internal test seam until the approved
    // authentication/cryptographic protocol is integrated.
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
fn version_validation_is_monotonic_and_bounded() {
    assert_eq!(validate_version(1, 1), Ok(1));
    assert_eq!(
        validate_version(0, 1),
        Err(eagle_core::ProtocolError::DowngradeRejected)
    );
    assert_eq!(
        validate_version(2, 1),
        Err(eagle_core::ProtocolError::UnsupportedVersion)
    );
}

#[test]
fn identifiers_are_constructor_bounded() {
    assert_eq!(
        OpaqueId::new(Vec::new()),
        Err(eagle_core::ProtocolError::EmptyIdentifier)
    );
    assert_eq!(
        OpaqueId::new(vec![0; MAX_ID_BYTES + 1]),
        Err(eagle_core::ProtocolError::IdentifierTooLarge)
    );
    assert_eq!(OpaqueId::new(vec![7; 8]).unwrap().as_bytes(), &[7; 8]);
    assert_eq!(MessageId::new([9; 16]).as_bytes(), &[9; 16]);
}

#[test]
fn envelope_is_structurally_validated() {
    let envelope = EncryptedEnvelope::new(
        MessageId::new([1; 16]),
        OpaqueId::new(vec![2; 8]).unwrap(),
        OpaqueId::new(vec![3; 8]).unwrap(),
        None,
        vec![0xAA; 32],
        CURRENT_PROTOCOL_VERSION,
        42,
    )
    .unwrap();

    let header = envelope.frame_header().unwrap();
    assert_eq!(header.protocol_version(), CURRENT_PROTOCOL_VERSION);
    assert_eq!(header.payload_len(), 32);
    assert_eq!(header.validate_payload_len(32), Ok(()));
}

#[test]
fn frame_header_rejects_length_and_version_mismatch() {
    let envelope = EncryptedEnvelope::new(
        MessageId::new([4; 16]),
        OpaqueId::new(vec![5; 8]).unwrap(),
        OpaqueId::new(vec![6; 8]).unwrap(),
        None,
        vec![0xBB; 8],
        CURRENT_PROTOCOL_VERSION,
        42,
    )
    .unwrap();
    let header = envelope.frame_header().unwrap();
    assert_eq!(
        header.validate_payload_len(7),
        Err(eagle_core::ProtocolError::PayloadLengthMismatch)
    );
    assert_eq!(header.validate_payload_len(8), Ok(()));
}

#[test]
fn device_revocation_and_replacement_are_terminal() {
    let mut revoked = Device::new(10, 20, Platform::Desktop);
    let mut replaced = Device::new(10, 21, Platform::Ios);

    // Pairing/approval are internal until the real authentication verifier exists.
    // The public API exposes only terminal fail-closed transitions.
    assert_eq!(
        revoked.revoke(),
        Err(eagle_core::DeviceError::InvalidTransition)
    );
    assert_eq!(
        replaced.replace(),
        Err(eagle_core::DeviceError::InvalidTransition)
    );
}

#[test]
fn replay_window_enforces_duplicate_epoch_and_window_bounds() {
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
    assert_eq!(window.observe(1, 8, MessageId::new([8; 16])), Ok(()));
    assert_eq!(
        window.observe(2, 1, MessageId::new([1; 16])),
        Err(ReplayError::EpochChanged)
    );
    assert_eq!(window.advance_epoch(2), Ok(()));
    assert_eq!(window.observe(2, 1, MessageId::new([1; 16])), Ok(()));
    assert_eq!(window.advance_epoch(1), Err(ReplayError::EpochRollback));
    assert_eq!(window.observe(2, 0, MessageId::new([0; 16])), Ok(()));
    assert_eq!(
        window.observe(2, 0, MessageId::new([0; 16])),
        Err(ReplayError::Duplicate)
    );
}

#[test]
fn freshness_policy_rejects_expired_and_future_messages() {
    let policy = FreshnessPolicy::new(1_000, 100);

    assert_eq!(policy.validate(10_000, 10_050, Some(10_100)), Ok(()));
    assert_eq!(
        policy.validate(10_000, 10_101, None),
        Err(FreshnessError::CreatedInFuture)
    );
    assert_eq!(
        policy.validate(10_000, 10_100, None),
        Ok(())
    );
    assert_eq!(
        policy.validate(10_101, 10_202, None),
        Err(FreshnessError::CreatedInFuture)
    );
    assert_eq!(
        policy.validate(10_200, 10_050, Some(10_150)),
        Err(FreshnessError::Expired)
    );
    assert_eq!(
        policy.validate(10_000, 8_999, None),
        Err(FreshnessError::TooOld)
    );
}

#[test]
fn key_custody_contract_is_fail_closed() {
    use eagle_core::{
        KeyCustody, KeyError, KeyPolicy, KeyPurpose, KeyRecord, KeyReference, KeyScope,
    };

    let reference = KeyReference::new([8; 16]).unwrap();
    let scope = KeyScope::new(10, 20, 1).unwrap();
    let policy = KeyPolicy::for_purpose(KeyPurpose::IdentitySigning);
    let mut record = KeyRecord::new(reference, scope, policy);

    assert_eq!(
        record.authorize(
            scope,
            KeyPurpose::IdentitySigning,
            KeyCustody::PlatformSecure
        ),
        Ok(())
    );
    assert_eq!(record.export(), Err(KeyError::ExportForbidden));

    record.revoke();
    assert_eq!(
        record.authorize(
            scope,
            KeyPurpose::IdentitySigning,
            KeyCustody::PlatformSecure
        ),
        Err(KeyError::Revoked)
    );
}
