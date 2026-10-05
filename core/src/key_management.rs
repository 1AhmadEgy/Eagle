use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyPurpose {
    IdentitySigning,
    IdentityAgreement,
    PreKey,
    Session,
    Storage,
    RecoveryWrapping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCustody {
    PlatformSecure,
    HardwareBackedRequired,
    Ephemeral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyLifecycle {
    Active,
    Suspended,
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    InvalidReference,
    PurposeMismatch,
    ScopeMismatch,
    CustodyMismatch,
    Revoked,
    Suspended,
    ExportForbidden,
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidReference => "invalid key reference",
            Self::PurposeMismatch => "key purpose mismatch",
            Self::ScopeMismatch => "key scope mismatch",
            Self::CustodyMismatch => "key custody requirement mismatch",
            Self::Revoked => "key is revoked",
            Self::Suspended => "key is suspended",
            Self::ExportForbidden => "key export is forbidden",
        })
    }
}

impl std::error::Error for KeyError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyReference([u8; 16]);

impl KeyReference {
    pub fn new(bytes: [u8; 16]) -> Result<Self, KeyError> {
        if bytes == [0; 16] {
            return Err(KeyError::InvalidReference);
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyScope {
    account_id: u64,
    device_id: u64,
    trust_epoch: u64,
}

impl KeyScope {
    pub fn new(account_id: u64, device_id: u64, trust_epoch: u64) -> Result<Self, KeyError> {
        if account_id == 0 || device_id == 0 {
            return Err(KeyError::InvalidReference);
        }
        Ok(Self {
            account_id,
            device_id,
            trust_epoch,
        })
    }

    pub fn account_id(&self) -> u64 {
        self.account_id
    }

    pub fn device_id(&self) -> u64 {
        self.device_id
    }

    pub fn trust_epoch(&self) -> u64 {
        self.trust_epoch
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyPolicy {
    pub purpose: KeyPurpose,
    pub custody: KeyCustody,
}

impl KeyPolicy {
    pub const fn for_purpose(purpose: KeyPurpose) -> Self {
        Self {
            purpose,
            custody: match purpose {
                KeyPurpose::IdentitySigning
                | KeyPurpose::IdentityAgreement
                | KeyPurpose::PreKey
                | KeyPurpose::Storage
                | KeyPurpose::RecoveryWrapping => KeyCustody::PlatformSecure,
                KeyPurpose::Session => KeyCustody::Ephemeral,
            },
        }
    }

    pub const fn require_hardware_backing(self) -> Self {
        Self {
            custody: KeyCustody::HardwareBackedRequired,
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyRecord {
    reference: KeyReference,
    scope: KeyScope,
    policy: KeyPolicy,
    lifecycle: KeyLifecycle,
}

impl KeyRecord {
    pub fn new(reference: KeyReference, scope: KeyScope, policy: KeyPolicy) -> Self {
        Self {
            reference,
            scope,
            policy,
            lifecycle: KeyLifecycle::Active,
        }
    }

    pub fn reference(&self) -> KeyReference {
        self.reference
    }

    pub fn scope(&self) -> KeyScope {
        self.scope
    }

    pub fn policy(&self) -> KeyPolicy {
        self.policy
    }

    pub fn lifecycle(&self) -> KeyLifecycle {
        self.lifecycle
    }

    pub fn suspend(&mut self) -> Result<(), KeyError> {
        if self.lifecycle == KeyLifecycle::Revoked {
            return Err(KeyError::Revoked);
        }
        self.lifecycle = KeyLifecycle::Suspended;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), KeyError> {
        if self.lifecycle == KeyLifecycle::Revoked {
            return Err(KeyError::Revoked);
        }
        self.lifecycle = KeyLifecycle::Active;
        Ok(())
    }

    pub fn revoke(&mut self) {
        self.lifecycle = KeyLifecycle::Revoked;
    }

    pub fn authorize(
        &self,
        scope: KeyScope,
        purpose: KeyPurpose,
        custody: KeyCustody,
    ) -> Result<(), KeyError> {
        match self.lifecycle {
            KeyLifecycle::Revoked => return Err(KeyError::Revoked),
            KeyLifecycle::Suspended => return Err(KeyError::Suspended),
            KeyLifecycle::Active => {}
        }

        if self.scope != scope {
            return Err(KeyError::ScopeMismatch);
        }

        if self.policy.purpose != purpose {
            return Err(KeyError::PurposeMismatch);
        }

        if self.policy.custody != custody
            && !(self.policy.custody == KeyCustody::PlatformSecure
                && custody == KeyCustody::HardwareBackedRequired)
        {
            return Err(KeyError::CustodyMismatch);
        }

        Ok(())
    }

    pub fn export(&self) -> Result<(), KeyError> {
        Err(KeyError::ExportForbidden)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference() -> KeyReference {
        KeyReference::new([7; 16]).unwrap()
    }

    #[test]
    fn zero_reference_is_rejected() {
        assert_eq!(
            KeyReference::new([0; 16]),
            Err(KeyError::InvalidReference)
        );
    }

    #[test]
    fn long_lived_keys_are_secure_and_non_exportable() {
        let scope = KeyScope::new(10, 20, 1).unwrap();
        for purpose in [
            KeyPurpose::IdentitySigning,
            KeyPurpose::IdentityAgreement,
            KeyPurpose::PreKey,
            KeyPurpose::Storage,
            KeyPurpose::RecoveryWrapping,
        ] {
            let policy = KeyPolicy::for_purpose(purpose);
            assert_eq!(policy.custody, KeyCustody::PlatformSecure);

            let record = KeyRecord::new(reference(), scope, policy);
            assert_eq!(
                record.authorize(scope, purpose, KeyCustody::PlatformSecure),
                Ok(())
            );
            assert_eq!(record.export(), Err(KeyError::ExportForbidden));
        }
    }

    #[test]
    fn hardware_backing_can_strengthen_platform_secure_policy() {
        let policy = KeyPolicy::for_purpose(KeyPurpose::IdentityAgreement).require_hardware_backing();
        let scope = KeyScope::new(10, 20, 1).unwrap();
        let record = KeyRecord::new(reference(), scope, policy);

        assert_eq!(record.policy().custody, KeyCustody::HardwareBackedRequired);
        assert_eq!(
            record.authorize(
                scope,
                KeyPurpose::IdentityAgreement,
                KeyCustody::PlatformSecure
            ),
            Err(KeyError::CustodyMismatch)
        );
        assert_eq!(
            record.authorize(
                scope,
                KeyPurpose::IdentityAgreement,
                KeyCustody::HardwareBackedRequired
            ),
            Ok(())
        );
        assert_eq!(record.export(), Err(KeyError::ExportForbidden));
    }

    #[test]
    fn revoked_keys_are_terminal() {
        let scope = KeyScope::new(10, 20, 1).unwrap();
        let mut record =
            KeyRecord::new(reference(), scope, KeyPolicy::for_purpose(KeyPurpose::Storage));

        record.revoke();
        assert_eq!(record.lifecycle(), KeyLifecycle::Revoked);
        assert_eq!(
            record.authorize(scope, KeyPurpose::Storage, KeyCustody::PlatformSecure),
            Err(KeyError::Revoked)
        );
        assert_eq!(record.resume(), Err(KeyError::Revoked));
        assert_eq!(record.suspend(), Err(KeyError::Revoked));
    }

    #[test]
    fn suspended_keys_must_resume_before_use() {
        let scope = KeyScope::new(10, 20, 1).unwrap();
        let mut record =
            KeyRecord::new(reference(), scope, KeyPolicy::for_purpose(KeyPurpose::Session));

        record.suspend().unwrap();
        assert_eq!(
            record.authorize(scope, KeyPurpose::Session, KeyCustody::Ephemeral),
            Err(KeyError::Suspended)
        );

        record.resume().unwrap();
        assert_eq!(
            record.authorize(scope, KeyPurpose::Session, KeyCustody::Ephemeral),
            Ok(())
        );
    }

    #[test]
    fn session_keys_are_ephemeral_by_policy() {
        let policy = KeyPolicy::for_purpose(KeyPurpose::Session);
        assert_eq!(policy.custody, KeyCustody::Ephemeral);
    }

    #[test]
    fn key_scope_must_match_exactly() {
        let scope = KeyScope::new(10, 20, 1).unwrap();
        let other_device = KeyScope::new(10, 21, 1).unwrap();
        let record = KeyRecord::new(
            reference(),
            scope,
            KeyPolicy::for_purpose(KeyPurpose::IdentitySigning),
        );

        assert_eq!(
            record.authorize(
                other_device,
                KeyPurpose::IdentitySigning,
                KeyCustody::PlatformSecure
            ),
            Err(KeyError::ScopeMismatch)
        );
        assert_eq!(
            record.authorize(
                scope,
                KeyPurpose::IdentitySigning,
                KeyCustody::PlatformSecure
            ),
            Ok(())
        );
    }

    #[test]
    fn invalid_scope_is_rejected() {
        assert_eq!(
            KeyScope::new(0, 20, 1),
            Err(KeyError::InvalidReference)
        );
        assert_eq!(
            KeyScope::new(10, 0, 1),
            Err(KeyError::InvalidReference)
        );
    }
}
