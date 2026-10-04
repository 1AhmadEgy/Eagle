use crate::protocol::{OpaqueId, ProtocolError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceTrustState {
    Unknown,
    Pending,
    Trusted,
    Revoked,
    Replaced,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub device_id: OpaqueId,
    pub account_id: OpaqueId,
    pub platform: Platform,
    pub trust: DeviceTrustState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Android,
    Ios,
    Web,
    Desktop,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    InvalidTransition,
    RevokedDevice,
    ReplacedDevice,
    Protocol(ProtocolError),
}

impl From<ProtocolError> for DeviceError {
    fn from(value: ProtocolError) -> Self {
        Self::Protocol(value)
    }
}

impl Device {
    pub fn new(device_id: OpaqueId, account_id: OpaqueId, platform: Platform) -> Self {
        Self {
            device_id,
            account_id,
            platform,
            trust: DeviceTrustState::Unknown,
        }
    }

    pub fn begin_pairing(&mut self) -> Result<(), DeviceError> {
        if self.trust != DeviceTrustState::Unknown {
            return Err(DeviceError::InvalidTransition);
        }
        self.trust = DeviceTrustState::Pending;
        Ok(())
    }

    pub fn approve(&mut self) -> Result<(), DeviceError> {
        if self.trust != DeviceTrustState::Pending {
            return Err(DeviceError::InvalidTransition);
        }
        self.trust = DeviceTrustState::Trusted;
        Ok(())
    }

    pub fn revoke(&mut self) -> Result<(), DeviceError> {
        if matches!(
            self.trust,
            DeviceTrustState::Unknown | DeviceTrustState::Revoked
        ) {
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

    pub fn can_receive_new_messages(&self) -> Result<(), DeviceError> {
        match self.trust {
            DeviceTrustState::Trusted => Ok(()),
            DeviceTrustState::Revoked => Err(DeviceError::RevokedDevice),
            DeviceTrustState::Replaced => Err(DeviceError::ReplacedDevice),
            DeviceTrustState::Unknown | DeviceTrustState::Pending => {
                Err(DeviceError::InvalidTransition)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(byte: u8) -> OpaqueId {
        OpaqueId::new(vec![byte; 8]).unwrap()
    }

    #[test]
    fn pairing_requires_pending_then_approval() {
        let mut device = Device::new(id(1), id(2), Platform::Android);
        assert_eq!(device.approve(), Err(DeviceError::InvalidTransition));
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        assert_eq!(device.trust, DeviceTrustState::Trusted);
        assert_eq!(device.can_receive_new_messages(), Ok(()));
    }

    #[test]
    fn revoked_device_cannot_receive_new_messages() {
        let mut device = Device::new(id(1), id(2), Platform::Android);
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        device.revoke().unwrap();
        assert_eq!(
            device.can_receive_new_messages(),
            Err(DeviceError::RevokedDevice)
        );
    }

    #[test]
    fn revoked_device_cannot_be_revoked_again() {
        let mut device = Device::new(id(1), id(2), Platform::Android);
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        device.revoke().unwrap();
        assert_eq!(device.revoke(), Err(DeviceError::InvalidTransition));
    }

    #[test]
    fn trusted_device_can_be_replaced_but_replaced_cannot_receive() {
        let mut device = Device::new(id(1), id(2), Platform::Android);
        device.begin_pairing().unwrap();
        device.approve().unwrap();
        device.replace().unwrap();
        assert_eq!(
            device.can_receive_new_messages(),
            Err(DeviceError::ReplacedDevice)
        );
    }
}
