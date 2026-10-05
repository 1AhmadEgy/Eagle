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
    IdentityMismatch,
    AuthorityMismatch,
    AuthorityExhausted,
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidTransition => "invalid device trust transition",
            Self::Revoked => "device is revoked",
            Self::Replaced => "device is replaced",
            Self::IdentityMismatch => "device identity does not match",
            Self::AuthorityMismatch => "device authority epoch does not match",
            Self::AuthorityExhausted => "device authority epoch exhausted",
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
    authority_epoch: u64,
}

impl Device {
    pub fn new(account: u64, device: u64, platform: Platform) -> Self {
        Self {
            account,
            device,
            platform,
            trust: DeviceTrustState::Unknown,
            authority_epoch: 0,
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

    pub(crate) fn authority_epoch(&self) -> u64 {
        self.authority_epoch
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
        let next_epoch = self
            .authority_epoch
            .checked_add(1)
            .ok_or(DeviceError::AuthorityExhausted)?;
        self.trust = DeviceTrustState::Revoked;
        self.authority_epoch = next_epoch;
        Ok(())
    }

    pub fn replace(&mut self) -> Result<(), DeviceError> {
        if self.trust != DeviceTrustState::Trusted {
            return Err(DeviceError::InvalidTransition);
        }
        let next_epoch = self
            .authority_epoch
            .checked_add(1)
            .ok_or(DeviceError::AuthorityExhausted)?;
        self.trust = DeviceTrustState::Replaced;
        self.authority_epoch = next_epoch;
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

    pub(crate) fn validate_authority(
        &self,
        account: u64,
        device: u64,
        epoch: u64,
    ) -> Result<(), DeviceError> {
        if self.account != account || self.device != device {
            return Err(DeviceError::IdentityMismatch);
        }
        self.can_authorize()?;
        if self.authority_epoch != epoch {
            return Err(DeviceError::AuthorityMismatch);
        }
        Ok(())
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
        assert_eq!(device.authority_epoch(), 0);
        assert_eq!(device.approve(), Err(DeviceError::InvalidTransition));

        device.begin_pairing().unwrap();
        assert_eq!(device.begin_pairing(), Err(DeviceError::InvalidTransition));

        device.approve().unwrap();
        assert_eq!(device.trust_state(), DeviceTrustState::Trusted);
        assert_eq!(device.can_authorize(), Ok(()));
    }

    #[test]
    fn authority_epoch_overflow_does_not_partially_apply_revocation() {
        let mut device = Device::new(1, 2, Platform::Android);
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        device.authority_epoch = u64::MAX;
        assert_eq!(device.revoke(), Err(DeviceError::AuthorityExhausted));
        assert_eq!(device.trust_state(), DeviceTrustState::Trusted);
        assert_eq!(device.authority_epoch(), u64::MAX);
    }

    #[test]
    fn revoked_and_replaced_fail_closed() {
        let mut revoked = Device::new(1, 2, Platform::Desktop);
        revoked.begin_pairing().unwrap();
        revoked.approve().unwrap();
        revoked.revoke().unwrap();
        assert_eq!(revoked.authority_epoch(), 1);
        assert_eq!(revoked.can_authorize(), Err(DeviceError::Revoked));
        assert_eq!(revoked.revoke(), Err(DeviceError::InvalidTransition));
        assert_eq!(revoked.replace(), Err(DeviceError::InvalidTransition));

        let mut replaced = Device::new(1, 3, Platform::Ios);
        replaced.begin_pairing().unwrap();
        replaced.approve().unwrap();
        replaced.replace().unwrap();
        assert_eq!(replaced.authority_epoch(), 1);
        assert_eq!(replaced.can_authorize(), Err(DeviceError::Replaced));
        assert_eq!(replaced.revoke(), Err(DeviceError::InvalidTransition));
        assert_eq!(replaced.replace(), Err(DeviceError::InvalidTransition));
    }
}
