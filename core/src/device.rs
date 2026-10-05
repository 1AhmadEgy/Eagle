use std::fmt;

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

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidTransition => "invalid device trust transition",
            Self::Revoked => "device is revoked",
            Self::Replaced => "device is replaced",
        })
    }
}

impl std::error::Error for DeviceError {}

#[derive(Debug, PartialEq, Eq)]
pub struct Device {
    account: u64,
    device: u64,
    platform: Platform,
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

    pub fn account(&self) -> u64 {
        self.account
    }

    pub fn device(&self) -> u64 {
        self.device
    }

    pub fn platform(&self) -> Platform {
        self.platform
    }

    pub fn trust_state(&self) -> DeviceTrustState {
        self.trust
    }

    #[cfg(test)]
    pub(crate) fn begin_pairing(&mut self) -> Result<(), DeviceError> {
        if self.trust != DeviceTrustState::Unknown {
            return Err(DeviceError::InvalidTransition);
        }
        self.trust = DeviceTrustState::Pending;
        Ok(())
    }

    #[cfg(test)]
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
        assert_eq!(device.account(), 1);
        assert_eq!(device.device(), 2);
        assert_eq!(device.platform(), Platform::Android);
        assert_eq!(device.trust_state(), DeviceTrustState::Unknown);
        assert_eq!(device.approve(), Err(DeviceError::InvalidTransition));

        device.begin_pairing().unwrap();
        assert_eq!(device.begin_pairing(), Err(DeviceError::InvalidTransition));

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
        assert_eq!(revoked.revoke(), Err(DeviceError::InvalidTransition));
        assert_eq!(revoked.replace(), Err(DeviceError::InvalidTransition));

        let mut replaced = Device::new(1, 3, Platform::Ios);
        replaced.begin_pairing().unwrap();
        replaced.approve().unwrap();
        replaced.replace().unwrap();
        assert_eq!(replaced.can_authorize(), Err(DeviceError::Replaced));
        assert_eq!(replaced.revoke(), Err(DeviceError::InvalidTransition));
        assert_eq!(replaced.replace(), Err(DeviceError::InvalidTransition));
    }
}
