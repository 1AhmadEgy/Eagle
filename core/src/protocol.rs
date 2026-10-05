use std::fmt;

pub const CURRENT_PROTOCOL_VERSION: u16 = 1;
pub const MAX_PAYLOAD_BYTES: usize = 1024 * 1024;
pub const MAX_ID_BYTES: usize = 128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageId([u8; 16]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpaqueId(Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedEnvelope {
    message_id: MessageId,
    conversation_id: OpaqueId,
    sender_device_id: OpaqueId,
    recipient_device_id: Option<OpaqueId>,
    ciphertext: Vec<u8>,
    protocol_version: u16,
    created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameHeader {
    protocol_version: u16,
    payload_len: u32,
    flags: u16,
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
    UnsupportedFlags,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedVersion => "unsupported protocol version",
            Self::DowngradeRejected => "protocol downgrade rejected",
            Self::EmptyIdentifier => "empty identifier",
            Self::IdentifierTooLarge => "identifier exceeds maximum length",
            Self::EmptyCiphertext => "empty ciphertext",
            Self::PayloadTooLarge => "payload exceeds maximum size",
            Self::PayloadLengthMismatch => "payload length mismatch",
            Self::UnsupportedFlags => "unsupported frame flags",
        })
    }
}

impl std::error::Error for ProtocolError {}

impl MessageId {
    pub fn new(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl OpaqueId {
    pub fn new(bytes: Vec<u8>) -> Result<Self, ProtocolError> {
        match bytes.len() {
            0 => Err(ProtocolError::EmptyIdentifier),
            n if n > MAX_ID_BYTES => Err(ProtocolError::IdentifierTooLarge),
            _ => Ok(Self(bytes)),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl EncryptedEnvelope {
    pub fn new(
        message_id: MessageId,
        conversation_id: OpaqueId,
        sender_device_id: OpaqueId,
        recipient_device_id: Option<OpaqueId>,
        ciphertext: Vec<u8>,
        protocol_version: u16,
        created_at_epoch_ms: u64,
    ) -> Result<Self, ProtocolError> {
        let envelope = Self {
            message_id,
            conversation_id,
            sender_device_id,
            recipient_device_id,
            ciphertext,
            protocol_version,
            created_at_epoch_ms,
        };
        envelope.validate(CURRENT_PROTOCOL_VERSION)?;
        Ok(envelope)
    }

    pub fn message_id(&self) -> &MessageId {
        &self.message_id
    }

    pub fn conversation_id(&self) -> &OpaqueId {
        &self.conversation_id
    }

    pub fn sender_device_id(&self) -> &OpaqueId {
        &self.sender_device_id
    }

    pub fn recipient_device_id(&self) -> Option<&OpaqueId> {
        self.recipient_device_id.as_ref()
    }

    pub fn ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    pub fn protocol_version(&self) -> u16 {
        self.protocol_version
    }

    pub fn created_at_epoch_ms(&self) -> u64 {
        self.created_at_epoch_ms
    }

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

    pub fn frame_header(&self) -> Result<FrameHeader, ProtocolError> {
        self.validate(CURRENT_PROTOCOL_VERSION)?;
        let payload_len =
            u32::try_from(self.ciphertext.len()).map_err(|_| ProtocolError::PayloadTooLarge)?;
        FrameHeader::new(self.protocol_version, payload_len, 0)
    }
}

impl FrameHeader {
    pub fn new(protocol_version: u16, payload_len: u32, flags: u16) -> Result<Self, ProtocolError> {
        validate_version(protocol_version, CURRENT_PROTOCOL_VERSION)?;
        let payload_len =
            usize::try_from(payload_len).map_err(|_| ProtocolError::PayloadTooLarge)?;
        if payload_len > MAX_PAYLOAD_BYTES {
            return Err(ProtocolError::PayloadTooLarge);
        }
        if flags != 0 {
            return Err(ProtocolError::UnsupportedFlags);
        }

        Ok(Self {
            protocol_version,
            payload_len: u32::try_from(payload_len).map_err(|_| ProtocolError::PayloadTooLarge)?,
            flags,
        })
    }

    pub fn protocol_version(&self) -> u16 {
        self.protocol_version
    }

    pub fn payload_len(&self) -> u32 {
        self.payload_len
    }

    pub fn flags(&self) -> u16 {
        self.flags
    }

    pub fn validate_version(&self, minimum_version: u16) -> Result<u16, ProtocolError> {
        validate_version(self.protocol_version, minimum_version)
    }

    pub fn validate_payload_len(&self, actual_len: usize) -> Result<(), ProtocolError> {
        if actual_len > MAX_PAYLOAD_BYTES {
            return Err(ProtocolError::PayloadTooLarge);
        }
        if usize::try_from(self.payload_len).ok() != Some(actual_len) {
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


    #[test]
    fn identifier_bounds_are_enforced_at_construction() {
        assert_eq!(
            OpaqueId::new(Vec::new()),
            Err(ProtocolError::EmptyIdentifier)
        );
        assert_eq!(
            OpaqueId::new(vec![0; MAX_ID_BYTES + 1]),
            Err(ProtocolError::IdentifierTooLarge)
        );
        assert_eq!(id(7).as_bytes().len(), 8);
    }

    #[test]
    fn structural_validation_is_fail_closed() {
        let valid = EncryptedEnvelope::new(
            MessageId::new([1; 16]),
            id(2),
            id(3),
            Some(id(4)),
            vec![0xAA; 32],
            1,
            1,
        )
        .unwrap();
        assert_eq!(valid.validate(1), Ok(()));
        assert_eq!(valid.message_id().as_bytes(), &[1; 16]);
        assert_eq!(
            EncryptedEnvelope::new(
                MessageId::new([1; 16]),
                id(2),
                id(3),
                Some(id(4)),
                Vec::new(),
                1,
                1
            ),
            Err(ProtocolError::EmptyCiphertext)
        );
        assert_eq!(
            EncryptedEnvelope::new(
                MessageId::new([1; 16]),
                id(2),
                id(3),
                Some(id(4)),
                vec![0xAA; MAX_PAYLOAD_BYTES + 1],
                1,
                1
            ),
            Err(ProtocolError::PayloadTooLarge)
        );
    }

    #[test]
    fn structural_validation_rejects_versions() {
        assert_eq!(
            validate_version(0, 1),
            Err(ProtocolError::DowngradeRejected)
        );
        assert_eq!(
            validate_version(2, 1),
            Err(ProtocolError::UnsupportedVersion)
        );
    }

    #[test]
    fn payload_length_is_exact_and_header_version_is_checked() {
        let header = FrameHeader {
            protocol_version: 1,
            payload_len: 8,
            flags: 0,
        };
        assert_eq!(header.validate_version(1), Ok(1));
        assert_eq!(
            header.validate_payload_len(7),
            Err(ProtocolError::PayloadLengthMismatch)
        );
        assert_eq!(header.validate_payload_len(8), Ok(()));
        assert_eq!(
            (FrameHeader {
                protocol_version: 0,
                payload_len: 8,
                flags: 0
            })
            .validate_version(1),
            Err(ProtocolError::DowngradeRejected)
        );
    }

    #[test]
    fn envelope_frame_header_is_consistent() {
        let envelope = EncryptedEnvelope::new(
            MessageId::new([7; 16]),
            id(2),
            id(3),
            None,
            vec![0x11; 64],
            1,
            42,
        )
        .unwrap();
        let header = envelope.frame_header().unwrap();
        assert_eq!(header.protocol_version(), 1);
        assert_eq!(header.payload_len(), 64);
        assert_eq!(header.flags(), 0);
    }
}
