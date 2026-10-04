use crate::protocol::{FrameHeader, ProtocolError, MAX_PAYLOAD_BYTES};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportFrame {
    pub header: FrameHeader,
    pub payload: Vec<u8>,
}

impl TransportFrame {
    pub fn new(header: FrameHeader, payload: Vec<u8>) -> Result<Self, TransportError> {
        header.validate_payload_len(payload.len())?;
        Ok(Self { header, payload })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportState { Disconnected, Connecting, Connected, Closing, Closed }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    Protocol(ProtocolError),
    NotConnected,
    Closing,
    Closed,
    FrameTooLarge,
}

impl From<ProtocolError> for TransportError {
    fn from(value: ProtocolError) -> Self { Self::Protocol(value) }
}

pub trait SecureTransport {
    fn state(&self) -> TransportState;
    fn connect(&mut self) -> Result<(), TransportError>;
    fn send(&mut self, frame: TransportFrame) -> Result<(), TransportError>;
    fn receive(&mut self) -> Result<Option<TransportFrame>, TransportError>;
    fn close(&mut self) -> Result<(), TransportError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mismatched_payload_length_is_rejected_at_boundary() {
        let result = TransportFrame::new(
            FrameHeader { protocol_version: 1, payload_len: 8, flags: 0 },
            vec![0xA5; 4],
        );
        assert_eq!(result, Err(TransportError::Protocol(ProtocolError::PayloadLengthMismatch)));
    }

    #[test]
    fn oversized_frame_is_rejected() {
        let header = FrameHeader {
            protocol_version: 1,
            payload_len: (MAX_PAYLOAD_BYTES + 1) as u32,
            flags: 0,
        };
        let result = TransportFrame::new(header, vec![0xA5; MAX_PAYLOAD_BYTES + 1]);
        assert_eq!(result, Err(TransportError::Protocol(ProtocolError::PayloadTooLarge)));
    }
}
