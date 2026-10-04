use crate::{SecurityContext, SecurityError, SessionState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Session {
    pub protocol: u16,
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

    pub fn begin_rekey(ctx: &mut SecurityContext) -> Result<(), SecurityError> {
        ctx.begin_rekey()
    }

    pub fn finish_rekey(ctx: &mut SecurityContext) -> Result<(), SecurityError> {
        ctx.finish_rekey()
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
}
