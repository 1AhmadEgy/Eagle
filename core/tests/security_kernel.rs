use eagle_core::{
    authorize, Capability, Device, DeviceTrustState, Platform, SecurityContext, SecurityError,
    Session, SessionState,
};

#[test]
fn untrusted_kernel_is_not_usable() {
    let ctx = SecurityContext::new(1, 1).unwrap();
    assert_eq!(
        authorize(&ctx, Capability::Read),
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
fn invalid_configuration_fails_closed() {
    assert_eq!(
        SecurityContext::new(0, 1),
        Err(SecurityError::UnsupportedProtocol)
    );
    assert_eq!(
        SecurityContext::new(2, 1),
        Err(SecurityError::UnsupportedProtocol)
    );
}
