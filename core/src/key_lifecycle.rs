//! Deterministic key-lifecycle state machine.
//!
//! This module stores only key metadata/handles. It never stores private key bytes.
//! The purpose is to make rotation, revocation, destruction and rollback resistance
//! explicit before a real cryptographic provider is introduced.

use crate::{KeyError, KeyId, KeyPurpose};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Active,
    Revoked,
    Destroyed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyRecord {
    id: KeyId,
    purpose: KeyPurpose,
    generation: u64,
    epoch: u64,
    state: LifecycleState,
}

impl KeyRecord {
    pub const fn new(id: KeyId, purpose: KeyPurpose, generation: u64) -> Self {
        Self {
            id,
            purpose,
            generation,
            epoch: 0,
            state: LifecycleState::Active,
        }
    }

    pub const fn id(&self) -> KeyId { self.id }
    pub const fn purpose(&self) -> KeyPurpose { self.purpose }
    pub const fn generation(&self) -> u64 { self.generation }
    pub const fn epoch(&self) -> u64 { self.epoch }
    pub const fn state(&self) -> LifecycleState { self.state }

    pub const fn is_active(&self) -> bool {
        matches!(self.state, LifecycleState::Active)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleError {
    MissingKey,
    PurposeMismatch,
    GenerationRollback,
    InvalidTransition,
    Destroyed,
    Revoked,
}

impl From<LifecycleError> for KeyError {
    fn from(value: LifecycleError) -> Self {
        match value {
            LifecycleError::Destroyed => KeyError::DestroyedKey,
            LifecycleError::Revoked => KeyError::RevokedKey,
            LifecycleError::MissingKey
            | LifecycleError::PurposeMismatch
            | LifecycleError::GenerationRollback
            | LifecycleError::InvalidTransition => KeyError::InvalidKeyState,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyMutation {
    Register,
    Rotate,
    Revoke,
    Destroy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleEvent {
    pub mutation: KeyMutation,
    pub key: KeyId,
    pub purpose: KeyPurpose,
    pub generation: u64,
    pub epoch: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyLifecycle {
    records: [Option<KeyRecord>; 32],
    count: usize,
    next_epoch: u64,
}

impl Default for KeyLifecycle {
    fn default() -> Self {
        Self {
            records: [None; 32],
            count: 0,
            next_epoch: 1,
        }
    }
}

impl KeyLifecycle {
    pub const MAX_KEYS: usize = 32;

    pub fn register(
        &mut self,
        id: KeyId,
        purpose: KeyPurpose,
        generation: u64,
    ) -> Result<LifecycleEvent, LifecycleError> {
        if self.find(id).is_some() || self.count >= Self::MAX_KEYS {
            return Err(LifecycleError::InvalidTransition);
        }
        let record = KeyRecord {
            id,
            purpose,
            generation,
            epoch: self.next_epoch(),
            state: LifecycleState::Active,
        };
        self.insert(record)?;
        Ok(self.event(KeyMutation::Register, record))
    }

    pub fn rotate(
        &mut self,
        old: KeyId,
        new: KeyId,
        generation: u64,
    ) -> Result<LifecycleEvent, LifecycleError> {
        let old_record = *self.find(old).ok_or(LifecycleError::MissingKey)?;
        if old_record.state != LifecycleState::Active {
            return Err(match old_record.state {
                LifecycleState::Revoked => LifecycleError::Revoked,
                LifecycleState::Destroyed => LifecycleError::Destroyed,
                LifecycleState::Active => LifecycleError::InvalidTransition,
            });
        }
        if generation <= old_record.generation {
            return Err(LifecycleError::GenerationRollback);
        }
        let event = self.register(new, old_record.purpose, generation)?;
        self.transition(old, LifecycleState::Revoked)?;
        Ok(event)
    }

    pub fn revoke(
        &mut self,
        id: KeyId,
    ) -> Result<LifecycleEvent, LifecycleError> {
        let mut record = *self.find(id).ok_or(LifecycleError::MissingKey)?;
        if record.state != LifecycleState::Active {
            return Err(match record.state {
                LifecycleState::Revoked => LifecycleError::Revoked,
                LifecycleState::Destroyed => LifecycleError::Destroyed,
                LifecycleState::Active => LifecycleError::InvalidTransition,
            });
        }
        record.state = LifecycleState::Revoked;
        record.epoch = self.next_epoch();
        self.replace(record)?;
        Ok(self.event(KeyMutation::Revoke, record))
    }

    pub fn destroy(
        &mut self,
        id: KeyId,
    ) -> Result<LifecycleEvent, LifecycleError> {
        let mut record = *self.find(id).ok_or(LifecycleError::MissingKey)?;
        if record.state == LifecycleState::Destroyed {
            return Err(LifecycleError::Destroyed);
        }
        record.state = LifecycleState::Destroyed;
        record.epoch = self.next_epoch();
        self.replace(record)?;
        Ok(self.event(KeyMutation::Destroy, record))
    }

    pub fn require_active(
        &self,
        id: KeyId,
        purpose: KeyPurpose,
    ) -> Result<KeyRecord, LifecycleError> {
        let record = *self.find(id).ok_or(LifecycleError::MissingKey)?;
        if record.purpose != purpose {
            return Err(LifecycleError::PurposeMismatch);
        }
        match record.state {
            LifecycleState::Active => Ok(record),
            LifecycleState::Revoked => Err(LifecycleError::Revoked),
            LifecycleState::Destroyed => Err(LifecycleError::Destroyed),
        }
    }

    pub fn get(&self, id: KeyId) -> Option<KeyRecord> {
        self.find(id).copied()
    }

    fn next_epoch(&mut self) -> u64 {
        let epoch = self.next_epoch;
        self.next_epoch = self.next_epoch.saturating_add(1).max(1);
        epoch
    }

    fn event(&self, mutation: KeyMutation, record: KeyRecord) -> LifecycleEvent {
        LifecycleEvent {
            mutation,
            key: record.id,
            purpose: record.purpose,
            generation: record.generation,
            epoch: record.epoch,
        }
    }

    fn insert(&mut self, record: KeyRecord) -> Result<(), LifecycleError> {
        for slot in &mut self.records {
            if slot.is_none() {
                *slot = Some(record);
                self.count += 1;
                return Ok(());
            }
        }
        Err(LifecycleError::InvalidTransition)
    }

    fn replace(&mut self, record: KeyRecord) -> Result<(), LifecycleError> {
        for slot in &mut self.records {
            if slot.map(|item| item.id) == Some(record.id) {
                *slot = Some(record);
                return Ok(());
            }
        }
        Err(LifecycleError::MissingKey)
    }

    fn find(&self, id: KeyId) -> Option<&KeyRecord> {
        self.records.iter().flatten().find(|record| record.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u8) -> KeyId {
        KeyId::new([value; 16])
    }

    #[test]
    fn registration_and_purpose_binding_are_deterministic() {
        let mut lifecycle = KeyLifecycle::default();
        lifecycle
            .register(id(1), KeyPurpose::DeviceIdentity, 1)
            .unwrap();

        assert!(lifecycle
            .require_active(id(1), KeyPurpose::DeviceIdentity)
            .is_ok());
        assert_eq!(
            lifecycle.require_active(id(1), KeyPurpose::Message),
            Err(LifecycleError::PurposeMismatch)
        );
    }

    #[test]
    fn rotation_is_monotonic_and_revokes_old_key() {
        let mut lifecycle = KeyLifecycle::default();
        lifecycle
            .register(id(1), KeyPurpose::Session, 4)
            .unwrap();

        let event = lifecycle.rotate(id(1), id(2), 5).unwrap();
        assert_eq!(event.mutation, KeyMutation::Register);
        assert_eq!(
            lifecycle.get(id(1)).unwrap().state(),
            LifecycleState::Revoked
        );
        assert!(lifecycle
            .require_active(id(2), KeyPurpose::Session)
            .is_ok());
    }

    #[test]
    fn generation_rollback_is_rejected() {
        let mut lifecycle = KeyLifecycle::default();
        lifecycle
            .register(id(1), KeyPurpose::Session, 9)
            .unwrap();

        assert_eq!(
            lifecycle.rotate(id(1), id(2), 9),
            Err(LifecycleError::GenerationRollback)
        );
        assert_eq!(
            lifecycle.rotate(id(1), id(3), 8),
            Err(LifecycleError::GenerationRollback)
        );
    }

    #[test]
    fn revocation_and_destruction_are_fail_closed() {
        let mut lifecycle = KeyLifecycle::default();
        lifecycle
            .register(id(1), KeyPurpose::Recovery, 1)
            .unwrap();
        lifecycle.revoke(id(1)).unwrap();
        assert_eq!(
            lifecycle.require_active(id(1), KeyPurpose::Recovery),
            Err(LifecycleError::Revoked)
        );
        lifecycle.destroy(id(1)).unwrap();
        assert_eq!(
            lifecycle.require_active(id(1), KeyPurpose::Recovery),
            Err(LifecycleError::Destroyed)
        );
        assert_eq!(lifecycle.destroy(id(1)), Err(LifecycleError::Destroyed));
    }

    #[test]
    fn epochs_never_move_backwards() {
        let mut lifecycle = KeyLifecycle::default();
        let a = lifecycle
            .register(id(1), KeyPurpose::Message, 1)
            .unwrap();
        let b = lifecycle.revoke(id(1)).unwrap();
        let c = lifecycle.destroy(id(1)).unwrap();
        assert!(a.epoch < b.epoch);
        assert!(b.epoch < c.epoch);
    }
}
