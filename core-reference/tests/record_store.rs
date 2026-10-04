use eagle_core::protocol::OpaqueId;
use eagle_core::storage::{
    EncryptedRecord, RecordId, RecordStore, RecoveryState, SchemaVersion, StorageError,
    CURRENT_SCHEMA_VERSION,
};
use eagle_core_reference::record_store::MemoryRecordStore;

fn owner(byte: u8) -> OpaqueId { OpaqueId::new(vec![byte; 8]).unwrap() }

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
    assert_eq!(receipt, eagle_core::storage::DeletionReceipt {
        record_id: item.record_id,
        schema_version: CURRENT_SCHEMA_VERSION
    });
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
    assert_eq!(store.get(RecordId([1; 16])), Err(StorageError::RecoveryUnavailable));
    assert_eq!(store.delete(RecordId([1; 16])), Err(StorageError::RecoveryUnavailable));
}

#[test]
fn unsupported_schema_is_rejected_without_persistence() {
    let mut store = MemoryRecordStore::new();
    let mut item = record(1);
    item.schema_version = SchemaVersion(CURRENT_SCHEMA_VERSION.0 + 1);
    assert_eq!(store.put(item), Err(StorageError::UnsupportedSchema));
    assert_eq!(store.get(RecordId([1; 16])).unwrap(), None);
}
