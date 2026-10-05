use crate::{SecurityContext, SecurityError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Read,
    Write,
    Administrative,
}

pub fn authorize(ctx: &SecurityContext, capability: Capability) -> Result<(), SecurityError> {
    ctx.authorize()?;
    match capability {
        Capability::Read | Capability::Write => Ok(()),
        Capability::Administrative => Err(SecurityError::Unauthorized),
    }
}
