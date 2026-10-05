use crate::{Device, SecurityContext, SecurityError, SessionState};

#[derive(Debug, PartialEq, Eq)]
pub struct Session {
    protocol: u16,
}

impl Session {
    pub fn establish(
        ctx: &mut SecurityContext,
        device: &Device,
        offered_protocol: u16,
    ) -> Result<Self, SecurityError> {
        let protocol = ctx.validate_and_negotiate(device, offered_protocol)?;
        ctx.establish(device)?;
        Ok(Self { protocol })
    }

    pub fn protocol(&self) -> u16 {
        self.protocol
    }

    pub fn begin_rekey(ctx: &mut SecurityContext, device: &Device) -> Result<(), SecurityError> {
        ctx.begin_rekey(device)
    }

    #[cfg(test)]
    pub(crate) fn finish_rekey(
        ctx: &mut SecurityContext,
        device: &Device,
    ) -> Result<(), SecurityError> {
        ctx.finish_rekey(device)
    }

    pub fn abort_rekey(ctx: &mut SecurityContext) -> Result<(), SecurityError> {
        ctx.abort_rekey()
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

    fn authenticated() -> (SecurityContext, Device) {
        let mut device = crate::Device::new(1, 2, crate::Platform::Android);
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        let mut ctx = SecurityContext::new(1, 1).unwrap();
        ctx.bind_device(&device).unwrap();
        ctx.begin_authentication().unwrap();
        ctx.accept_verified_authentication(&device).unwrap();
        (ctx, device)
    }

    #[test]
    fn establish_does_not_mutate_invalid_context() {
        let mut ctx = SecurityContext::new(1, 1).unwrap();
        let device = crate::Device::new(1, 2, crate::Platform::Android);
        assert_eq!(
            Session::establish(&mut ctx, &device, 1),
            Err(SecurityError::InvalidSessionTransition)
        );
        assert_eq!(ctx.negotiated_protocol(), 1);
        assert_eq!(ctx.session_state(), SessionState::Idle);
    }

    #[test]
    fn aborted_rekey_closes_session() {
        let (mut ctx, device) = authenticated();
        Session::establish(&mut ctx, &device, 1).unwrap();
        Session::begin_rekey(&mut ctx, &device).unwrap();
        assert_eq!(Session::abort_rekey(&mut ctx), Ok(()));
        assert_eq!(Session::state(&ctx), SessionState::Closed);
    }

    #[test]
    fn establish_commits_protocol_and_state() {
        let (mut ctx, device) = authenticated();
        let session = Session::establish(&mut ctx, &device, 1).unwrap();
        assert_eq!(session.protocol(), 1);
        assert_eq!(ctx.negotiated_protocol(), session.protocol());
        assert_eq!(ctx.session_state(), SessionState::Established);
    }

    #[test]
    fn rekey_and_close_are_state_guarded() {
        let (mut ctx, device) = authenticated();
        assert_eq!(
            Session::begin_rekey(&mut ctx, &device),
            Err(SecurityError::InvalidSessionTransition)
        );
        Session::establish(&mut ctx, &device, 1).unwrap();
        Session::begin_rekey(&mut ctx, &device).unwrap();
        Session::finish_rekey(&mut ctx, &device).unwrap();
        assert_eq!(Session::state(&ctx), SessionState::Established);
        Session::close(&mut ctx);
        assert_eq!(Session::state(&ctx), SessionState::Closed);
        assert_eq!(
            Session::finish_rekey(&mut ctx, &device),
            Err(SecurityError::InvalidSessionTransition)
        );
    }
}
