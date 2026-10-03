use crate::{SecurityContext, SecurityError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrincipalId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identity {
    pub principal: PrincipalId,
}

impl Identity {
    /// State-machine primitive used only after an external authenticator has
    /// performed the real authentication/verification.
    ///
    /// This function performs no cryptography and must not be treated as
    /// production identity verification.
    pub fn authenticate(
        ctx: &mut SecurityContext,
        principal: PrincipalId,
    ) -> Result<Self, SecurityError> {
        ctx.begin_authentication()?;
        ctx.authenticate()?;
        Ok(Self { principal })
    }
}
