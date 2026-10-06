use crate::{SecurityContext, SecurityError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrincipalId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identity {
    pub principal: PrincipalId,
}

impl Identity {
    /// Creates an identity record from authentication already completed by an
    /// external authenticator. No cryptography is performed here.
    pub fn from_verified_principal(
        ctx: &mut SecurityContext,
        principal: PrincipalId,
    ) -> Result<Self, SecurityError> {
        ctx.begin_authentication()?;
        ctx.mark_authenticated()?;
        Ok(Self { principal })
    }
}
