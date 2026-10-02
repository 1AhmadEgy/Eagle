use crate::{SecurityContext, SecurityError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability { Read, Write, Administrative }

pub fn authorize(ctx: &SecurityContext, capability: Capability) -> Result<(), SecurityError> {
    ctx.authorize()?;
    if matches!(capability, Capability::Administrative) { return Err(SecurityError::Unauthorized); }
    Ok(())
}
