#![forbid(unsafe_code)]

use std::fmt;

mod device;
mod policy;
mod protocol;
mod session;

pub use device::{Device, DeviceError, DeviceTrustState, Platform};
pub use policy::{authorize, Capability};
pub use protocol::{
    validate_version, EncryptedEnvelope, FrameHeader, MessageId, OpaqueId, ProtocolError,
    CURRENT_PROTOCOL_VERSION, FRAME_HEADER_BYTES, MAX_ID_BYTES, MAX_PAYLOAD_BYTES,

};
pub use session::Session;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustState {
    Untrusted,
    Pending,
    Trusted,
    Revoked,
    Replaced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Idle,
    Authenticating,
    Authenticated,
    Established,
    Rekeying,
    Closing,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityError {
    Unauthorized,
    InvalidTrustTransition,
    InvalidSessionTransition,
    ProtocolDowngrade,
    UnsupportedProtocol,
    ClosedSession,
    Device(DeviceError),
}

impl fmt::Display for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Unauthorized => "unauthorized",
            Self::InvalidTrustTransition => "invalid trust transition",
            Self::InvalidSessionTransition => "invalid session transition",
            Self::ProtocolDowngrade => "protocol downgrade rejected",
            Self::UnsupportedProtocol => "unsupported protocol",
            Self::ClosedSession => "closed session",
            Self::Device(error) => return write!(f, "device authority error: {error}"),
        })
    }
}

impl std::error::Error for SecurityError {}

