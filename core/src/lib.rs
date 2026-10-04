#![forbid(unsafe_code)]

pub mod identity;
pub mod policy;
pub mod protocol;
pub mod session;

pub use identity::{Identity, PrincipalId};
pub use policy::{authorize, Capability};
pub use protocol::{EncryptedEnvelope, FrameHeader, MessageId, OpaqueId, ProtocolError};
pub use session::Session;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustState {
    Untrusted,
    Pending,
    Trusted,
    Revoked,
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
    Failed,
    RecoveryRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityError {
    Unauthenticated,
    Unauthorized,
    InvalidTrustTransition,
    InvalidSessionTransition,
    ProtocolDowngrade,
    RecoveryRequired,
    ClosedSession,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityContext {
    pub trust: TrustState,
    pub session: SessionState,
    pub negotiated_protocol: u16,
    pub minimum_protocol: u16,
}

impl SecurityContext {
    pub fn new(minimum_protocol: u16) -> Self {
        Self {
            trust: TrustState::Untrusted,
            session: SessionState::Idle,
            negotiated_protocol: minimum_protocol,
            minimum_protocol,
        }
    }

    pub fn begin_authentication(&mut self) -> Result<(), SecurityError> {
        if self.trust == TrustState::Revoked {
            return Err(SecurityError::InvalidTrustTransition);
        }
        if self.session != SessionState::Idle {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.trust = TrustState::Pending;
        self.session = SessionState::Authenticating;
        Ok(())
    }

    /// State transition only. This does not perform cryptographic verification.
    pub fn authenticate(&mut self) -> Result<(), SecurityError> {
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
        if self.session != SessionState::Established || self.trust != TrustState::Trusted {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.session = SessionState::Rekeying;
        Ok(())
    }

    pub fn finish_rekey(&mut self) -> Result<(), SecurityError> {
        if self.session != SessionState::Rekeying || self.trust != TrustState::Trusted {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.session = SessionState::Established;
        Ok(())
    }

    pub fn begin_closing(&mut self) -> Result<(), SecurityError> {
        match self.session {
            SessionState::Established | SessionState::Rekeying | SessionState::Authenticated => {
                self.session = SessionState::Closing;
                Ok(())
            }
            SessionState::Closing | SessionState::Closed => Err(SecurityError::ClosedSession),
            _ => Err(SecurityError::InvalidSessionTransition),
        }
    }

    pub fn close_session(&mut self) {
        self.session = SessionState::Closed;
    }

    pub fn fail(&mut self) {
        self.session = SessionState::Failed;
    }

    pub fn require_recovery(&mut self) -> Result<(), SecurityError> {
        if self.session != SessionState::Failed {
            return Err(SecurityError::InvalidSessionTransition);
        }

        self.session = SessionState::RecoveryRequired;
        Ok(())
    }

    pub fn revoke_trust(&mut self) {
        self.trust = TrustState::Revoked;
        self.session = SessionState::Closed;
    }

    pub fn authorize(&self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted {
            return Err(SecurityError::Unauthorized);
        }
        if self.session != SessionState::Established {
            return Err(SecurityError::Unauthenticated);
        }
        Ok(())
    }

    pub fn negotiate_protocol(&mut self, offered: u16) -> Result<u16, SecurityError> {
        if offered < self.minimum_protocol || offered < self.negotiated_protocol {
            return Err(SecurityError::ProtocolDowngrade);
        }
        self.negotiated_protocol = offered;
        Ok(offered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn established_context() -> SecurityContext {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.authenticate().unwrap();
        ctx.establish().unwrap();
        ctx
    }

    #[test]
    fn unauthenticated_context_cannot_authorize() {
        let ctx = SecurityContext::new(1);
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthorized));
    }

    #[test]
    fn authentication_requires_pending_state() {
        let mut ctx = SecurityContext::new(1);
        assert_eq!(ctx.authenticate(), Err(SecurityError::InvalidSessionTransition));
    }

    #[test]
    fn successful_authentication_requires_explicit_establishment() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.authenticate().unwrap();
        assert_eq!(ctx.session, SessionState::Authenticated);
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthenticated));
        ctx.establish().unwrap();
        assert_eq!(ctx.authorize(), Ok(()));
    }

    #[test]
    fn rekey_returns_to_established() {
        let mut ctx = established_context();
        ctx.begin_rekey().unwrap();
        assert_eq!(ctx.session, SessionState::Rekeying);
        ctx.finish_rekey().unwrap();
        assert_eq!(ctx.session, SessionState::Established);
        assert_eq!(ctx.authorize(), Ok(()));
    }

    #[test]
    fn failure_requires_explicit_recovery_transition() {
        let mut ctx = established_context();
        ctx.fail();
        assert_eq!(ctx.session, SessionState::Failed);
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthenticated));
        ctx.require_recovery().unwrap();
        assert_eq!(ctx.session, SessionState::RecoveryRequired);
    }

    #[test]
    fn revocation_closes_session_and_blocks_authorization() {
        let mut ctx = established_context();
        ctx.revoke_trust();
        assert_eq!(ctx.trust, TrustState::Revoked);
        assert_eq!(ctx.session, SessionState::Closed);
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthorized));
    }

    #[test]
    fn revoked_trust_cannot_restart_authentication() {
        let mut ctx = established_context();
        ctx.revoke_trust();
        assert_eq!(
            ctx.begin_authentication(),
            Err(SecurityError::InvalidTrustTransition)
        );
    }

    #[test]
    fn below_minimum_protocol_is_rejected_without_mutation() {
        let mut ctx = SecurityContext::new(2);
        assert_eq!(ctx.negotiate_protocol(1), Err(SecurityError::ProtocolDowngrade));
        assert_eq!(ctx.negotiated_protocol, 2);
    }

    #[test]
    fn renegotiation_cannot_downgrade_after_upgrade() {
        let mut ctx = SecurityContext::new(1);
        assert_eq!(ctx.negotiate_protocol(3), Ok(3));
        assert_eq!(ctx.negotiate_protocol(2), Err(SecurityError::ProtocolDowngrade));
        assert_eq!(ctx.negotiated_protocol, 3);
        assert_eq!(ctx.negotiate_protocol(1), Err(SecurityError::ProtocolDowngrade));
        assert_eq!(ctx.negotiated_protocol, 3);
    }
}
