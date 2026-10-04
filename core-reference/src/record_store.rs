use eagle_core::protocol::{OpaqueId, MAX_ID_BYTES};
use eagle_core::storage::{
    DeletionReceipt, EncryptedRecord, RecordId, RecordStore, RecoveryState, SchemaVersion,
    StorageError, CURRENT_SCHEMA_VERSION, MAX_RECORD_BYTES,
};

/// Deterministic in-memory implementation of the storage contract for tests.
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
    if record.owner_id.0.len() > MAX_ID_BYTES {
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
