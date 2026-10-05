use alloc::vec::Vec;

use minicbor::{
    data::Type,
    decode::Decoder,
    encode::{Encoder, Error as EncodeError, Write},
    Encode,
};

use crate::{
    EncryptedEnvelope, MessageId, OpaqueId, ProtocolError, CURRENT_PROTOCOL_VERSION,
    MAX_ID_BYTES, MAX_PAYLOAD_BYTES,
};

pub const SERIALIZED_ENVELOPE_FIELD_COUNT: u64 = 7;
pub const MAX_SERIALIZED_ENVELOPE_BYTES: usize = MAX_PAYLOAD_BYTES + 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationError {
    InputTooLarge,
    InvalidShape,
    DecodeError,
    EncodeError,
    InvalidEnvelope,
    NonCanonical,
    TrailingData,
}

impl From<ProtocolError> for SerializationError {
    fn from(_: ProtocolError) -> Self {
        Self::InvalidEnvelope
    }
}

impl Encode<()> for EncryptedEnvelope {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _: &mut (),
    ) -> Result<(), EncodeError<W::Error>> {
        e.array(SERIALIZED_ENVELOPE_FIELD_COUNT)?;
        e.bytes(self.message_id().as_bytes())?;
        e.bytes(self.conversation_id().as_bytes())?;
        e.bytes(self.sender_device_id().as_bytes())?;
        match self.recipient_device_id() {
            Some(recipient) => e.bytes(recipient.as_bytes())?,
            None => e.null()?,
        };
        e.bytes(self.ciphertext())?;
        e.u16(self.protocol_version())?;
        e.u64(self.created_at_epoch_ms())?;
        Ok(())
    }
}

pub fn serialize_envelope(envelope: &EncryptedEnvelope) -> Result<Vec<u8>, SerializationError> {
    envelope.validate(CURRENT_PROTOCOL_VERSION)?;
    let bytes = minicbor::to_vec(envelope).map_err(|_| SerializationError::EncodeError)?;
    if bytes.len() > MAX_SERIALIZED_ENVELOPE_BYTES {
        return Err(SerializationError::InputTooLarge);
    }
    Ok(bytes)
}

pub fn deserialize_envelope(bytes: &[u8]) -> Result<EncryptedEnvelope, SerializationError> {
    if bytes.is_empty() || bytes.len() > MAX_SERIALIZED_ENVELOPE_BYTES {
        return Err(SerializationError::InputTooLarge);
    }

    let mut decoder = Decoder::new(bytes);
    let array_len = decoder
        .array()
        .map_err(|_| SerializationError::DecodeError)?;
    if array_len != Some(SERIALIZED_ENVELOPE_FIELD_COUNT) {
        return Err(SerializationError::InvalidShape);
    }

    let message_id_slice = decoder
        .bytes()
        .map_err(|_| SerializationError::DecodeError)?;
    let message_id: [u8; 16] = message_id_slice
        .try_into()
        .map_err(|_| SerializationError::InvalidEnvelope)?;

    let conversation_bytes = decoder
        .bytes()
        .map_err(|_| SerializationError::DecodeError)?;
    let conversation_id = OpaqueId::new(conversation_bytes.to_vec())?;

    let sender_bytes = decoder
        .bytes()
        .map_err(|_| SerializationError::DecodeError)?;
    let sender_device_id = OpaqueId::new(sender_bytes.to_vec())?;

    let recipient_device_id = if decoder.datatype().map_err(|_| SerializationError::DecodeError)?
        == Type::Null
    {
        decoder
            .null()
            .map_err(|_| SerializationError::DecodeError)?;
        None
    } else {
        Some(OpaqueId::new(
            decoder
                .bytes()
                .map_err(|_| SerializationError::DecodeError)?
                .to_vec(),
        )?)
    };

    let ciphertext = decoder
        .bytes()
        .map_err(|_| SerializationError::DecodeError)?
        .to_vec();
    if ciphertext.len() > MAX_PAYLOAD_BYTES || ciphertext.is_empty() {
        return Err(SerializationError::InvalidEnvelope);
    }

    let protocol_version = decoder
        .u16()
        .map_err(|_| SerializationError::DecodeError)?;
    let created_at_epoch_ms = decoder
        .u64()
        .map_err(|_| SerializationError::DecodeError)?;

    if decoder.position() != bytes.len() {
        return Err(SerializationError::TrailingData);
    }

    let envelope = EncryptedEnvelope::new(
        MessageId::new(message_id),
        conversation_id,
        sender_device_id,
        recipient_device_id,
        ciphertext,
        protocol_version,
        created_at_epoch_ms,
    )?;

    let canonical = serialize_envelope(&envelope)?;
    if canonical.as_slice() != bytes {
        return Err(SerializationError::NonCanonical);
    }

    Ok(envelope)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u8) -> OpaqueId {
        OpaqueId::new(vec![value; 8]).unwrap()
    }

    fn envelope() -> EncryptedEnvelope {
        EncryptedEnvelope::new(
            MessageId::new([1; 16]),
            id(2),
            id(3),
            None,
            vec![0xAA; 32],
            CURRENT_PROTOCOL_VERSION,
            42,
        )
        .unwrap()
    }

    #[test]
    fn round_trip_is_exact() {
        let original = envelope();
        let bytes = serialize_envelope(&original).unwrap();
        let decoded = deserialize_envelope(&bytes).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn canonical_encoding_is_stable() {
        let bytes = serialize_envelope(&envelope()).unwrap();
        let mut expected = vec![
            0x87, 0x50, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0x48, 2, 2,
            2, 2, 2, 2, 2, 2, 0x48, 3, 3, 3, 3, 3, 3, 3, 3, 0xf6, 0x58, 0x20,
        ];
        expected.extend([0xAA; 32]);
        expected.extend([0x01, 0x18, 0x2a]);

        assert_eq!(bytes, expected);
    }

    #[test]
    fn non_canonical_integer_encoding_is_rejected() {
        let mut bytes = serialize_envelope(&envelope()).unwrap();
        let protocol_index = bytes.len() - 3;
        bytes.splice(protocol_index..protocol_index + 1, [0x19, 0x00]);
        assert_eq!(
            deserialize_envelope(&bytes),
            Err(SerializationError::NonCanonical)
        );
    }

    #[test]
    fn indefinite_array_and_trailing_data_are_rejected() {
        let canonical = serialize_envelope(&envelope()).unwrap();
        let mut indefinite = canonical.clone();
        indefinite[0] = 0x9f;
        indefinite.insert(indefinite.len() - 1, 0xff);
        assert_eq!(
            deserialize_envelope(&indefinite),
            Err(SerializationError::InvalidShape)
        );

        let mut trailing = canonical;
        trailing.push(0x00);
        assert_eq!(
            deserialize_envelope(&trailing),
            Err(SerializationError::TrailingData)
        );
    }

    #[test]
    fn identifiers_and_payload_remain_bounded() {
        let mut too_large_id = envelope();
        too_large_id = EncryptedEnvelope::new(
            MessageId::new([1; 16]),
            OpaqueId::new(vec![2; MAX_ID_BYTES]).unwrap(),
            id(3),
            None,
            vec![0xAA; 32],
            CURRENT_PROTOCOL_VERSION,
            42,
        )
        .unwrap();
        assert!(serialize_envelope(&too_large_id).is_ok());
    }
}
