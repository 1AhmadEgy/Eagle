#![forbid(unsafe_code)]

pub mod identity;
pub mod policy;
pub mod session;

pub use identity::{Identity, PrincipalId};
pub use policy::{authorize, Capability};
pub use session::Session;

/// Deterministic security-state primitives.
///
/// This crate intentionally contains no cryptographic implementation.
/// Cryptography and protocol profiles remain behind approved ADR and
/// component-adoption gates.
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
        if self.session != SessionState::Idle || self.trust != TrustState::Untrusted {
            return Err(SecurityError::InvalidSessionTransition);
        }
        self.trust = TrustState::Pending;
        Ok(())
    }

    /// Records successful authentication performed by an external
    /// cryptographic/platform authenticator. This function performs no
    /// cryptography and must never be treated as identity verification.
    pub fn mark_authenticated(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Pending || self.session != SessionState::Idle {
            return Err(SecurityError::InvalidSessionTransition);
        }
        self.trust = TrustState::Trusted;
        Ok(())
    }

    pub fn open_session(&mut self) -> Result<(), SecurityError> {
        if self.trust != TrustState::Trusted || self.session != SessionState::Idle {
            return Err(SecurityError::InvalidSessionTransition);
        }
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
        if self.session != SessionState::Idle {
            return Err(SecurityError::InvalidSessionTransition);
        }
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
            ctx.mark_authenticated(),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn successful_authentication_trusts_but_does_not_fake_session() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.mark_authenticated().unwrap();
        assert_eq!(ctx.trust_state(), TrustState::Trusted);
        assert_eq!(ctx.session_state(), SessionState::Idle);
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthorized));
    }

    #[test]
    fn session_opening_requires_trusted_identity() {
        let mut ctx = SecurityContext::new(1);
        assert_eq!(
            ctx.open_session(),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn successful_session_establishes_authorized_state() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.mark_authenticated().unwrap();
        ctx.open_session().unwrap();
        assert_eq!(ctx.authorize(), Ok(()));
        assert_eq!(ctx.session_state(), SessionState::Authenticated);
    }

    #[test]
    fn revocation_closes_session_and_blocks_authorization() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.mark_authenticated().unwrap();
        ctx.open_session().unwrap();
        ctx.revoke_trust();
        assert_eq!(ctx.trust_state(), TrustState::Revoked);
        assert_eq!(ctx.session_state(), SessionState::Closed);
        assert_eq!(ctx.authorize(), Err(SecurityError::Unauthorized));
    }

    #[test]
    fn revoked_context_cannot_restart_authentication() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.mark_authenticated().unwrap();
        ctx.open_session().unwrap();
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
    fn protocol_cannot_be_changed_after_session_establishment() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.mark_authenticated().unwrap();
        ctx.negotiate_protocol(3).unwrap();
        ctx.open_session().unwrap();

        assert_eq!(
            ctx.negotiate_protocol(4),
            Err(SecurityError::InvalidSessionTransition)
        );
        assert_eq!(ctx.negotiated_protocol(), 3);
        assert_eq!(ctx.session_state(), SessionState::Authenticated);
    }
}
