use eagle_core::{
    authorize, Capability, Identity, PrincipalId, SecurityContext, Session, SessionState,
    TrustState,
};

#[test]
fn identity_and_policy_flow_is_deterministic() {
    let mut ctx = SecurityContext::new(1);
    let identity = Identity::authenticate(&mut ctx, PrincipalId(7)).unwrap();

    assert_eq!(identity.principal, PrincipalId(7));
    assert_eq!(ctx.trust_state(), TrustState::Trusted);
    assert_eq!(ctx.session_state(), SessionState::Authenticated);
    authorize(&ctx, Capability::Read).unwrap();
}

#[test]
fn administrative_capability_is_denied() {
    let mut ctx = SecurityContext::new(1);
    Identity::authenticate(&mut ctx, PrincipalId(7)).unwrap();

    assert_eq!(
        authorize(&ctx, Capability::Administrative),
        Err(eagle_core::SecurityError::Unauthorized)
    );
}

#[test]
fn session_rejects_protocol_downgrade_without_mutation() {
    let mut ctx = SecurityContext::new(2);
    let result = Session::establish(&mut ctx, 1);

    assert_eq!(result, Err(eagle_core::SecurityError::ProtocolDowngrade));
    assert_eq!(ctx.negotiated_protocol(), 2);
}

#[test]
fn session_rejects_renegotiation_downgrade_without_mutation() {
    let mut ctx = SecurityContext::new(1);
    assert_eq!(Session::establish(&mut ctx, 3).unwrap().protocol, 3);

    let result = Session::establish(&mut ctx, 2);
    assert_eq!(result, Err(eagle_core::SecurityError::ProtocolDowngrade));
    assert_eq!(ctx.negotiated_protocol(), 3);
}
