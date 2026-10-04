use std::fmt;

pub const CURRENT_PROTOCOL_VERSION: u16 = 1;
pub const MAX_PAYLOAD_BYTES: usize = 1024 * 1024;
pub const MAX_ID_BYTES: usize = 128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageId(pub [u8; 16]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpaqueId(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedEnvelope {
    pub message_id: MessageId,
    pub conversation_id: OpaqueId,
    pub sender_device_id: OpaqueId,
    pub recipient_device_id: Option<OpaqueId>,
    pub ciphertext: Vec<u8>,
    pub protocol_version: u16,
    pub created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameHeader {
    pub protocol_version: u16,
    pub payload_len: u32,
    pub flags: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    UnsupportedVersion,
    DowngradeRejected,
    EmptyIdentifier,
    IdentifierTooLarge,
    EmptyCiphertext,
    PayloadTooLarge,
    PayloadLengthMismatch,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedVersion => "unsupported protocol version",
            Self::DowngradeRejected => "protocol downgrade rejected",
            Self::EmptyIdentifier => "identifier is empty",
            Self::IdentifierTooLarge => "identifier exceeds maximum length",
            Self::EmptyCiphertext => "ciphertext is empty",
            Self::PayloadTooLarge => "payload exceeds maximum size",
            Self::PayloadLengthMismatch => "payload length does not match frame header",
        })
    }
}

impl OpaqueId {
    pub fn new(bytes: Vec<u8>) -> Result<Self, ProtocolError> {
        if bytes.is_empty() {
            return Err(ProtocolError::EmptyIdentifier);
        }
        if bytes.len() > MAX_ID_BYTES {
            return Err(ProtocolError::IdentifierTooLarge);
        }
        Ok(Self(bytes))
    }
}

impl EncryptedEnvelope {
    pub fn validate(&self, minimum_version: u16) -> Result<(), ProtocolError> {
        validate_version(self.protocol_version, minimum_version)?;
        validate_id(&self.conversation_id)?;
        validate_id(&self.sender_device_id)?;

        if let Some(recipient) = &self.recipient_device_id {
            validate_id(recipient)?;
        }

        if self.ciphertext.is_empty() {
            return Err(ProtocolError::EmptyCiphertext);
        }
        if self.ciphertext.len() > MAX_PAYLOAD_BYTES {
            return Err(ProtocolError::PayloadTooLarge);
        }

        Ok(())
    }

    pub fn frame_header(&self, flags: u16) -> Result<FrameHeader, ProtocolError> {
        self.validate(CURRENT_PROTOCOL_VERSION)?;
        let payload_len =
            u32::try_from(self.ciphertext.len()).map_err(|_| ProtocolError::PayloadTooLarge)?;

        Ok(FrameHeader {
            protocol_version: self.protocol_version,
            payload_len,
            flags,
        })
    }
}

impl FrameHeader {
    pub fn validate_payload_len(&self, actual_len: usize) -> Result<(), ProtocolError> {
        if actual_len > MAX_PAYLOAD_BYTES {
            return Err(ProtocolError::PayloadTooLarge);
        }
        if self.payload_len as usize != actual_len {
            return Err(ProtocolError::PayloadLengthMismatch);
        }
        Ok(())
    }
}

pub fn validate_version(offered: u16, minimum_version: u16) -> Result<u16, ProtocolError> {
    if offered < minimum_version || offered < CURRENT_PROTOCOL_VERSION {
        return Err(ProtocolError::DowngradeRejected);
    }
    if offered > CURRENT_PROTOCOL_VERSION {
        return Err(ProtocolError::UnsupportedVersion);
    }
    Ok(offered)
}

fn validate_id(id: &OpaqueId) -> Result<(), ProtocolError> {
    if id.0.is_empty() {
        return Err(ProtocolError::EmptyIdentifier);
    }
    if id.0.len() > MAX_ID_BYTES {
        return Err(ProtocolError::IdentifierTooLarge);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u8) -> OpaqueId {
        OpaqueId::new(vec![value; 8]).unwrap()
    }

    fn envelope(version: u16, payload_len: usize) -> EncryptedEnvelope {
        EncryptedEnvelope {
            message_id: MessageId([7; 16]),
            conversation_id: id(1),
            sender_device_id: id(2),
            recipient_device_id: Some(id(3)),
            ciphertext: vec![0xAA; payload_len],
            protocol_version: version,
            created_at_epoch_ms: 1_760_000_000_000,
        }
    }

    #[test]
    fn valid_envelope_passes_validation() {
        assert_eq!(envelope(1, 32).validate(1), Ok(()));
    }

    #[test]
    fn downgrade_is_rejected() {
        assert_eq!(
            validate_version(0, 1),
            Err(ProtocolError::DowngradeRejected)
        );
    }

    #[test]
    fn future_version_is_rejected_until_adopted() {
        assert_eq!(
            validate_version(2, 1),
            Err(ProtocolError::UnsupportedVersion)
        );
    }

    #[test]
    fn oversized_identifier_is_rejected() {
        assert_eq!(
            OpaqueId::new(vec![1; MAX_ID_BYTES + 1]),
            Err(ProtocolError::IdentifierTooLarge)
        );
    }

    #[test]
    fn empty_ciphertext_is_rejected() {
        assert_eq!(
            envelope(1, 0).validate(1),
            Err(ProtocolError::EmptyCiphertext)
        );
    }

    #[test]
    fn oversized_payload_is_rejected() {
        assert_eq!(
            envelope(1, MAX_PAYLOAD_BYTES + 1).validate(1),
            Err(ProtocolError::PayloadTooLarge)
        );
    }

    #[test]
    fn frame_header_detects_payload_length_mismatch() {
        let header = FrameHeader {
            protocol_version: 1,
            payload_len: 10,
            flags: 0,
        };
        assert_eq!(
            header.validate_payload_len(9),
            Err(ProtocolError::PayloadLengthMismatch)
        );
        assert_eq!(header.validate_payload_len(10), Ok(()));
    }
}
