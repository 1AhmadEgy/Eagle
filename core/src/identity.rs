use crate::{SecurityContext, SecurityError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrincipalId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identity {
    pub principal: PrincipalId,
}

impl Identity {
    /// Deterministic state-machine primitive only.
    ///
    /// This does NOT perform cryptographic authentication and must not be
    /// treated as production identity verification.
    pub fn authenticate(
        ctx: &mut SecurityContext,
        principal: PrincipalId,
    ) -> Result<Self, SecurityError> {
        ctx.begin_authentication()?;
        ctx.authenticate()?;
        Ok(Self { principal })
    }
}
