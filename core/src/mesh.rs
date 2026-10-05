use crate::OpaqueId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpaquePayload(Vec<u8>);

impl OpaquePayload {
    pub fn new(bytes: Vec<u8>) -> Result<Self, TransportError> {
        if bytes.is_empty() || bytes.len() > crate::MAX_PAYLOAD_BYTES {
            return Err(TransportError::InvalidPayload);
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpaqueFrame {
    protocol_version: u16,
    source: OpaqueId,
    destination: OpaqueId,
    payload: OpaquePayload,
    hop_limit: u8,
}

impl OpaqueFrame {
    pub fn new(
        protocol_version: u16,
        source: OpaqueId,
        destination: OpaqueId,
        payload: OpaquePayload,
        hop_limit: u8,
    ) -> Result<Self, TransportError> {
        if protocol_version == 0 || hop_limit == 0 {
            return Err(TransportError::InvalidFrame);
        }
        Ok(Self {
            protocol_version,
            source,
            destination,
            payload,
            hop_limit,
        })
    }

    pub fn protocol_version(&self) -> u16 {
        self.protocol_version
    }

    pub fn source(&self) -> &OpaqueId {
        &self.source
    }

    pub fn destination(&self) -> &OpaqueId {
        &self.destination
    }

    pub fn payload(&self) -> &OpaquePayload {
        &self.payload
    }

    pub fn hop_limit(&self) -> u8 {
        self.hop_limit
    }

    pub fn decrement_hop_limit(&mut self) -> Result<(), TransportError> {
        self.hop_limit = self
            .hop_limit
            .checked_sub(1)
            .filter(|value| *value > 0)
            .ok_or(TransportError::HopLimitExceeded)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    InvalidPayload,
    InvalidFrame,
    HopLimitExceeded,
}

pub trait MeshTransport {
    fn send(&mut self, frame: OpaqueFrame) -> Result<(), TransportError>;
    fn receive(&mut self) -> Option<OpaqueFrame>;
}

#[derive(Debug, Default)]
pub struct InMemoryOpaqueTransport {
    frames: Vec<OpaqueFrame>,
}

impl MeshTransport for InMemoryOpaqueTransport {
    fn send(&mut self, frame: OpaqueFrame) -> Result<(), TransportError> {
        self.frames.push(frame);
        Ok(())
    }

    fn receive(&mut self) -> Option<OpaqueFrame> {
        if self.frames.is_empty() {
            None
        } else {
            Some(self.frames.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u8) -> OpaqueId {
        OpaqueId::new(vec![value; 8]).unwrap()
    }

    #[test]
    fn transport_accepts_only_bounded_opaque_payloads() {
        assert_eq!(
            OpaquePayload::new(Vec::new()),
            Err(TransportError::InvalidPayload)
        );
        assert!(OpaquePayload::new(vec![0xAA; 32]).is_ok());
    }

    #[test]
    fn forwarding_is_hop_limited() {
        let mut frame = OpaqueFrame::new(
            1,
            id(1),
            id(2),
            OpaquePayload::new(vec![0xAA; 4]).unwrap(),
            2,
        )
        .unwrap();
        frame.decrement_hop_limit().unwrap();
        assert_eq!(frame.hop_limit(), 1);
        assert_eq!(
            frame.decrement_hop_limit(),
            Err(TransportError::HopLimitExceeded)
        );
    }

    #[test]
    fn in_memory_transport_does_not_transform_payload() {
        let mut transport = InMemoryOpaqueTransport::default();
        let frame = OpaqueFrame::new(
            1,
            id(1),
            id(2),
            OpaquePayload::new(vec![0xAA; 8]).unwrap(),
            4,
        )
        .unwrap();
        transport.send(frame.clone()).unwrap();
        assert_eq!(transport.receive(), Some(frame));
    }
}
