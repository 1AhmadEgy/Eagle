use eagle_core::{
    authorize, Capability, Identity, PrincipalId, SecurityContext, Session, SessionState,
    TrustState,
};

#[test]
fn identity_and_policy_flow_requires_explicit_session_establishment() {
    let mut ctx = SecurityContext::new(1);
    let identity = Identity::from_verified_principal(&mut ctx, PrincipalId(7)).unwrap();

    assert_eq!(identity.principal, PrincipalId(7));
    assert_eq!(ctx.trust_state(), TrustState::Trusted);
    assert_eq!(ctx.session_state(), SessionState::Idle);
    assert_eq!(
        authorize(&ctx, Capability::Read),
        Err(eagle_core::SecurityError::Unauthorized)
    );

    Session::establish(&mut ctx, 1).unwrap();
    authorize(&ctx, Capability::Read).unwrap();
}

#[test]
fn administrative_capability_is_denied_by_default() {
    let mut ctx = SecurityContext::new(1);
    Identity::from_verified_principal(&mut ctx, PrincipalId(7)).unwrap();
    Session::establish(&mut ctx, 1).unwrap();

    assert_eq!(
        authorize(&ctx, Capability::Administrative),
        Err(eagle_core::SecurityError::Unauthorized)
    );
}

#[test]
fn session_rejects_protocol_downgrade_without_mutation() {
    let mut ctx = SecurityContext::new(2);
    Identity::from_verified_principal(&mut ctx, PrincipalId(7)).unwrap();

    let result = Session::establish(&mut ctx, 1);

    assert_eq!(result, Err(eagle_core::SecurityError::ProtocolDowngrade));
    assert_eq!(ctx.negotiated_protocol(), 2);
    assert_eq!(ctx.session_state(), SessionState::Idle);
}

#[test]
fn protocol_cannot_be_renegotiated_after_session_establishment() {
    let mut ctx = SecurityContext::new(1);
    Identity::from_verified_principal(&mut ctx, PrincipalId(7)).unwrap();
    assert_eq!(Session::establish(&mut ctx, 3).unwrap().protocol, 3);

    let result = ctx.negotiate_protocol(4);
    assert_eq!(
        result,
        Err(eagle_core::SecurityError::InvalidSessionTransition)
    );
    assert_eq!(ctx.negotiated_protocol(), 3);
    assert_eq!(ctx.session_state(), SessionState::Authenticated);
}
