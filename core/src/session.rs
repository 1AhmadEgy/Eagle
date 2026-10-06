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
        if ctx.trust_state() != crate::TrustState::Trusted {
            return Err(SecurityError::Unauthorized);
        }
        let protocol = ctx.negotiate_protocol(offered_protocol)?;
        ctx.open_session()?;
        Ok(Self { protocol })
    }

    pub fn close(ctx: &mut SecurityContext) {
        ctx.close_session();
    }
}
