#![forbid(unsafe_code)]

mod device;
mod policy;
mod protocol;
mod session;

pub use device::{Device, DeviceError, DeviceTrustState, Platform};
pub use policy::{authorize, Capability};
pub use protocol::{
    validate_version, EncryptedEnvelope, FrameHeader, MessageId, OpaqueId, ProtocolError,
    CURRENT_PROTOCOL_VERSION, MAX_ID_BYTES, MAX_PAYLOAD_BYTES,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityContext {
    trust: TrustState,
    session: SessionState,
    negotiated_protocol: u16,
    minimum_protocol: u16,
    maximum_protocol: u16,
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
        })
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

    pub fn begin_authentication(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Untrusted || self.session != SessionState::Idle {
            return Err(SecurityError::InvalidTrustTransition);
        }

        self.trust = TrustState::Pending;
        self.session = SessionState::Authenticating;
        Ok(())
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn accept_verified_authentication(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Pending || self.session != SessionState::Authenticating {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.trust = TrustState::Trusted;
        self.session = SessionState::Authenticated;
        Ok(())
    }

    pub fn establish(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Authenticated {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.session = SessionState::Established;
        Ok(())
    }

    pub fn begin_rekey(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Established {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.session = SessionState::Rekeying;
        Ok(())
    }

    pub fn finish_rekey(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Rekeying {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.session = SessionState::Established;
        Ok(())
    }

    pub fn close_session(&mut self) {
        self.session = SessionState::Closed;
    }

    pub fn revoke_trust(&mut self) {
        self.trust = TrustState::Revoked;
        self.session = SessionState::Closed;
    }

    pub fn authorize(&self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Established {
            return Err(SecurityError::Unauthorized);
        }

        Ok(())
    }

    pub(crate) fn validate_and_negotiate(&mut self, offered: u16) -> Result<u16, SecurityError> {
        if self.session != SessionState::Authenticated {
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

    fn authenticated() -> SecurityContext {
        let mut ctx = SecurityContext::new(1, CURRENT_PROTOCOL_VERSION).unwrap();
        ctx.begin_authentication().unwrap();
        ctx.accept_verified_authentication().unwrap();
        ctx
    }

    #[test]
    fn unknown_trust_cannot_authorize() {
        let ctx = SecurityContext::new(1, 1).unwrap();
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthorized));
    }

    #[test]
    fn trust_elevation_is_guarded() {
        let mut ctx = SecurityContext::new(1, 1).unwrap();
        ctx.begin_authentication().unwrap();
        assert_eq!(ctx.trust_state(), TrustState::Pending);
        ctx.accept_verified_authentication().unwrap();
        assert_eq!(ctx.trust_state(), TrustState::Trusted);
    }

    #[test]
    fn revocation_closes_context() {
        let mut ctx = authenticated();
        ctx.establish().unwrap();
        ctx.revoke_trust();
        assert_eq!(ctx.trust_state(), TrustState::Revoked);
        assert_eq!(ctx.session_state(), SessionState::Closed);
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthorized));
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
        let mut ctx = authenticated();
        assert_eq!(
            ctx.validate_and_negotiate(0),
            Err(SecurityError::ProtocolDowngrade)
        );
        assert_eq!(
            ctx.validate_and_negotiate(2),
            Err(SecurityError::UnsupportedProtocol)
        );
        assert_eq!(ctx.negotiated_protocol(), 1);
    }
}
