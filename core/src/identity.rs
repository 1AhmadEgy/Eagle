use crate::{SecurityError, SecurityContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrincipalId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identity {
    pub principal: PrincipalId,
}

impl Identity {
    pub fn authenticate(ctx: &mut SecurityContext, principal: PrincipalId) -> Result<Self, SecurityError> {
        ctx.begin_authentication()?;
        ctx.authenticate()?;
        Ok(Self { principal })
    }
}
