use eagle_core::protocol::{FrameHeader, ProtocolError, MAX_PAYLOAD_BYTES};
use eagle_core::transport::{SecureTransport, TransportError, TransportFrame, TransportState};
use eagle_core_reference::transport::MemoryTransport;

fn frame() -> TransportFrame {
    TransportFrame::new(
        FrameHeader { protocol_version: 1, payload_len: 4, flags: 0 },
        vec![0xA5; 4],
    ).unwrap()
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
    assert_eq!(transport.connect(), Err(TransportError::Closed));
}

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