impl From<DeviceError> for SecurityError {
    fn from(value: DeviceError) -> Self {
        Self::Device(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SecurityContext {
    trust: TrustState,
    session: SessionState,
    negotiated_protocol: u16,
    minimum_protocol: u16,
    maximum_protocol: u16,
    account: u64,
    device: u64,
    authority_epoch: u64,
}

impl SecurityContext {
    pub fn new(minimum_protocol: u16, maximum_protocol: u16) -> Result<Self, SecurityError> {
        if minimum_protocol == 0
            || maximum_protocol == 0
            || minimum_protocol > maximum_protocol
            || maximum_protocol > CURRENT_PROTOCOL_VERSION
        {
            return Err(SecurityError::UnsupportedProtocol);
        }

        Ok(Self {
            trust: TrustState::Untrusted,
            session: SessionState::Idle,
            negotiated_protocol: minimum_protocol,
            minimum_protocol,
            maximum_protocol,
            account: 0,
            device: 0,
            authority_epoch: 0,
        })
    }

    pub fn bind_device(&mut self, device: &Device) -> Result<(), SecurityError> {
        if self.session != SessionState::Idle || self.trust != TrustState::Untrusted {
            return Err(SecurityError::InvalidTrustTransition);
        }
        self.account = device.account();
        self.device = device.device();
        self.authority_epoch = device.authority_epoch();
        Ok(())
    }

    pub fn trust_state(&self) -> TrustState {
        self.trust
    }

    pub fn session_state(&self) -> SessionState {
        self.session
    }

    pub fn negotiated_protocol(&self) -> u16 {
        self.negotiated_protocol
    }

    pub fn minimum_protocol(&self) -> u16 {
        self.minimum_protocol
    }

    pub fn maximum_protocol(&self) -> u16 {
        self.maximum_protocol
    }

    pub fn begin_authentication(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Untrusted || self.session != SessionState::Idle {
            return Err(SecurityError::InvalidTrustTransition);
        }

        self.trust = TrustState::Pending;
        self.session = SessionState::Authenticating;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn accept_verified_authentication(
        &mut self,
        device: &Device,
    ) -> Result<(), SecurityError> {
        if self.trust != TrustState::Pending || self.session != SessionState::Authenticating {
            return Err(SecurityError::InvalidSessionTransition);
        }

        device.validate_authority(self.account, self.device, self.authority_epoch)?;
        self.trust = TrustState::Trusted;
        self.session = SessionState::Authenticated;
        Ok(())
    }

    pub fn establish(&mut self, device: &Device) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Authenticated {
            return Err(SecurityError::InvalidSessionTransition);
        }
        device.validate_authority(self.account, self.device, self.authority_epoch)?;
        self.session = SessionState::Established;
        Ok(())
    }

    /// Aborts an in-progress authentication attempt and returns to the initial
    /// untrusted state. No trust is granted by cancellation.
    pub fn abort_authentication(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Pending || self.session != SessionState::Authenticating {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.trust = TrustState::Untrusted;
        self.session = SessionState::Idle;
        Ok(())
    }

    pub fn begin_rekey(&mut self, device: &Device) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Established {
            return Err(SecurityError::InvalidSessionTransition);
        }
        device.validate_authority(self.account, self.device, self.authority_epoch)?;
        self.session = SessionState::Rekeying;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn finish_rekey(&mut self, device: &Device) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Rekeying {
            return Err(SecurityError::InvalidSessionTransition);
        }
        device.validate_authority(self.account, self.device, self.authority_epoch)?;
        self.session = SessionState::Established;
        Ok(())
    }

    /// Aborts a rekey attempt by closing the session. A failed or incomplete
    /// rekey must not silently continue using the pre-rekey session state.
    pub fn abort_rekey(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Rekeying {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.session = SessionState::Closed;
        Ok(())
    }

    pub fn close_session(&mut self) {
        self.session = SessionState::Closed;
    }

    pub fn revoke_trust(&mut self) {
        self.trust = TrustState::Revoked;
        self.session = SessionState::Closed;
    }

    pub fn authorize(&self, device: &Device) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Established {
            return Err(SecurityError::Unauthorized);
        }
        device.validate_authority(self.account, self.device, self.authority_epoch)?;
        Ok(())
    }

    pub(crate) fn validate_and_negotiate(&mut self, offered: u16) -> Result<u16, SecurityError> {
        if self.session != SessionState::Authenticated || self.trust != TrustState::Trusted {
            return Err(SecurityError::InvalidSessionTransition);
        }

        if offered < self.minimum_protocol || offered < self.negotiated_protocol {
            return Err(SecurityError::ProtocolDowngrade);
        }

        if offered > self.maximum_protocol {
            return Err(SecurityError::UnsupportedProtocol);
        }

        self.negotiated_protocol = offered;
        Ok(offered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn authenticated() -> (SecurityContext, Device) {
        let mut device = Device::new(1, 2, Platform::Android);
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        let mut ctx = SecurityContext::new(1, CURRENT_PROTOCOL_VERSION).unwrap();
        ctx.bind_device(&device).unwrap();
        ctx.begin_authentication().unwrap();
        ctx.accept_verified_authentication(&device).unwrap();
        (ctx, device)
    }

    #[test]
    fn unknown_trust_cannot_authorize() {
        let ctx = SecurityContext::new(1, 1).unwrap();
        let device = Device::new(1, 2, Platform::Android);
        assert_eq!(ctx.authorize(&device), Err(SecurityError::Unauthorized));
    }

    #[test]
    fn authentication_is_one_way_and_guarded() {
        let mut ctx = SecurityContext::new(1, 1).unwrap();
        let mut device = Device::new(1, 2, Platform::Android);
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        ctx.bind_device(&device).unwrap();
        ctx.begin_authentication().unwrap();
        assert_eq!(ctx.trust_state(), TrustState::Pending);
        assert_eq!(
            ctx.begin_authentication(),
            Err(SecurityError::InvalidTrustTransition)
        );
        ctx.accept_verified_authentication(&device).unwrap();
        assert_eq!(ctx.trust_state(), TrustState::Trusted);
        assert_eq!(ctx.session_state(), SessionState::Authenticated);
        assert_eq!(
            ctx.accept_verified_authentication(&device),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn authentication_can_abort_without_granting_trust() {
        let mut ctx = SecurityContext::new(1, 1).unwrap();
        let device = Device::new(1, 2, Platform::Android);
        ctx.bind_device(&device).unwrap();
        ctx.begin_authentication().unwrap();
        assert_eq!(ctx.abort_authentication(), Ok(()));
        assert_eq!(ctx.trust_state(), TrustState::Untrusted);
        assert_eq!(ctx.session_state(), SessionState::Idle);
        assert_eq!(ctx.authorize(&device), Err(SecurityError::Unauthorized));
        assert_eq!(
            ctx.abort_authentication(),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn establishment_requires_authenticated_trusted_state() {
        let mut ctx = SecurityContext::new(1, 1).unwrap();
        let mut device = Device::new(1, 2, Platform::Android);
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        ctx.bind_device(&device).unwrap();
        assert_eq!(
            ctx.establish(&device),
            Err(SecurityError::InvalidSessionTransition)
        );
        ctx.begin_authentication().unwrap();
        assert_eq!(
            ctx.establish(&device),
            Err(SecurityError::InvalidSessionTransition)
        );
        ctx.accept_verified_authentication(&device).unwrap();
        ctx.establish(&device).unwrap();
        assert_eq!(ctx.session_state(), SessionState::Established);
        ctx.close_session();
        assert_eq!(ctx.session_state(), SessionState::Closed);
        assert_eq!(ctx.authorize(&device), Err(SecurityError::Unauthorized));
    }

    #[test]
    fn rekey_requires_established_session() {
        let (mut ctx, device) = authenticated();
        assert_eq!(
            ctx.begin_rekey(&device),
            Err(SecurityError::InvalidSessionTransition)
        );
        ctx.establish(&device).unwrap();
        ctx.begin_rekey(&device).unwrap();
        assert_eq!(ctx.session_state(), SessionState::Rekeying);
        assert_eq!(
            ctx.begin_rekey(&device),
            Err(SecurityError::InvalidSessionTransition)
        );
        ctx.finish_rekey(&device).unwrap();
        assert_eq!(ctx.session_state(), SessionState::Established);
        assert_eq!(
            ctx.finish_rekey(&device),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn incomplete_rekey_fails_closed() {
        let (mut ctx, device) = authenticated();
        ctx.establish(&device).unwrap();
        ctx.begin_rekey(&device).unwrap();
        assert_eq!(ctx.session_state(), SessionState::Rekeying);
        assert_eq!(ctx.abort_rekey(), Ok(()));
        assert_eq!(ctx.session_state(), SessionState::Closed);
        assert_eq!(ctx.authorize(&device), Err(SecurityError::Unauthorized));
        assert_eq!(
            ctx.abort_rekey(),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn revocation_is_terminal_for_context() {
        let (mut ctx, device) = authenticated();
        ctx.establish(&device).unwrap();
        ctx.revoke_trust();
        assert_eq!(ctx.trust_state(), TrustState::Revoked);
        assert_eq!(ctx.session_state(), SessionState::Closed);
        assert_eq!(ctx.authorize(&device), Err(SecurityError::Unauthorized));
        assert_eq!(
            ctx.begin_authentication(),
            Err(SecurityError::InvalidTrustTransition)
        );
        assert_eq!(
            ctx.establish(&device),
            Err(SecurityError::InvalidSessionTransition)
        );
        assert_eq!(
            ctx.begin_rekey(&device),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn revocation_invalidates_bound_context() {
        let (mut ctx, mut device) = authenticated();
        ctx.establish(&device).unwrap();
        assert_eq!(ctx.authorize(&device), Ok(()));
        device.revoke().unwrap();
        assert_eq!(
            ctx.authorize(&device),
            Err(SecurityError::Device(DeviceError::Revoked))
        );
    }

    #[test]
    fn replacement_invalidates_bound_context() {
        let (mut ctx, mut device) = authenticated();
        ctx.establish().unwrap();
        assert_eq!(ctx.authorize(&device), Ok(()));
        device.replace().unwrap();
        assert_eq!(
            ctx.authorize(&device),
            Err(SecurityError::Device(DeviceError::Replaced))
        );
    }

    #[test]
    fn wrong_device_cannot_authorize_bound_context() {
        let (mut ctx, device) = authenticated();
        ctx.establish().unwrap();
        let mut other = Device::new(1, 3, Platform::Android);
        other.begin_pairing().unwrap();
        other.approve().unwrap();
        assert_eq!(
            ctx.authorize(&other),
            Err(SecurityError::Device(DeviceError::IdentityMismatch))
        );
    }

    #[test]
    fn invalid_protocol_configuration_is_rejected() {
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
    fn rejected_protocol_does_not_mutate() {
        let (mut ctx, device) = authenticated();
        assert_eq!(
            ctx.validate_and_negotiate(0),
            Err(SecurityError::ProtocolDowngrade)
        );
        assert_eq!(ctx.negotiated_protocol(), 1);

        assert_eq!(
            ctx.validate_and_negotiate(2),
            Err(SecurityError::UnsupportedProtocol)
        );
        assert_eq!(ctx.negotiated_protocol(), 1);

        ctx.establish().unwrap();
        assert_eq!(
            ctx.validate_and_negotiate(1),
            Err(SecurityError::InvalidSessionTransition)
        );
        assert_eq!(ctx.negotiated_protocol(), 1);
    }
}
