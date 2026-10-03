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
        Ok(Self { protocol })
    }

    pub fn close(ctx: &mut SecurityContext) {
        ctx.close_session();
    }
}
