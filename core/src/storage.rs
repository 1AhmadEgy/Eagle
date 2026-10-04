use crate::protocol::{OpaqueId, MAX_PAYLOAD_BYTES};

/// Stable upper bound for persisted application records.
pub const MAX_RECORD_BYTES: usize = MAX_PAYLOAD_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordId(pub [u8; 16]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaVersion(pub u16);

pub const CURRENT_SCHEMA_VERSION: SchemaVersion = SchemaVersion(1);

/// A persisted record contains only opaque/encrypted application bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedRecord {
    pub record_id: RecordId,
    pub schema_version: SchemaVersion,
    pub owner_id: OpaqueId,
    pub ciphertext: Vec<u8>,
    pub created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeletionReceipt {
    pub record_id: RecordId,
    pub schema_version: SchemaVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryState {
    Healthy,
    RecoveryRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    EmptyOwnerId,
    OwnerIdTooLarge,
    EmptyCiphertext,
    RecordTooLarge,
    UnsupportedSchema,
    NotFound,
    AlreadyExists,
    RecoveryRequired,
    RecoveryUnavailable,
    AlreadyDeleted,
}

pub trait RecordStore {
    fn put(&mut self, record: EncryptedRecord) -> Result<(), StorageError>;
    fn get(&self, record_id: RecordId) -> Result<Option<EncryptedRecord>, StorageError>;
    fn delete(&mut self, record_id: RecordId) -> Result<DeletionReceipt, StorageError>;
}

pub fn validate_record(record: &EncryptedRecord) -> Result<(), StorageError> {
    if record.owner_id.0.is_empty() {
        return Err(StorageError::EmptyOwnerId);
    }
    if record.owner_id.0.len() > crate::protocol::MAX_ID_BYTES {
        return Err(StorageError::OwnerIdTooLarge);
    }
    if record.ciphertext.is_empty() {
        return Err(StorageError::EmptyCiphertext);
    }
    if record.ciphertext.len() > MAX_RECORD_BYTES {
        return Err(StorageError::RecordTooLarge);
    }
    if record.schema_version.0 != CURRENT_SCHEMA_VERSION.0 {
        return Err(StorageError::UnsupportedSchema);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owner(byte: u8) -> OpaqueId {
        OpaqueId::new(vec![byte; 8]).unwrap()
    }

    fn record(id: u8) -> EncryptedRecord {
        EncryptedRecord {
            record_id: RecordId([id; 16]),
            schema_version: CURRENT_SCHEMA_VERSION,
            owner_id: owner(id),
            ciphertext: vec![0xAA; 32],
            created_at_epoch_ms: 1_760_000_000_000,
        }
    }

    #[test]
    fn protocol_error_mapping_remains_separate_from_storage_contract() {
        let id = OpaqueId::new(Vec::new());
        assert_eq!(id, Err(crate::protocol::ProtocolError::EmptyIdentifier));
    }

    #[test]
    fn invalid_record_is_rejected() {
        let mut item = record(1);
        item.ciphertext.clear();
        assert_eq!(validate_record(&item), Err(StorageError::EmptyCiphertext));
    }

    #[test]
    fn unsupported_schema_is_rejected() {
        let mut item = record(1);
        item.schema_version = SchemaVersion(CURRENT_SCHEMA_VERSION.0 + 1);
        assert_eq!(validate_record(&item), Err(StorageError::UnsupportedSchema));
    }
}
