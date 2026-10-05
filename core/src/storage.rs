use crate::OpaqueId;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageState {
    Healthy,
    RecoveryRequired,
    Unavailable,
}

impl Default for StorageState {
    fn default() -> Self {
        Self::Healthy
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedRecord {
    record_id: OpaqueId,
    owner_id: OpaqueId,
    schema_version: u16,
    ciphertext: Vec<u8>,
    created_at_epoch_ms: u64,
}

impl EncryptedRecord {
    pub fn new(
        record_id: OpaqueId,
        owner_id: OpaqueId,
        schema_version: u16,
        ciphertext: Vec<u8>,
        created_at_epoch_ms: u64,
    ) -> Result<Self, StorageError> {
        if schema_version == 0
            || ciphertext.is_empty()
            || ciphertext.len() > crate::MAX_PAYLOAD_BYTES
        {
            return Err(StorageError::InvalidRecord);
        }

        Ok(Self {
            record_id,
            owner_id,
            schema_version,
            ciphertext,
            created_at_epoch_ms,
        })
    }

    pub fn record_id(&self) -> &OpaqueId {
        &self.record_id
    }

    pub fn owner_id(&self) -> &OpaqueId {
        &self.owner_id
    }

    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    pub fn created_at_epoch_ms(&self) -> u64 {
        self.created_at_epoch_ms
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteReceipt {
    record_id: OpaqueId,
    schema_version: u16,
}

impl DeleteReceipt {
    pub fn record_id(&self) -> &OpaqueId {
        &self.record_id
    }

    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    InvalidRecord,
    RecoveryRequired,
    Unavailable,
    NotFound,
    AccessDenied,
}

pub trait SecureStorage {
    fn state(&self) -> StorageState;
    fn put(&mut self, record: EncryptedRecord) -> Result<(), StorageError>;
    fn get(
        &self,
        record_id: &OpaqueId,
        owner_id: &OpaqueId,
    ) -> Result<EncryptedRecord, StorageError>;
    fn delete(
        &mut self,
        record_id: &OpaqueId,
        owner_id: &OpaqueId,
    ) -> Result<DeleteReceipt, StorageError>;
    fn recover(&mut self) -> Result<(), StorageError>;
}

#[derive(Debug, Default)]
pub struct InMemorySecureStorage {
    state: StorageState,
    records: BTreeMap<Vec<u8>, EncryptedRecord>,
}

impl InMemorySecureStorage {
    pub fn new() -> Self {
        Self {
            state: StorageState::Healthy,
            records: BTreeMap::new(),
        }
    }

    pub fn set_recovery_required(&mut self) {
        self.state = StorageState::RecoveryRequired;
    }

    pub fn set_unavailable(&mut self) {
        self.state = StorageState::Unavailable;
    }
}

impl SecureStorage for InMemorySecureStorage {
    fn state(&self) -> StorageState {
        self.state
    }

    fn put(&mut self, record: EncryptedRecord) -> Result<(), StorageError> {
        match self.state {
            StorageState::Healthy => {}
            StorageState::RecoveryRequired => return Err(StorageError::RecoveryRequired),
            StorageState::Unavailable => return Err(StorageError::Unavailable),
        }

        let key = record.record_id().as_bytes().to_vec();
        if let Some(existing) = self.records.get(&key) {
            if existing.owner_id() != record.owner_id() {
                return Err(StorageError::AccessDenied);
            }
        }

        self.records.insert(key, record);
        Ok(())
    }

    fn get(
        &self,
        record_id: &OpaqueId,
        owner_id: &OpaqueId,
    ) -> Result<EncryptedRecord, StorageError> {
        match self.state {
            StorageState::Healthy => {}
            StorageState::RecoveryRequired => return Err(StorageError::RecoveryRequired),
            StorageState::Unavailable => return Err(StorageError::Unavailable),
        }

        let record = self.records.get(record_id.as_bytes()).ok_or(StorageError::NotFound)?;
        if record.owner_id() != owner_id {
            return Err(StorageError::NotFound);
        }
        Ok(record.clone())
    }

    fn delete(
        &mut self,
        record_id: &OpaqueId,
        owner_id: &OpaqueId,
    ) -> Result<DeleteReceipt, StorageError> {
        match self.state {
            StorageState::Healthy => {}
            StorageState::RecoveryRequired => return Err(StorageError::RecoveryRequired),
            StorageState::Unavailable => return Err(StorageError::Unavailable),
        }

        let record = self.records.get(record_id.as_bytes()).ok_or(StorageError::NotFound)?;
        if record.owner_id() != owner_id {
            return Err(StorageError::NotFound);
        }
        let schema_version = record.schema_version();
        self.records.remove(record_id.as_bytes());
        Ok(DeleteReceipt {
            record_id: record_id.clone(),
            schema_version,
        })
    }

    fn recover(&mut self) -> Result<(), StorageError> {
        if self.state == StorageState::Unavailable {
            return Err(StorageError::Unavailable);
        }
        self.state = StorageState::Healthy;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u8) -> OpaqueId {
        OpaqueId::new(vec![value; 8]).unwrap()
    }

    fn record() -> EncryptedRecord {
        EncryptedRecord::new(id(1), id(2), 1, vec![0xAA; 16], 7).unwrap()
    }

    #[test]
    fn invalid_records_are_rejected() {
        assert_eq!(
            EncryptedRecord::new(id(1), id(2), 0, vec![1], 0),
            Err(StorageError::InvalidRecord)
        );
        assert_eq!(
            EncryptedRecord::new(id(1), id(2), 1, Vec::new(), 0),
            Err(StorageError::InvalidRecord)
        );
    }

    #[test]
    fn owner_binding_is_enforced() {
        let mut store = InMemorySecureStorage::new();
        store.put(record()).unwrap();
        assert_eq!(store.get(&id(1), &id(3)), Err(StorageError::NotFound));

        let duplicate = EncryptedRecord::new(id(1), id(3), 1, vec![0xBB; 16], 8).unwrap();
        assert_eq!(store.put(duplicate), Err(StorageError::AccessDenied));
    }

    #[test]
    fn recovery_required_blocks_mutation_and_reads() {
        let mut store = InMemorySecureStorage::new();
        store.put(record()).unwrap();
        let receipt = store.delete(&id(1), &id(2)).unwrap();
        assert_eq!(receipt.record_id(), &id(1));
        assert_eq!(receipt.schema_version(), 1);

        store.put(record()).unwrap();
        store.set_recovery_required();

        assert_eq!(store.state(), StorageState::RecoveryRequired);
        assert_eq!(store.put(record()), Err(StorageError::RecoveryRequired));
        assert_eq!(
            store.get(&id(1), &id(2)),
            Err(StorageError::RecoveryRequired)
        );
        assert_eq!(
            store.delete(&id(1), &id(2)),
            Err(StorageError::RecoveryRequired)
        );

        store.recover().unwrap();
        assert_eq!(store.state(), StorageState::Healthy);
        assert!(store.get(&id(1), &id(2)).is_ok());
    }

    #[test]
    fn unavailable_fails_closed() {
        let mut store = InMemorySecureStorage::new();
        store.set_unavailable();

        assert_eq!(store.put(record()), Err(StorageError::Unavailable));
        assert_eq!(
            store.get(&id(1), &id(2)),
            Err(StorageError::Unavailable)
        );
        assert_eq!(
            store.delete(&id(1), &id(2)),
            Err(StorageError::Unavailable)
        );
        assert_eq!(store.recover(), Err(StorageError::Unavailable));
    }
}
