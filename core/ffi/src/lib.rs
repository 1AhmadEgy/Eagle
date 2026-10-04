#![forbid(unsafe_code)]

// UniFFI proc-macro mode keeps the bridge contract declared next to the Rust
// implementation while generated foreign bindings remain build artifacts.
uniffi::setup_scaffolding!();

use std::fmt;
use std::sync::Arc;

use eagle_core::DeviceTrustState;

/// Public identity material only. Private key material is intentionally absent.
#[derive(uniffi::Record, Clone, Debug)]
pub struct DeviceIdentity {
    pub device_id: String,
    pub public_key: Vec<u8>,
    pub trust_level: TrustLevel,
}

#[derive(uniffi::Enum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrustLevel {
    Untrusted,
    Pending,
    Trusted,
    Revoked,
    Replaced,
}

impl From<DeviceTrustState> for TrustLevel {
    fn from(value: DeviceTrustState) -> Self {
        match value {
            DeviceTrustState::Unknown => Self::Untrusted,
            DeviceTrustState::Pending => Self::Pending,
            DeviceTrustState::Trusted => Self::Trusted,
            DeviceTrustState::Revoked => Self::Revoked,
            DeviceTrustState::Replaced => Self::Replaced,
        }
    }
}

#[derive(uniffi::Error, Debug, PartialEq, Eq)]
pub enum EagleError {
    DeviceNotTrusted,
    HandshakeFailed,
    Crypto { message: String },
    InvalidArgument { message: String },
    ContractNotReady { operation: String },
}

impl fmt::Display for EagleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeviceNotTrusted => f.write_str("device not trusted"),
            Self::HandshakeFailed => f.write_str("handshake failed"),
            Self::Crypto { message } => write!(f, "crypto error: {message}"),
            Self::InvalidArgument { message } => write!(f, "invalid argument: {message}"),
            Self::ContractNotReady { operation } => {
                write!(f, "contract operation is not implemented yet: {operation}")
            }
        }
    }
}

impl std::error::Error for EagleError {}

/// Opaque session capability. Its internal representation is deliberately not
/// visible to foreign code and must never contain exported key material.
#[derive(uniffi::Object)]
pub struct SessionHandle {
    _private: (),
}

/// The concrete Rust object exported through UniFFI.
///
/// This crate currently exposes the stable FFI surface only. Operations which
/// require the not-yet-approved cryptographic, key-management, or serialization
/// decisions fail closed with ContractNotReady rather than returning fabricated
/// security state.
#[derive(uniffi::Object)]
pub struct EagleCore {}

#[uniffi::export]
impl EagleCore {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    pub async fn register_device(&self, alias: String) -> Result<DeviceIdentity, EagleError> {
        validate_non_empty("alias", &alias)?;
        Err(EagleError::ContractNotReady {
            operation: "register_device".to_owned(),
        })
    }

    pub async fn begin_session(&self, peer_id: String) -> Result<Arc<SessionHandle>, EagleError> {
        validate_non_empty("peer_id", &peer_id)?;
        Err(EagleError::ContractNotReady {
            operation: "begin_session".to_owned(),
        })
    }

    pub async fn end_session(&self, _handle: Arc<SessionHandle>) -> Result<(), EagleError> {
        Err(EagleError::ContractNotReady {
            operation: "end_session".to_owned(),
        })
    }

    pub async fn trust_status(&self, device_id: String) -> Result<TrustLevel, EagleError> {
        validate_non_empty("device_id", &device_id)?;
        Err(EagleError::ContractNotReady {
            operation: "trust_status".to_owned(),
        })
    }
}

fn validate_non_empty(field: &str, value: &str) -> Result<(), EagleError> {
    if value.trim().is_empty() {
        return Err(EagleError::InvalidArgument {
            message: format!("{field} must not be empty"),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_mapping_preserves_core_device_lifecycle() {
        assert_eq!(
            TrustLevel::from(DeviceTrustState::Unknown),
            TrustLevel::Untrusted
        );
        assert_eq!(
            TrustLevel::from(DeviceTrustState::Pending),
            TrustLevel::Pending
        );
        assert_eq!(
            TrustLevel::from(DeviceTrustState::Trusted),
            TrustLevel::Trusted
        );
        assert_eq!(
            TrustLevel::from(DeviceTrustState::Revoked),
            TrustLevel::Revoked
        );
        assert_eq!(
            TrustLevel::from(DeviceTrustState::Replaced),
            TrustLevel::Replaced
        );
    }

    #[test]
    fn empty_identifier_is_rejected_before_contract_not_ready() {
        assert_eq!(
            validate_non_empty("peer_id", "   "),
            Err(EagleError::InvalidArgument {
                message: "peer_id must not be empty".to_owned(),
            })
        );
    }
}
