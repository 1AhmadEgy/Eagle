#![forbid(unsafe_code)]

pub mod identity;
pub mod policy;
pub mod session;

pub use identity::{Identity, PrincipalId};
pub use policy::{authorize, Capability};
pub use session::Session;

/// Deterministic security-kernel primitives.
///
/// This crate intentionally contains no cryptographic implementation.
/// Cryptography and protocol profiles stay behind their approved ADR gates.
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
    Authenticated,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityError {
    Unauthorized,
    InvalidSessionTransition,
    ProtocolDowngrade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityContext {
    trust: TrustState,
    session: SessionState,
    negotiated_protocol: u16,
    minimum_protocol: u16,
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

    pub fn begin_authentication(&mut self) -> Result<(), SecurityError> {
        if self.session != SessionState::Idle {
            return Err(SecurityError::InvalidSessionTransition);
        }
        self.trust = TrustState::Pending;
        Ok(())
    }

    pub fn authenticate(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Pending || self.session != SessionState::Idle {
            return Err(SecurityError::InvalidSessionTransition);
        }
        self.trust = TrustState::Trusted;
        self.session = SessionState::Authenticated;
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
        if self.trust != TrustState::Trusted || self.session != SessionState::Authenticated {
            return Err(SecurityError::Unauthorized);
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

    #[test]
    fn unauthenticated_context_cannot_authorize() {
        let ctx = SecurityContext::new(1);
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthorized));
    }

    #[test]
    fn authentication_requires_pending_state() {
        let mut ctx = SecurityContext::new(1);
        assert_eq!(
            ctx.authenticate(),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn successful_authentication_establishes_trusted_session() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.authenticate().unwrap();
        assert_eq!(ctx.authorize(), Ok(()));
        assert_eq!(ctx.trust_state(), TrustState::Trusted);
        assert_eq!(ctx.session_state(), SessionState::Authenticated);
    }

    #[test]
    fn revocation_closes_session_and_blocks_authorization() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.authenticate().unwrap();
        ctx.revoke_trust();
        assert_eq!(ctx.trust_state(), TrustState::Revoked);
        assert_eq!(ctx.session_state(), SessionState::Closed);
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthorized));
    }

    #[test]
    fn revoked_context_cannot_restart_authentication() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.authenticate().unwrap();
        ctx.revoke_trust();
        assert_eq!(
            ctx.begin_authentication(),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn below_minimum_protocol_is_rejected_without_mutation() {
        let mut ctx = SecurityContext::new(2);
        assert_eq!(
            ctx.negotiate_protocol(1),
            Err(SecurityError::ProtocolDowngrade)
        );
        assert_eq!(ctx.negotiated_protocol(), 2);
    }

    #[test]
    fn renegotiation_cannot_downgrade_after_upgrade() {
        let mut ctx = SecurityContext::new(1);
        assert_eq!(ctx.negotiate_protocol(3), Ok(3));
        assert_eq!(
            ctx.negotiate_protocol(2),
            Err(SecurityError::ProtocolDowngrade)
        );
        assert_eq!(ctx.negotiated_protocol(), 3);
        assert_eq!(
            ctx.negotiate_protocol(1),
            Err(SecurityError::ProtocolDowngrade)
        );
        assert_eq!(ctx.negotiate_protocol(4), Ok(4));
        assert_eq!(ctx.negotiated_protocol(), 4);
    }
}
