use eagle_core_ffi::{EagleCore, EagleError, TrustLevel};

#[test]
fn contract_exposes_expected_error_surface() {
    assert_eq!(
        EagleError::ContractNotReady {
            operation: "begin_session".to_owned(),
        }
        .to_string(),
        "contract operation is not implemented yet: begin_session"
    );
}

#[test]
fn contract_exposes_expected_trust_states() {
    assert_eq!(TrustLevel::Untrusted as u8, 0);
    assert_eq!(TrustLevel::Pending as u8, 1);
    assert_eq!(TrustLevel::Trusted as u8, 2);
    assert_eq!(TrustLevel::Revoked as u8, 3);
    assert_eq!(TrustLevel::Replaced as u8, 4);
}

#[test]
fn contract_constructor_is_available() {
    let _ = EagleCore::new();
}
