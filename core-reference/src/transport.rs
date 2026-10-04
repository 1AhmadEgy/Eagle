use eagle_core::protocol::{FrameHeader, ProtocolError, MAX_PAYLOAD_BYTES};
use eagle_core::transport::{SecureTransport, TransportError, TransportFrame, TransportState};

/// Deterministic in-memory transport adapter for contract tests.
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
            TransportState::Connected | TransportState::Disconnected => {
                self.state = TransportState::Closed;
                Ok(())
            }
            TransportState::Connecting => Err(TransportError::NotConnected),
            TransportState::Closing => Err(TransportError::Closing),
            TransportState::Closed => Err(TransportError::Closed),
        }
    }
}
