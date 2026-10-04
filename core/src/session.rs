use crate::{SecurityContext, SecurityError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Session {
    pub protocol: u16,
}

impl Session {
    pub fn establish(
        ctx: &mut SecurityContext,
        offered_protocol: u16,
    ) -> Result<Self, SecurityError> {
        let protocol = ctx.negotiate_protocol(offered_protocol)?;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn establishing_session_requires_authenticated_context() {
        let mut ctx = SecurityContext::new(1);
        assert_eq!(
            Session::establish(&mut ctx, 1),
            Err(SecurityError::InvalidSessionTransition)
        );
    }

    #[test]
    fn establishing_session_locks_in_negotiated_protocol() {
        let mut ctx = SecurityContext::new(1);
        ctx.begin_authentication().unwrap();
        ctx.authenticate().unwrap();

        let session = Session::establish(&mut ctx, 3).unwrap();

        assert_eq!(session.protocol, 3);
        assert_eq!(ctx.session, crate::SessionState::Established);
        assert_eq!(ctx.negotiated_protocol, 3);
    }
}
