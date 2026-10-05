use crate::{SecurityContext, SecurityError, SessionState};

#[derive(Debug, PartialEq, Eq)]
pub struct Session {
    protocol: u16,
}

impl Session {
    pub fn establish(
        ctx: &mut SecurityContext,
        offered_protocol: u16,
    ) -> Result<Self, SecurityError> {
        let protocol = ctx.validate_and_negotiate(offered_protocol)?;
        ctx.establish()?;
        Ok(Self { protocol })
    }

    pub fn protocol(&self) -> u16 {
        self.protocol
    }

    pub fn begin_rekey(ctx: &mut SecurityContext) -> Result<(), SecurityError> {
        ctx.begin_rekey()
    }

    #[cfg(test)]
    pub(crate) fn finish_rekey(ctx: &mut SecurityContext) -> Result<(), SecurityError> {
        ctx.finish_rekey()
    }

    pub fn abort_authentication(ctx: &mut SecurityContext) -> Result<(), SecurityError> {
        ctx.abort_authentication()
    }

    pub fn abort_rekey(ctx: &mut SecurityContext) -> Result<(), SecurityError> {
        ctx.abort_rekey()
    }

    pub fn close(ctx: &mut SecurityContext) {
        ctx.close_session();
    }

    pub fn state(ctx: &SecurityContext) -> SessionState {
        ctx.session_state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn authenticated() -> SecurityContext {
        let mut ctx = SecurityContext::new(1, 1).unwrap();
        ctx.begin_authentication().unwrap();
        ctx.accept_verified_authentication().unwrap();
        ctx
    }

    #[test]
    fn establish_does_not_mutate_invalid_context() {
        let mut ctx = SecurityContext::new(1, 1).unwrap();
        assert_eq!(
            Session::establish(&mut ctx, 1),
            Err(SecurityError::InvalidSessionTransition)
        );
        assert_eq!(ctx.negotiated_protocol(), 1);
        assert_eq!(ctx.session_state(), SessionState::Idle);
    }

    #[test]
    fn establish_commits_protocol_and_state() {
        let mut ctx = authenticated();
        let session = Session::establish(&mut ctx, 1).unwrap();
        assert_eq!(session.protocol(), 1);
        assert_eq!(ctx.negotiated_protocol(), session.protocol());
        assert_eq!(ctx.session_state(), SessionState::Established);
    }

    #[test]
    fn rekey_and_close_are_state_guarded() {
        let mut ctx = authenticated();
        assert_eq!(
            Session::begin_rekey(&mut ctx),
            Err(SecurityError::InvalidSessionTransition)
        );
        Session::establish(&mut ctx, 1).unwrap();
        Session::begin_rekey(&mut ctx).unwrap();
        Session::finish_rekey(&mut ctx).unwrap();
        assert_eq!(Session::state(&ctx), SessionState::Established);
        Session::close(&mut ctx);
        assert_eq!(Session::state(&ctx), SessionState::Closed);
        assert_eq!(
            Session::finish_rekey(&mut ctx),
            Err(SecurityError::InvalidSessionTransition)
        );
    }
}
