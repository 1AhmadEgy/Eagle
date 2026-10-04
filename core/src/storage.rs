use crate::protocol::{OpaqueId, ProtocolError, MAX_PAYLOAD_BYTES};

/// Stable upper bound for persisted application records.
///
/// This is a contract limit only; it does not define a database page size,
/// serialization format, or encryption algorithm.
pub const MAX_RECORD_BYTES: usize = MAX_PAYLOAD_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordId(pub [u8; 16]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaVersion(pub u16);

pub const CURRENT_SCHEMA_VERSION: SchemaVersion = SchemaVersion(1);

/// A persisted record contains only opaque/encrypted application bytes.
///
/// Private keys and live cryptographic key handles are intentionally not
/// representable by this contract.
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

/// Minimal deterministic in-memory implementation for contract tests.
///
/// This is not a production persistence engine and intentionally has no key
/// storage, backup, serialization, or platform-specific behavior.
#[derive(Debug)]
pub struct MemoryRecordStore {
    records: Vec<EncryptedRecord>,
    recovery: RecoveryState,
}

impl Default for MemoryRecordStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryRecordStore {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            recovery: RecoveryState::Healthy,
        }
    }

    pub fn recovery_state(&self) -> RecoveryState {
        self.recovery
    }

    pub fn require_recovery(&mut self) {
        self.recovery = RecoveryState::RecoveryRequired;
    }

    pub fn mark_recovered(&mut self) {
        self.recovery = RecoveryState::Healthy;
    }

    pub fn mark_unavailable(&mut self) {
        self.recovery = RecoveryState::Unavailable;
    }
}

impl RecordStore for MemoryRecordStore {
    fn put(&mut self, record: EncryptedRecord) -> Result<(), StorageError> {
        validate_record(&record)?;

        if self.recovery != RecoveryState::Healthy {
            return Err(StorageError::RecoveryRequired);
        }

        if self
            .records
            .iter()
            .any(|item| item.record_id == record.record_id)
        {
            return Err(StorageError::AlreadyExists);
        }

        self.records.push(record);
        Ok(())
    }

    fn get(&self, record_id: RecordId) -> Result<Option<EncryptedRecord>, StorageError> {
        if self.recovery == RecoveryState::Unavailable {
            return Err(StorageError::RecoveryUnavailable);
        }

        Ok(self
            .records
            .iter()
            .find(|item| item.record_id == record_id)
            .cloned())
    }

    fn delete(&mut self, record_id: RecordId) -> Result<DeletionReceipt, StorageError> {
        if self.recovery == RecoveryState::Unavailable {
            return Err(StorageError::RecoveryUnavailable);
        }

        let index = self
            .records
            .iter()
            .position(|item| item.record_id == record_id)
            .ok_or(StorageError::NotFound)?;

        let removed = self.records.swap_remove(index);
        Ok(DeletionReceipt {
            record_id: removed.record_id,
            schema_version: removed.schema_version,
        })
    }
}

fn validate_record(record: &EncryptedRecord) -> Result<(), StorageError> {
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
    fn encrypted_record_can_be_stored_and_loaded() {
        let mut store = MemoryRecordStore::new();
        let item = record(1);

        store.put(item.clone()).unwrap();

        assert_eq!(store.get(item.record_id).unwrap(), Some(item));
    }

    #[test]
    fn duplicate_record_is_rejected() {
        let mut store = MemoryRecordStore::new();
        let item = record(1);

        store.put(item.clone()).unwrap();

        assert_eq!(store.put(item), Err(StorageError::AlreadyExists));
    }

    #[test]
    fn deletion_returns_receipt_and_removes_record() {
        let mut store = MemoryRecordStore::new();
        let item = record(1);
        store.put(item.clone()).unwrap();

        let receipt = store.delete(item.record_id).unwrap();

        assert_eq!(
            receipt,
            DeletionReceipt {
                record_id: item.record_id,
                schema_version: CURRENT_SCHEMA_VERSION
            }
        );
        assert_eq!(store.get(item.record_id).unwrap(), None);
    }

    #[test]
    fn empty_ciphertext_is_rejected() {
        let mut store = MemoryRecordStore::new();
        let mut item = record(1);
        item.ciphertext.clear();

        assert_eq!(store.put(item), Err(StorageError::EmptyCiphertext));
    }

    #[test]
    fn recovery_required_fails_closed_for_new_writes() {
        let mut store = MemoryRecordStore::new();
        store.require_recovery();

        assert_eq!(store.put(record(1)), Err(StorageError::RecoveryRequired));
        assert_eq!(store.recovery_state(), RecoveryState::RecoveryRequired);

        store.mark_recovered();
        assert_eq!(store.put(record(1)), Ok(()));
    }

    #[test]
    fn unavailable_storage_fails_closed() {
        let mut store = MemoryRecordStore::new();
        store.put(record(1)).unwrap();
        store.mark_unavailable();

        assert_eq!(
            store.get(RecordId([1; 16])),
            Err(StorageError::RecoveryUnavailable)
        );
        assert_eq!(
            store.delete(RecordId([1; 16])),
            Err(StorageError::RecoveryUnavailable)
        );
    }

    #[test]
    fn unsupported_schema_is_rejected_without_persistence() {
        let mut store = MemoryRecordStore::new();
        let mut item = record(1);
        item.schema_version = SchemaVersion(CURRENT_SCHEMA_VERSION.0 + 1);

        assert_eq!(store.put(item), Err(StorageError::UnsupportedSchema));
        assert_eq!(store.get(RecordId([1; 16])).unwrap(), None);
    }

    #[test]
    fn protocol_error_mapping_remains_separate_from_storage_contract() {
        let id = OpaqueId::new(Vec::new());
        assert_eq!(id, Err(ProtocolError::EmptyIdentifier));
    }
}
