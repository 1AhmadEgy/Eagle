#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceTrustState {
    Unknown,
    Pending,
    Trusted,
    Revoked,
    Replaced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Android,
    Desktop,
    Ios,
    Web,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    InvalidTransition,
    Revoked,
    Replaced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Device {
    pub account: u64,
    pub device: u64,
    pub platform: Platform,
    trust: DeviceTrustState,
}

impl Device {
    pub fn new(account: u64, device: u64, platform: Platform) -> Self {
        Self {
            account,
            device,
            platform,
            trust: DeviceTrustState::Unknown,
        }
    }

    pub fn trust_state(&self) -> DeviceTrustState {
        self.trust
    }

    #[cfg(test)]
    #[expect(
        dead_code,
        reason = "test scaffold until approved pairing verifier is integrated"
    )]
    pub(crate) fn begin_pairing(&mut self) -> Result<(), DeviceError> {
        if self.trust != DeviceTrustState::Unknown {
            return Err(DeviceError::InvalidTransition);
        }
        self.trust = DeviceTrustState::Pending;
        Ok(())
    }

    #[cfg(test)]
    #[expect(
        dead_code,
        reason = "test scaffold until approved pairing verifier is integrated"
    )]
    pub(crate) fn approve(&mut self) -> Result<(), DeviceError> {
        if self.trust != DeviceTrustState::Pending {
            return Err(DeviceError::InvalidTransition);
        }
        self.trust = DeviceTrustState::Trusted;
        Ok(())
    }

    pub fn revoke(&mut self) -> Result<(), DeviceError> {
        if self.trust != DeviceTrustState::Trusted {
            return Err(DeviceError::InvalidTransition);
        }
        self.trust = DeviceTrustState::Revoked;
        Ok(())
    }

    pub fn replace(&mut self) -> Result<(), DeviceError> {
        if self.trust != DeviceTrustState::Trusted {
            return Err(DeviceError::InvalidTransition);
        }
        self.trust = DeviceTrustState::Replaced;
        Ok(())
    }

    pub fn can_authorize(&self) -> Result<(), DeviceError> {
        match self.trust {
            DeviceTrustState::Trusted => Ok(()),
            DeviceTrustState::Revoked => Err(DeviceError::Revoked),
            DeviceTrustState::Replaced => Err(DeviceError::Replaced),
            DeviceTrustState::Unknown | DeviceTrustState::Pending => {
                Err(DeviceError::InvalidTransition)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_requires_pairing() {
        let mut device = Device::new(1, 2, Platform::Android);
        assert_eq!(device.approve(), Err(DeviceError::InvalidTransition));
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        assert_eq!(device.trust_state(), DeviceTrustState::Trusted);
        assert_eq!(device.can_authorize(), Ok(()));
    }

    #[test]
    fn revoked_and_replaced_fail_closed() {
        let mut revoked = Device::new(1, 2, Platform::Desktop);
        revoked.begin_pairing().unwrap();
        revoked.approve().unwrap();
        revoked.revoke().unwrap();
        assert_eq!(revoked.can_authorize(), Err(DeviceError::Revoked));

        let mut replaced = Device::new(1, 3, Platform::Ios);
        replaced.begin_pairing().unwrap();
        replaced.approve().unwrap();
        replaced.replace().unwrap();
        assert_eq!(replaced.can_authorize(), Err(DeviceError::Replaced));
    }
}
