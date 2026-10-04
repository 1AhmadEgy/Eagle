#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustState {
    Provisioning,
    Pending,
    Trusted,
    Suspended,
    Revoked,
    Replaced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformAssurance {
    Software,
    HardwareBacked,
    PlatformAttested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationAction {
    StartProtectedSession,
    GrantDeviceTrust,
    UseRevokedDevice,
    RecoverHistoricalData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustError {
    InvalidStateTransition,
    AccountMismatch,
    PairingExpired,
    PairingAlreadyConsumed,
    PairingCancelled,
    PairingAlreadyCompleted,
    TrustEpochStale,
    RevokedDevice,
    ReplacedDevice,
    DataRecoveryNotImpliedByAccountRecovery,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustRecord {
    pub account_id: String,
    pub device_id: String,
    pub state: TrustState,
    pub trust_epoch: u64,
    pub assurance: PlatformAssurance,
}

impl TrustRecord {
    pub fn new(
        account_id: impl Into<String>,
        device_id: impl Into<String>,
        assurance: PlatformAssurance,
    ) -> Self {
        Self {
            account_id: account_id.into(),
            device_id: device_id.into(),
            state: TrustState::Provisioning,
            trust_epoch: 0,
            assurance,
        }
    }

    pub fn enter_pending(&mut self) -> Result<(), TrustError> {
        if self.state != TrustState::Provisioning {
            return Err(TrustError::InvalidStateTransition);
        }
        self.state = TrustState::Pending;
        Ok(())
    }

    pub fn approve_trust(
        &mut self,
        account_id: &str,
        pairing: &mut PairingContext,
        current_epoch: u64,
    ) -> Result<SecurityEvent, TrustError> {
        if self.state != TrustState::Pending {
            return Err(TrustError::InvalidStateTransition);
        }
        if self.account_id != account_id {
            return Err(TrustError::AccountMismatch);
        }
        pairing.consume(current_epoch)?;
        self.state = TrustState::Trusted;
        self.trust_epoch = current_epoch;
        Ok(SecurityEvent::TrustPromoted {
            account_id_hash: safe_identifier(&self.account_id),
            device_id_hash: safe_identifier(&self.device_id),
            trust_epoch: self.trust_epoch,
        })
    }

    pub fn suspend(&mut self) -> Result<(), TrustError> {
        if self.state != TrustState::Trusted {
            return Err(TrustError::InvalidStateTransition);
        }
        self.state = TrustState::Suspended;
        Ok(())
    }

    pub fn reinstate(&mut self, current_epoch: u64) -> Result<(), TrustError> {
        if self.state != TrustState::Suspended {
            return Err(TrustError::InvalidStateTransition);
        }
        self.state = TrustState::Trusted;
        self.trust_epoch = current_epoch;
        Ok(())
    }

    pub fn revoke(&mut self, new_epoch: u64) -> Result<SecurityEvent, TrustError> {
        match self.state {
            TrustState::Trusted | TrustState::Suspended => {
                self.state = TrustState::Revoked;
                self.trust_epoch = new_epoch;
                Ok(SecurityEvent::TrustRevoked {
                    account_id_hash: safe_identifier(&self.account_id),
                    device_id_hash: safe_identifier(&self.device_id),
                    trust_epoch: new_epoch,
                })
            }
            _ => Err(TrustError::InvalidStateTransition),
        }
    }

    pub fn replace(&mut self, new_epoch: u64) -> Result<(), TrustError> {
        if !matches!(self.state, TrustState::Trusted | TrustState::Suspended) {
            return Err(TrustError::InvalidStateTransition);
        }
        self.state = TrustState::Replaced;
        self.trust_epoch = new_epoch;
        Ok(())
    }

    pub fn authorize(
        &self,
        action: AuthorizationAction,
        observed_epoch: u64,
    ) -> Result<(), TrustError> {
        if observed_epoch < self.trust_epoch {
            return Err(TrustError::TrustEpochStale);
        }

        match action {
            AuthorizationAction::StartProtectedSession
            | AuthorizationAction::GrantDeviceTrust => match self.state {
                TrustState::Trusted if observed_epoch == self.trust_epoch => Ok(()),
                TrustState::Revoked => Err(TrustError::RevokedDevice),
                TrustState::Replaced => Err(TrustError::ReplacedDevice),
                _ => Err(TrustError::InvalidStateTransition),
            },
            AuthorizationAction::UseRevokedDevice => {
                if self.state == TrustState::Revoked {
                    Err(TrustError::RevokedDevice)
                } else {
                    Err(TrustError::InvalidStateTransition)
                }
            }
            AuthorizationAction::RecoverHistoricalData => {
                Err(TrustError::DataRecoveryNotImpliedByAccountRecovery)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairingContext {
    token_id: String,
    account_id: String,
    expires_at_unix: u64,
    expected_epoch: u64,
    state: PairingState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PairingState {
    Active,
    Consumed,
    Cancelled,
}

impl PairingContext {
    pub fn new(
        token_id: impl Into<String>,
        account_id: impl Into<String>,
        ttl: Duration,
        expected_epoch: u64,
    ) -> Result<Self, TrustError> {
        let now = now_unix().ok_or(TrustError::PairingExpired)?;
        let expires_at_unix = now.saturating_add(ttl.as_secs());
        Ok(Self {
            token_id: token_id.into(),
            account_id: account_id.into(),
            expires_at_unix,
            expected_epoch,
            state: PairingState::Active,
        })
    }

    pub fn verify(&self, account_id: &str, current_epoch: u64) -> Result<(), TrustError> {
        if self.account_id != account_id {
            return Err(TrustError::AccountMismatch);
        }
        if self.state == PairingState::Cancelled {
            return Err(TrustError::PairingCancelled);
        }
        if self.state == PairingState::Consumed {
            return Err(TrustError::PairingAlreadyConsumed);
        }
        if current_epoch != self.expected_epoch {
            return Err(TrustError::TrustEpochStale);
        }
        if now_unix().map_or(true, |now| now >= self.expires_at_unix) {
            return Err(TrustError::PairingExpired);
        }
        Ok(())
    }

    fn consume(&mut self, current_epoch: u64) -> Result<(), TrustError> {
        let account_id = self.account_id.clone();
        self.verify(&account_id, current_epoch)?;
        self.state = PairingState::Consumed;
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), TrustError> {
        if self.state != PairingState::Active {
            return Err(TrustError::PairingAlreadyCompleted);
        }
        self.state = PairingState::Cancelled;
        Ok(())
    }

    pub fn token_id(&self) -> &str {
        &self.token_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityEvent {
    TrustPromoted {
        account_id_hash: String,
        device_id_hash: String,
        trust_epoch: u64,
    },
    TrustRevoked {
        account_id_hash: String,
        device_id_hash: String,
        trust_epoch: u64,
    },
}

pub struct TrustEpochSet {
    current: u64,
    revoked_devices: HashSet<String>,
}

impl TrustEpochSet {
    pub fn new(current: u64) -> Self {
        Self {
            current,
            revoked_devices: HashSet::new(),
        }
    }

    pub fn current(&self) -> u64 {
        self.current
    }

    pub fn advance(&mut self) -> u64 {
        self.current = self.current.saturating_add(1);
        self.current
    }

    pub fn revoke_device(&mut self, device_id: impl Into<String>) -> u64 {
        let epoch = self.advance();
        self.revoked_devices.insert(device_id.into());
        epoch
    }

    pub fn is_revoked(&self, device_id: &str) -> bool {
        self.revoked_devices.contains(device_id)
    }
}

fn now_unix() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_secs())
}

fn safe_identifier(value: &str) -> String {
    // Production identifiers/hashes belong to the approved cryptographic/key-management
    // module. This label is intentionally non-cryptographic and is safe for audit events.
    format!("id:{}", value.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pending_record() -> TrustRecord {
        let mut record =
            TrustRecord::new("acct-a", "dev-a", PlatformAssurance::Software);
        record.enter_pending().unwrap();
        record
    }

    #[test]
    fn pending_cannot_self_promote() {
        let record = pending_record();
        assert_eq!(
            record.authorize(AuthorizationAction::StartProtectedSession, 0),
            Err(TrustError::InvalidStateTransition)
        );
    }

    #[test]
    fn explicit_approval_promotes_pending_device() {
        let mut record =
            TrustRecord::new("acct-a", "dev-a", PlatformAssurance::HardwareBacked);
        record.enter_pending().unwrap();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", Duration::from_secs(60), 7).unwrap();

        let event = record.approve_trust("acct-a", &mut pairing, 7).unwrap();

        assert_eq!(record.state, TrustState::Trusted);
        assert_eq!(record.trust_epoch, 7);
        assert!(matches!(
            event,
            SecurityEvent::TrustPromoted { trust_epoch: 7, .. }
        ));
    }

    #[test]
    fn mismatched_account_is_rejected() {
        let mut record = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", Duration::from_secs(60), 1).unwrap();

        assert_eq!(
            record.approve_trust("acct-b", &mut pairing, 1),
            Err(TrustError::AccountMismatch)
        );
    }

    #[test]
    fn pairing_is_single_use() {
        let mut record = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", Duration::from_secs(60), 3).unwrap();

        record.approve_trust("acct-a", &mut pairing, 3).unwrap();

        assert_eq!(
            pairing.verify("acct-a", 3),
            Err(TrustError::PairingAlreadyConsumed)
        );
    }

    #[test]
    fn pairing_expiry_is_enforced() {
        let pairing =
            PairingContext::new("pair-1", "acct-a", Duration::from_secs(0), 4).unwrap();

        assert_eq!(
            pairing.verify("acct-a", 4),
            Err(TrustError::PairingExpired)
        );
    }

    #[test]
    fn stale_epoch_cannot_authorize() {
        let mut record =
            TrustRecord::new("acct-a", "dev-a", PlatformAssurance::PlatformAttested);
        record.enter_pending().unwrap();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", Duration::from_secs(60), 8).unwrap();

        record.approve_trust("acct-a", &mut pairing, 8).unwrap();

        assert_eq!(
            record.authorize(AuthorizationAction::StartProtectedSession, 7),
            Err(TrustError::TrustEpochStale)
        );
        assert_eq!(
            record.authorize(AuthorizationAction::StartProtectedSession, 8),
            Ok(())
        );
    }

    #[test]
    fn revocation_blocks_future_authorization() {
        let mut record = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", Duration::from_secs(60), 1).unwrap();

        record.approve_trust("acct-a", &mut pairing, 1).unwrap();
        let event = record.revoke(2).unwrap();

        assert!(matches!(
            event,
            SecurityEvent::TrustRevoked { trust_epoch: 2, .. }
        ));
        assert_eq!(
            record.authorize(AuthorizationAction::StartProtectedSession, 2),
            Err(TrustError::RevokedDevice)
        );
        assert_eq!(
            record.authorize(AuthorizationAction::UseRevokedDevice, 2),
            Err(TrustError::RevokedDevice)
        );
    }

    #[test]
    fn revoked_or_replaced_state_cannot_reactivate() {
        let mut revoked = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", Duration::from_secs(60), 1).unwrap();
        revoked.approve_trust("acct-a", &mut pairing, 1).unwrap();
        revoked.revoke(2).unwrap();
        assert_eq!(
            revoked.reinstate(3),
            Err(TrustError::InvalidStateTransition)
        );

        let mut replaced = pending_record();
        let mut replacement_pairing =
            PairingContext::new("pair-2", "acct-a", Duration::from_secs(60), 3).unwrap();
        replaced
            .approve_trust("acct-a", &mut replacement_pairing, 3)
            .unwrap();
        replaced.replace(4).unwrap();
        assert_eq!(
            replaced.reinstate(5),
            Err(TrustError::InvalidStateTransition)
        );
    }

    #[test]
    fn account_recovery_does_not_imply_data_recovery() {
        let record = TrustRecord::new("acct-a", "dev-a", PlatformAssurance::Software);

        assert_eq!(
            record.authorize(AuthorizationAction::RecoverHistoricalData, 0),
            Err(TrustError::DataRecoveryNotImpliedByAccountRecovery)
        );
    }

    #[test]
    fn trust_epoch_set_is_monotonic_and_tracks_revocation() {
        let mut epochs = TrustEpochSet::new(10);
        assert_eq!(epochs.current(), 10);
        let next = epochs.revoke_device("dev-a");
        assert_eq!(next, 11);
        assert_eq!(epochs.current(), 11);
        assert!(epochs.is_revoked("dev-a"));
        assert!(!epochs.is_revoked("dev-b"));
    }

    #[test]
    fn cancelled_pairing_cannot_resume() {
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", Duration::from_secs(60), 1).unwrap();

        pairing.cancel().unwrap();

        assert_eq!(
            pairing.verify("acct-a", 1),
            Err(TrustError::PairingCancelled)
        );
        assert_eq!(
            pairing.cancel(),
            Err(TrustError::PairingAlreadyCompleted)
        );
    }
}
