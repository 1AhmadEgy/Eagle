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
    policy: KeyPolicy,
    lifecycle: KeyLifecycle,
}

impl KeyRecord {
    pub fn new(reference: KeyReference, policy: KeyPolicy) -> Self {
        Self {
            reference,
            policy,
            lifecycle: KeyLifecycle::Active,
        }
    }

    pub fn reference(&self) -> KeyReference {
        self.reference
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

    pub fn authorize(&self, purpose: KeyPurpose, custody: KeyCustody) -> Result<(), KeyError> {
        match self.lifecycle {
            KeyLifecycle::Revoked => return Err(KeyError::Revoked),
            KeyLifecycle::Suspended => return Err(KeyError::Suspended),
            KeyLifecycle::Active => {}
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
        for purpose in [
            KeyPurpose::IdentitySigning,
            KeyPurpose::IdentityAgreement,
            KeyPurpose::PreKey,
            KeyPurpose::Storage,
            KeyPurpose::RecoveryWrapping,
        ] {
            let policy = KeyPolicy::for_purpose(purpose);
            assert_eq!(policy.custody, KeyCustody::PlatformSecure);

            let record = KeyRecord::new(reference(), policy);
            assert_eq!(
                record.authorize(purpose, KeyCustody::PlatformSecure),
                Ok(())
            );
            assert_eq!(record.export(), Err(KeyError::ExportForbidden));
        }
    }

    #[test]
    fn hardware_backing_can_strengthen_platform_secure_policy() {
        let policy = KeyPolicy::for_purpose(KeyPurpose::IdentityAgreement).require_hardware_backing();
        let record = KeyRecord::new(reference(), policy);

        assert_eq!(record.policy().custody, KeyCustody::HardwareBackedRequired);
        assert_eq!(
            record.authorize(
                KeyPurpose::IdentityAgreement,
                KeyCustody::PlatformSecure
            ),
            Err(KeyError::CustodyMismatch)
        );
        assert_eq!(
            record.authorize(
                KeyPurpose::IdentityAgreement,
                KeyCustody::HardwareBackedRequired
            ),
            Ok(())
        );
        assert_eq!(record.export(), Err(KeyError::ExportForbidden));
    }

    #[test]
    fn revoked_keys_are_terminal() {
        let mut record =
            KeyRecord::new(reference(), KeyPolicy::for_purpose(KeyPurpose::Storage));

        record.revoke();
        assert_eq!(record.lifecycle(), KeyLifecycle::Revoked);
        assert_eq!(
            record.authorize(KeyPurpose::Storage, KeyCustody::PlatformSecure),
            Err(KeyError::Revoked)
        );
        assert_eq!(record.resume(), Err(KeyError::Revoked));
        assert_eq!(record.suspend(), Err(KeyError::Revoked));
    }

    #[test]
    fn suspended_keys_must_resume_before_use() {
        let mut record =
            KeyRecord::new(reference(), KeyPolicy::for_purpose(KeyPurpose::Session));

        record.suspend().unwrap();
        assert_eq!(
            record.authorize(KeyPurpose::Session, KeyCustody::Ephemeral),
            Err(KeyError::Suspended)
        );

        record.resume().unwrap();
        assert_eq!(
            record.authorize(KeyPurpose::Session, KeyCustody::Ephemeral),
            Ok(())
        );
    }

    #[test]
    fn session_keys_are_ephemeral_by_policy() {
        let policy = KeyPolicy::for_purpose(KeyPurpose::Session);
        assert_eq!(policy.custody, KeyCustody::Ephemeral);
    }
}
