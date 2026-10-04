use eagle_core::{authorize, Capability, Identity, PrincipalId, SecurityContext, Session};

#[test]
fn identity_and_policy_flow_is_deterministic() {
    let mut ctx = SecurityContext::new(1);
    let identity = Identity::authenticate(&mut ctx, PrincipalId(7)).unwrap();
    assert_eq!(identity.principal, PrincipalId(7));

    Session::establish(&mut ctx, 1).unwrap();
    authorize(&ctx, Capability::Read).unwrap();
}

#[test]
fn session_rejects_protocol_downgrade_without_mutation() {
    let mut ctx = SecurityContext::new(2);
    Identity::authenticate(&mut ctx, PrincipalId(7)).unwrap();

    let result = Session::establish(&mut ctx, 1);
    assert!(result.is_err());
    assert_eq!(ctx.negotiated_protocol, 2);
    assert_eq!(ctx.session, eagle_core::SessionState::Authenticated);
}

#[test]
fn session_rejects_renegotiation_downgrade_without_mutation() {
    let mut ctx = SecurityContext::new(1);
    Identity::authenticate(&mut ctx, PrincipalId(7)).unwrap();
    assert_eq!(Session::establish(&mut ctx, 3).unwrap().protocol, 3);

    // The second establishment attempt is illegal after the session is
    // established, and must not mutate the negotiated protocol.
    let result = Session::establish(&mut ctx, 2);
    assert!(result.is_err());
    assert_eq!(ctx.negotiated_protocol, 3);
    assert_eq!(ctx.session, eagle_core::SessionState::Established);
}
