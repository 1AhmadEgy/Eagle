use crate::protocol::{FrameHeader, ProtocolError, MAX_PAYLOAD_BYTES};

/// An opaque, already-protected transport payload.
///
/// The transport boundary may carry bytes, but must never interpret them as
/// application plaintext or cryptographic key material.
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
pub enum TransportState {
    Disconnected,
    Connecting,
    Connected,
    Closing,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    Protocol(ProtocolError),
    NotConnected,
    Closing,
    Closed,
    FrameTooLarge,
}

impl From<ProtocolError> for TransportError {
    fn from(value: ProtocolError) -> Self {
        Self::Protocol(value)
    }
}

pub trait SecureTransport {
    fn state(&self) -> TransportState;
    fn connect(&mut self) -> Result<(), TransportError>;
    fn send(&mut self, frame: TransportFrame) -> Result<(), TransportError>;
    fn receive(&mut self) -> Result<Option<TransportFrame>, TransportError>;
    fn close(&mut self) -> Result<(), TransportError>;
}

/// Deterministic in-memory adapter for contract tests only.
///
/// No sockets, discovery, retries, background scheduling, serialization, or
/// plaintext interpretation are implemented here.
#[derive(Debug)]
pub struct MemoryTransport {
    state: TransportState,
    inbox: Vec<TransportFrame>,
}

impl Default for MemoryTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryTransport {
    pub fn new() -> Self {
        Self {
            state: TransportState::Disconnected,
            inbox: Vec::new(),
        }
    }

    pub fn inject_for_test(&mut self, frame: TransportFrame) -> Result<(), TransportError> {
        if frame.payload.len() > MAX_PAYLOAD_BYTES {
            return Err(TransportError::FrameTooLarge);
        }
        self.inbox.push(frame);
        Ok(())
    }
}

impl SecureTransport for MemoryTransport {
    fn state(&self) -> TransportState {
        self.state
    }

    fn connect(&mut self) -> Result<(), TransportError> {
        match self.state {
            TransportState::Disconnected => {
                self.state = TransportState::Connected;
                Ok(())
            }
            TransportState::Connected => Ok(()),
            TransportState::Connecting => Err(TransportError::NotConnected),
            TransportState::Closing => Err(TransportError::Closing),
            TransportState::Closed => Err(TransportError::Closed),
        }
    }

    fn send(&mut self, frame: TransportFrame) -> Result<(), TransportError> {
        match self.state {
            TransportState::Connected => self.inject_for_test(frame),
            TransportState::Closing => Err(TransportError::Closing),
            TransportState::Closed => Err(TransportError::Closed),
            TransportState::Disconnected | TransportState::Connecting => {
                Err(TransportError::NotConnected)
            }
        }
    }

    fn receive(&mut self) -> Result<Option<TransportFrame>, TransportError> {
        match self.state {
            TransportState::Connected => {
                if self.inbox.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(self.inbox.swap_remove(0)))
                }
            }
            TransportState::Closing => Err(TransportError::Closing),
            TransportState::Closed => Err(TransportError::Closed),
            TransportState::Disconnected | TransportState::Connecting => {
                Err(TransportError::NotConnected)
            }
        }
    }

    fn close(&mut self) -> Result<(), TransportError> {
        match self.state {
            TransportState::Connected => {
                self.state = TransportState::Closed;
                Ok(())
            }
            TransportState::Disconnected => {
                self.state = TransportState::Closed;
                Ok(())
            }
            TransportState::Connecting => Err(TransportError::NotConnected),
            TransportState::Closing => Err(TransportError::Closing),
            TransportState::Closed => Err(TransportError::Closed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> TransportFrame {
        TransportFrame::new(
            FrameHeader {
                protocol_version: 1,
                payload_len: 4,
                flags: 0,
            },
            vec![0xA5; 4],
        )
        .unwrap()
    }

    #[test]
    fn transport_requires_connection_before_send() {
        let mut transport = MemoryTransport::new();
        assert_eq!(transport.send(frame()), Err(TransportError::NotConnected));
    }

    #[test]
    fn connected_transport_round_trips_opaque_frame() {
        let mut transport = MemoryTransport::new();
        transport.connect().unwrap();

        let original = frame();
        transport.send(original.clone()).unwrap();

        assert_eq!(transport.receive().unwrap(), Some(original));
        assert_eq!(transport.receive().unwrap(), None);
    }

    #[test]
    fn close_blocks_future_transport_operations() {
        let mut transport = MemoryTransport::new();
        transport.connect().unwrap();
        transport.close().unwrap();

        assert_eq!(transport.state(), TransportState::Closed);
        assert_eq!(transport.send(frame()), Err(TransportError::Closed));
        assert_eq!(transport.receive(), Err(TransportError::Closed));
    }

    #[test]
    fn mismatched_payload_length_is_rejected_at_boundary() {
        let result = TransportFrame::new(
            FrameHeader {
                protocol_version: 1,
                payload_len: 8,
                flags: 0,
            },
            vec![0xA5; 4],
        );

        assert_eq!(
            result,
            Err(TransportError::Protocol(
                ProtocolError::PayloadLengthMismatch
            ))
        );
    }

    #[test]
    fn oversized_frame_is_rejected() {
        let header = FrameHeader {
            protocol_version: 1,
            payload_len: (MAX_PAYLOAD_BYTES + 1) as u32,
            flags: 0,
        };

        assert_eq!(
            result,
            Err(TransportError::Protocol(ProtocolError::PayloadTooLarge))
        );

        assert_eq!(
            result,
            Err(TransportError::Protocol(ProtocolError::PayloadTooLarge))
        );
    }
}
