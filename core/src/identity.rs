#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};
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
    TrustEpochNotMonotonic,
    TrustEpochOverflow,
    RevokedDevice,
    ReplacedDevice,
    DataRecoveryNotImpliedByAccountRecovery,
    MalformedIdentityKey,
    MalformedMembershipStatement,
    IdentityCollision,
    UnchangedIdentity,
    DeviceAlreadyBound,
    MembershipEpochStale,
    MembershipEpochFuture,
    MembershipAccountMismatch,
    PairingDeviceMismatch,
    MalformedPairingContext,
    PairingExpiryOverflow,
    PairingApprovalRequired,
    IdentityReverificationRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicIdentityKey {
    bytes: Vec<u8>,
}

impl PublicIdentityKey {
    pub fn new(bytes: Vec<u8>) -> Result<Self, TrustError> {
        if bytes.is_empty() {
            return Err(TrustError::MalformedIdentityKey);
        }
        Ok(Self { bytes })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityReference {
    pub id: String,
    pub public_key: PublicIdentityKey,
}

impl IdentityReference {
    pub fn new(id: impl Into<String>, public_key: PublicIdentityKey) -> Result<Self, TrustError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(TrustError::MalformedIdentityKey);
        }
        Ok(Self { id, public_key })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountMembershipStatement {
    pub protocol_version: u16,
    pub account: IdentityReference,
    pub device: IdentityReference,
    pub issued_at_unix: u64,
    pub not_before_unix: u64,
    pub not_after_unix: Option<u64>,
    pub trust_epoch: u64,
    pub capabilities: u64,
}

impl AccountMembershipStatement {
    pub fn validate_structure(&self) -> Result<(), TrustError> {
        if self.protocol_version == 0 {
            return Err(TrustError::MalformedMembershipStatement);
        }
        if self.issued_at_unix < self.not_before_unix {
            return Err(TrustError::MalformedMembershipStatement);
        }
        if let Some(not_after) = self.not_after_unix {
            if not_after <= self.not_before_unix || self.issued_at_unix > not_after {
                return Err(TrustError::MalformedMembershipStatement);
            }
        }
        if self.account.id == self.device.id {
            return Err(TrustError::IdentityCollision);
        }
        Ok(())
    }
}

pub trait MembershipProofVerifier {
    fn verify_identity_binding(&self, identity: &IdentityReference) -> Result<(), TrustError>;

    fn verify_membership(&self, statement: &AccountMembershipStatement) -> Result<(), TrustError>;
}

pub trait ContactReverificationVerifier {
    fn verify_reverification(
        &self,
        current: &ContactIdentity,
        replacement: &IdentityReference,
    ) -> Result<(), TrustError>;
}

pub trait PairingApprovalVerifier {
    fn verify_approval(
        &self,
        pairing: &PairingContext,
        account_id: &str,
        device_id: &str,
        current_epoch: u64,
    ) -> Result<(), TrustError>;
}

pub fn validate_membership(
    statement: &AccountMembershipStatement,
    verifier: &impl MembershipProofVerifier,
) -> Result<(), TrustError> {
    statement.validate_structure()?;
    verifier.verify_identity_binding(&statement.account)?;
    verifier.verify_identity_binding(&statement.device)?;
    verifier.verify_membership(statement)
}

#[derive(Debug, Default)]
pub struct MembershipRegistry {
    device_accounts: HashMap<String, String>,
    account_epochs: HashMap<String, u64>,
}

impl MembershipRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn observe_account_epoch(
        &mut self,
        account_id: impl Into<String>,
        epoch: u64,
    ) -> Result<(), TrustError> {
        let account_id = account_id.into();
        match self.account_epochs.get(&account_id).copied() {
            Some(current) if epoch < current => Err(TrustError::MembershipEpochStale),
            _ => {
                self.account_epochs.insert(account_id, epoch);
                Ok(())
            }
        }
    }

    pub fn bind(
        &mut self,
        statement: &AccountMembershipStatement,
        current_epoch: u64,
        verifier: &impl MembershipProofVerifier,
    ) -> Result<(), TrustError> {
        statement.validate_structure()?;
        match statement.trust_epoch.cmp(&current_epoch) {
            std::cmp::Ordering::Less => return Err(TrustError::MembershipEpochStale),
            std::cmp::Ordering::Greater => return Err(TrustError::MembershipEpochFuture),
            std::cmp::Ordering::Equal => {}
        }

        if let Some(bound_account) = self.device_accounts.get(&statement.device.id) {
            if bound_account != &statement.account.id {
                return Err(TrustError::MembershipAccountMismatch);
            }
            return Err(TrustError::DeviceAlreadyBound);
        }

        validate_membership(statement, verifier)?;
        self.observe_account_epoch(statement.account.id.clone(), statement.trust_epoch)?;
        self.device_accounts
            .insert(statement.device.id.clone(), statement.account.id.clone());
        Ok(())
    }

    pub fn bound_account(&self, device_id: &str) -> Option<&str> {
        self.device_accounts.get(device_id).map(String::as_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactIdentityState {
    Verified,
    Quarantined,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactIdentity {
    /// The last identity that has actually been verified and is safe for trust decisions.
    pub identity: IdentityReference,
    /// An untrusted replacement candidate. It must never be treated as verified identity.
    pub pending_identity: Option<IdentityReference>,
    pub state: ContactIdentityState,
}

impl ContactIdentity {
    pub fn new(identity: IdentityReference) -> Self {
        Self {
            identity,
            pending_identity: None,
            state: ContactIdentityState::Verified,
        }
    }

    pub fn observe_identity_change(
        &mut self,
        replacement: IdentityReference,
    ) -> Result<(), TrustError> {
        if self.identity.id == replacement.id && self.identity.public_key == replacement.public_key
        {
            return Err(TrustError::UnchangedIdentity);
        }
        self.pending_identity = Some(replacement);
        self.state = ContactIdentityState::Quarantined;
        Ok(())
    }

    pub fn reverify(
        &mut self,
        verified_identity: IdentityReference,
        verifier: &impl ContactReverificationVerifier,
    ) -> Result<(), TrustError> {
        if self.state != ContactIdentityState::Quarantined {
            return Err(TrustError::InvalidStateTransition);
        }
        let pending = self
            .pending_identity
            .as_ref()
            .ok_or(TrustError::IdentityReverificationRequired)?;
        if pending != &verified_identity {
            return Err(TrustError::IdentityReverificationRequired);
        }
        verifier.verify_reverification(self, &verified_identity)?;
        self.identity = verified_identity;
        self.pending_identity = None;
        self.state = ContactIdentityState::Verified;
        Ok(())
    }
}

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
        approval_verifier: &impl PairingApprovalVerifier,
    ) -> Result<SecurityEvent, TrustError> {
        if self.state != TrustState::Pending {
            return Err(TrustError::InvalidStateTransition);
        }
        if self.account_id != account_id {
            return Err(TrustError::AccountMismatch);
        }
        if pairing.account_id != self.account_id || pairing.device_id != self.device_id {
            return Err(TrustError::PairingDeviceMismatch);
        }
        pairing.verify(&self.account_id, &self.device_id, current_epoch)?;
        approval_verifier.verify_approval(
            pairing,
            &self.account_id,
            &self.device_id,
            current_epoch,
        )?;
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

    pub fn revoke(&mut self, new_epoch: u64) -> Result<SecurityEvent, TrustError> {
        if new_epoch <= self.trust_epoch {
            return Err(TrustError::TrustEpochNotMonotonic);
        }
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
        if new_epoch <= self.trust_epoch {
            return Err(TrustError::TrustEpochNotMonotonic);
        }
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
            AuthorizationAction::StartProtectedSession => match self.state {
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
    device_id: String,
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
        device_id: impl Into<String>,
        ttl: Duration,
        expected_epoch: u64,
    ) -> Result<Self, TrustError> {
        let token_id = token_id.into();
        let account_id = account_id.into();
        let device_id = device_id.into();
        if token_id.trim().is_empty() || account_id.trim().is_empty() || device_id.trim().is_empty()
        {
            return Err(TrustError::MalformedPairingContext);
        }
        let now = now_unix().ok_or(TrustError::PairingExpired)?;
        let expires_at_unix = now
            .checked_add(ttl.as_secs())
            .ok_or(TrustError::PairingExpiryOverflow)?;
        Ok(Self {
            token_id,
            account_id,
            device_id,
            expires_at_unix,
            expected_epoch,
            state: PairingState::Active,
        })
    }

    pub fn verify(
        &self,
        account_id: &str,
        device_id: &str,
        current_epoch: u64,
    ) -> Result<(), TrustError> {
        if self.account_id != account_id {
            return Err(TrustError::AccountMismatch);
        }
        if self.device_id != device_id {
            return Err(TrustError::PairingDeviceMismatch);
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
        if now_unix().is_none_or(|now| now >= self.expires_at_unix) {
            return Err(TrustError::PairingExpired);
        }
        Ok(())
    }

    fn consume(&mut self, current_epoch: u64) -> Result<(), TrustError> {
        let account_id = self.account_id.clone();
        let device_id = self.device_id.clone();
        self.verify(&account_id, &device_id, current_epoch)?;
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

    pub fn advance(&mut self) -> Result<u64, TrustError> {
        self.current = self
            .current
            .checked_add(1)
            .ok_or(TrustError::TrustEpochOverflow)?;
        Ok(self.current)
    }

    pub fn revoke_device(&mut self, device_id: impl Into<String>) -> Result<u64, TrustError> {
        let epoch = self.advance()?;
        self.revoked_devices.insert(device_id.into());
        Ok(epoch)
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
    let _ = value;
    "redacted".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pending_record() -> TrustRecord {
        let mut record = TrustRecord::new("acct-a", "dev-a", PlatformAssurance::Software);
        record.enter_pending().unwrap();
        record
    }

    struct AcceptAllVerifier;

    impl MembershipProofVerifier for AcceptAllVerifier {
        fn verify_identity_binding(&self, _identity: &IdentityReference) -> Result<(), TrustError> {
            Ok(())
        }

        fn verify_membership(
            &self,
            _statement: &AccountMembershipStatement,
        ) -> Result<(), TrustError> {
            Ok(())
        }
    }

    struct AcceptApprovalVerifier;

    impl MembershipProofVerifier for AcceptApprovalVerifier {
        fn verify_identity_binding(&self, _identity: &IdentityReference) -> Result<(), TrustError> {
            Ok(())
        }

        fn verify_membership(
            &self,
            _statement: &AccountMembershipStatement,
        ) -> Result<(), TrustError> {
            Ok(())
        }
    }

    impl PairingApprovalVerifier for AcceptApprovalVerifier {
        fn verify_approval(
            &self,
            _pairing: &PairingContext,
            _account_id: &str,
            _device_id: &str,
            _current_epoch: u64,
        ) -> Result<(), TrustError> {
            Ok(())
        }
    }

    struct AcceptReverificationVerifier;

    impl ContactReverificationVerifier for AcceptReverificationVerifier {
        fn verify_reverification(
            &self,
            _current: &ContactIdentity,
            _replacement: &IdentityReference,
        ) -> Result<(), TrustError> {
            Ok(())
        }
    }

    struct RejectApprovalVerifier;

    impl MembershipProofVerifier for RejectApprovalVerifier {
        fn verify_identity_binding(&self, _identity: &IdentityReference) -> Result<(), TrustError> {
            Ok(())
        }

        fn verify_membership(
            &self,
            _statement: &AccountMembershipStatement,
        ) -> Result<(), TrustError> {
            Ok(())
        }
    }

    impl PairingApprovalVerifier for RejectApprovalVerifier {
        fn verify_approval(
            &self,
            _pairing: &PairingContext,
            _account_id: &str,
            _device_id: &str,
            _current_epoch: u64,
        ) -> Result<(), TrustError> {
            Err(TrustError::PairingApprovalRequired)
        }
    }

    struct PanicMembershipVerifier;

    impl MembershipProofVerifier for PanicMembershipVerifier {
        fn verify_identity_binding(&self, _identity: &IdentityReference) -> Result<(), TrustError> {
            panic!("membership verifier must not run for rejected epoch");
        }

        fn verify_membership(
            &self,
            _statement: &AccountMembershipStatement,
        ) -> Result<(), TrustError> {
            panic!("membership verifier must not run for rejected epoch");
        }
    }

    struct RejectReverificationVerifier;

    impl ContactReverificationVerifier for RejectReverificationVerifier {
        fn verify_reverification(
            &self,
            _current: &ContactIdentity,
            _replacement: &IdentityReference,
        ) -> Result<(), TrustError> {
            Err(TrustError::IdentityReverificationRequired)
        }
    }

    #[test]
    fn membership_registry_rejects_cross_account_device_reuse() {
        let key_a = PublicIdentityKey::new(vec![1]).unwrap();
        let key_b = PublicIdentityKey::new(vec![2]).unwrap();
        let key_c = PublicIdentityKey::new(vec![3]).unwrap();
        let account_a = IdentityReference::new("acct-a", key_a).unwrap();
        let account_b = IdentityReference::new("acct-b", key_b).unwrap();
        let device = IdentityReference::new("dev-1", key_c).unwrap();
        let verifier = AcceptAllVerifier;
        let mut registry = MembershipRegistry::new();

        let first = AccountMembershipStatement {
            protocol_version: 1,
            account: account_a,
            device: device.clone(),
            issued_at_unix: 10,
            not_before_unix: 10,
            not_after_unix: None,
            trust_epoch: 4,
            capabilities: 0,
        };
        registry.bind(&first, 4, &verifier).unwrap();

        let second = AccountMembershipStatement {
            protocol_version: 1,
            account: account_b,
            device,
            issued_at_unix: 11,
            not_before_unix: 11,
            not_after_unix: None,
            trust_epoch: 4,
            capabilities: 0,
        };

        assert_eq!(
            registry.bind(&second, 4, &verifier),
            Err(TrustError::MembershipAccountMismatch)
        );
    }

    #[test]
    fn membership_registry_rejects_stale_membership_epoch() {
        let key_a = PublicIdentityKey::new(vec![1]).unwrap();
        let key_b = PublicIdentityKey::new(vec![2]).unwrap();
        let account = IdentityReference::new("acct-a", key_a).unwrap();
        let device = IdentityReference::new("dev-1", key_b).unwrap();
        let statement = AccountMembershipStatement {
            protocol_version: 1,
            account,
            device,
            issued_at_unix: 10,
            not_before_unix: 10,
            not_after_unix: None,
            trust_epoch: 3,
            capabilities: 0,
        };
        let mut registry = MembershipRegistry::new();

        assert_eq!(
            registry.bind(&statement, 4, &AcceptAllVerifier),
            Err(TrustError::MembershipEpochStale)
        );
    }

    #[test]
    fn membership_registry_rejects_duplicate_same_account_binding() {
        let key_a = PublicIdentityKey::new(vec![1]).unwrap();
        let key_b = PublicIdentityKey::new(vec![2]).unwrap();
        let account = IdentityReference::new("acct-a", key_a).unwrap();
        let device = IdentityReference::new("dev-1", key_b).unwrap();
        let statement = AccountMembershipStatement {
            protocol_version: 1,
            account,
            device,
            issued_at_unix: 10,
            not_before_unix: 10,
            not_after_unix: None,
            trust_epoch: 4,
            capabilities: 0,
        };
        let mut registry = MembershipRegistry::new();

        registry.bind(&statement, 4, &AcceptAllVerifier).unwrap();
        assert_eq!(
            registry.bind(&statement, 4, &AcceptAllVerifier),
            Err(TrustError::DeviceAlreadyBound)
        );
    }

    #[test]
    fn malformed_identity_key_is_rejected() {
        assert_eq!(
            PublicIdentityKey::new(Vec::new()),
            Err(TrustError::MalformedIdentityKey)
        );
    }

    #[test]
    fn membership_statement_is_structurally_validated_before_crypto_verification() {
        let key_a = PublicIdentityKey::new(vec![1]).unwrap();
        let key_b = PublicIdentityKey::new(vec![2]).unwrap();
        let account = IdentityReference::new("acct-1", key_a).unwrap();
        let device = IdentityReference::new("dev-1", key_b).unwrap();

        let statement = AccountMembershipStatement {
            protocol_version: 1,
            account,
            device,
            issued_at_unix: 20,
            not_before_unix: 10,
            not_after_unix: Some(30),
            trust_epoch: 7,
            capabilities: 0,
        };

        validate_membership(&statement, &AcceptAllVerifier).unwrap();
    }

    #[test]
    fn malformed_membership_time_window_is_rejected() {
        let key_a = PublicIdentityKey::new(vec![1]).unwrap();
        let key_b = PublicIdentityKey::new(vec![2]).unwrap();
        let account = IdentityReference::new("acct-1", key_a).unwrap();
        let device = IdentityReference::new("dev-1", key_b).unwrap();

        let statement = AccountMembershipStatement {
            protocol_version: 1,
            account,
            device,
            issued_at_unix: 5,
            not_before_unix: 10,
            not_after_unix: Some(9),
            trust_epoch: 1,
            capabilities: 0,
        };

        assert_eq!(
            validate_membership(&statement, &AcceptAllVerifier),
            Err(TrustError::MalformedMembershipStatement)
        );
    }

    #[test]
    fn future_membership_epoch_is_rejected_fail_closed() {
        let key_a = PublicIdentityKey::new(vec![1]).unwrap();
        let key_b = PublicIdentityKey::new(vec![2]).unwrap();
        let account = IdentityReference::new("acct-a", key_a).unwrap();
        let device = IdentityReference::new("dev-1", key_b).unwrap();

        let statement = AccountMembershipStatement {
            protocol_version: 1,
            account,
            device,
            issued_at_unix: 10,
            not_before_unix: 10,
            not_after_unix: None,
            trust_epoch: 5,
            capabilities: 0,
        };

        let mut registry = MembershipRegistry::new();
        assert_eq!(
            registry.bind(&statement, 4, &AcceptAllVerifier),
            Err(TrustError::MembershipEpochFuture)
        );
        assert!(registry.bound_account("dev-1").is_none());
    }

    #[test]
    fn rejected_future_epoch_does_not_invoke_membership_verifier() {
        let key_a = PublicIdentityKey::new(vec![1]).unwrap();
        let key_b = PublicIdentityKey::new(vec![2]).unwrap();
        let account = IdentityReference::new("acct-a", key_a).unwrap();
        let device = IdentityReference::new("dev-1", key_b).unwrap();

        let statement = AccountMembershipStatement {
            protocol_version: 1,
            account,
            device,
            issued_at_unix: 10,
            not_before_unix: 10,
            not_after_unix: None,
            trust_epoch: 5,
            capabilities: 0,
        };

        let mut registry = MembershipRegistry::new();
        assert_eq!(
            registry.bind(&statement, 4, &PanicMembershipVerifier),
            Err(TrustError::MembershipEpochFuture)
        );
    }

    #[test]
    fn membership_statement_issued_after_expiry_is_rejected() {
        let key_a = PublicIdentityKey::new(vec![1]).unwrap();
        let key_b = PublicIdentityKey::new(vec![2]).unwrap();
        let account = IdentityReference::new("acct-1", key_a).unwrap();
        let device = IdentityReference::new("dev-1", key_b).unwrap();

        let statement = AccountMembershipStatement {
            protocol_version: 1,
            account,
            device,
            issued_at_unix: 100,
            not_before_unix: 10,
            not_after_unix: Some(50),
            trust_epoch: 1,
            capabilities: 0,
        };

        assert_eq!(
            statement.validate_structure(),
            Err(TrustError::MalformedMembershipStatement)
        );
    }

    #[test]
    fn identity_key_change_enters_quarantine_until_reverified() {
        let old_key = PublicIdentityKey::new(vec![1]).unwrap();
        let new_key = PublicIdentityKey::new(vec![2]).unwrap();
        let old_identity = IdentityReference::new("dev-old", old_key).unwrap();
        let new_identity = IdentityReference::new("dev-new", new_key).unwrap();
        let mut contact = ContactIdentity::new(old_identity.clone());

        contact
            .observe_identity_change(new_identity.clone())
            .unwrap();
        assert_eq!(contact.state, ContactIdentityState::Quarantined);
        assert_eq!(contact.identity, old_identity);
        assert_eq!(contact.pending_identity, Some(new_identity.clone()));

        contact
            .reverify(new_identity, &AcceptReverificationVerifier)
            .unwrap();
        assert_eq!(contact.state, ContactIdentityState::Verified);
    }

    #[test]
    fn mismatched_reverification_candidate_is_rejected() {
        let old_key = PublicIdentityKey::new(vec![1]).unwrap();
        let pending_key = PublicIdentityKey::new(vec![2]).unwrap();
        let different_key = PublicIdentityKey::new(vec![3]).unwrap();
        let old_identity = IdentityReference::new("dev-old", old_key).unwrap();
        let pending_identity = IdentityReference::new("dev-new", pending_key).unwrap();
        let different_identity = IdentityReference::new("dev-other", different_key).unwrap();
        let mut contact = ContactIdentity::new(old_identity.clone());

        contact
            .observe_identity_change(pending_identity.clone())
            .unwrap();

        assert_eq!(
            contact.reverify(different_identity, &AcceptReverificationVerifier),
            Err(TrustError::IdentityReverificationRequired)
        );
        assert_eq!(contact.identity, old_identity);
        assert_eq!(contact.pending_identity, Some(pending_identity));
        assert_eq!(contact.state, ContactIdentityState::Quarantined);
    }

    #[test]
    fn rejected_identity_reverification_keeps_quarantine() {
        let old_key = PublicIdentityKey::new(vec![1]).unwrap();
        let new_key = PublicIdentityKey::new(vec![2]).unwrap();
        let old_identity = IdentityReference::new("dev-old", old_key).unwrap();
        let new_identity = IdentityReference::new("dev-new", new_key).unwrap();
        let mut contact = ContactIdentity::new(old_identity);

        contact
            .observe_identity_change(new_identity.clone())
            .unwrap();

        assert_eq!(
            contact.reverify(new_identity, &RejectReverificationVerifier),
            Err(TrustError::IdentityReverificationRequired)
        );
        assert_eq!(contact.state, ContactIdentityState::Quarantined);
    }

    #[test]
    fn identical_identity_change_is_rejected() {
        let key = PublicIdentityKey::new(vec![1]).unwrap();
        let identity = IdentityReference::new("dev-a", key).unwrap();
        let mut contact = ContactIdentity::new(identity.clone());

        assert_eq!(
            contact.observe_identity_change(identity),
            Err(TrustError::UnchangedIdentity)
        );
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
        let mut record = TrustRecord::new("acct-a", "dev-a", PlatformAssurance::HardwareBacked);
        record.enter_pending().unwrap();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 7).unwrap();

        let event = record
            .approve_trust("acct-a", &mut pairing, 7, &AcceptApprovalVerifier)
            .unwrap();

        assert_eq!(record.state, TrustState::Trusted);
        assert_eq!(record.trust_epoch, 7);
        assert!(matches!(
            event,
            SecurityEvent::TrustPromoted { trust_epoch: 7, .. }
        ));
    }

    #[test]
    fn rejected_pairing_approval_cannot_promote_device() {
        let mut record = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 1).unwrap();

        assert_eq!(
            record.approve_trust("acct-a", &mut pairing, 1, &RejectApprovalVerifier),
            Err(TrustError::PairingApprovalRequired)
        );
        assert_eq!(record.state, TrustState::Pending);
    }

    #[test]
    fn mismatched_account_is_rejected() {
        let mut record = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 1).unwrap();

        assert_eq!(
            record.approve_trust("acct-b", &mut pairing, 1, &AcceptApprovalVerifier),
            Err(TrustError::AccountMismatch)
        );
    }

    #[test]
    fn malformed_pairing_context_is_rejected() {
        assert_eq!(
            PairingContext::new("", "acct-a", "dev-a", Duration::from_secs(60), 1),
            Err(TrustError::MalformedPairingContext)
        );
        assert_eq!(
            PairingContext::new("pair-1", "acct-a", "", Duration::from_secs(60), 1),
            Err(TrustError::MalformedPairingContext)
        );
    }

    #[test]
    fn pairing_expiry_overflow_fails_closed() {
        assert_eq!(
            PairingContext::new(
                "pair-1",
                "acct-a",
                "dev-a",
                Duration::from_secs(u64::MAX),
                1
            ),
            Err(TrustError::PairingExpiryOverflow)
        );
    }

    #[test]
    fn pairing_is_bound_to_the_expected_device() {
        let mut record = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-other", Duration::from_secs(60), 3)
                .unwrap();

        assert_eq!(
            record.approve_trust("acct-a", &mut pairing, 3, &AcceptApprovalVerifier),
            Err(TrustError::PairingDeviceMismatch)
        );
        assert_eq!(record.state, TrustState::Pending);
    }

    #[test]
    fn pairing_is_single_use() {
        let mut record = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 3).unwrap();

        record
            .approve_trust("acct-a", &mut pairing, 3, &AcceptApprovalVerifier)
            .unwrap();

        assert_eq!(
            pairing.verify("acct-a", "dev-a", 3),
            Err(TrustError::PairingAlreadyConsumed)
        );
    }

    #[test]
    fn pairing_expiry_is_enforced() {
        let pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(0), 4).unwrap();

        assert_eq!(
            pairing.verify("acct-a", "dev-a", 4),
            Err(TrustError::PairingExpired)
        );
    }

    #[test]
    fn stale_epoch_cannot_authorize() {
        let mut record = TrustRecord::new("acct-a", "dev-a", PlatformAssurance::PlatformAttested);
        record.enter_pending().unwrap();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 8).unwrap();

        record
            .approve_trust("acct-a", &mut pairing, 8, &AcceptApprovalVerifier)
            .unwrap();

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
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 1).unwrap();

        record
            .approve_trust("acct-a", &mut pairing, 1, &AcceptApprovalVerifier)
            .unwrap();
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
    fn revoked_or_replaced_state_cannot_authorize() {
        let mut revoked = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 1).unwrap();
        revoked
            .approve_trust("acct-a", &mut pairing, 1, &AcceptApprovalVerifier)
            .unwrap();
        revoked.revoke(2).unwrap();
        assert_eq!(
            revoked.authorize(AuthorizationAction::StartProtectedSession, 2),
            Err(TrustError::RevokedDevice)
        );

        let mut replaced = pending_record();
        let mut replacement_pairing =
            PairingContext::new("pair-2", "acct-a", "dev-a", Duration::from_secs(60), 3).unwrap();
        replaced
            .approve_trust(
                "acct-a",
                &mut replacement_pairing,
                3,
                &AcceptApprovalVerifier,
            )
            .unwrap();
        replaced.replace(4).unwrap();
        assert_eq!(
            replaced.authorize(AuthorizationAction::StartProtectedSession, 4),
            Err(TrustError::ReplacedDevice)
        );
    }

    #[test]
    fn suspended_device_cannot_start_session() {
        let mut record = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 5).unwrap();
        record
            .approve_trust("acct-a", &mut pairing, 5, &AcceptApprovalVerifier)
            .unwrap();
        record.suspend().unwrap();

        assert_eq!(
            record.authorize(AuthorizationAction::StartProtectedSession, 5),
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
    fn trust_epoch_must_increase_for_revocation() {
        let mut record = pending_record();
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 2).unwrap();
        record
            .approve_trust("acct-a", &mut pairing, 2, &AcceptApprovalVerifier)
            .unwrap();

        assert_eq!(record.revoke(2), Err(TrustError::TrustEpochNotMonotonic));
        assert_eq!(record.replace(1), Err(TrustError::TrustEpochNotMonotonic));
    }

    #[test]
    fn trust_epoch_overflow_fails_closed() {
        let mut epochs = TrustEpochSet::new(u64::MAX);
        assert_eq!(epochs.advance(), Err(TrustError::TrustEpochOverflow));
    }

    #[test]
    fn trust_epoch_set_is_monotonic_and_tracks_revocation() {
        let mut epochs = TrustEpochSet::new(10);
        assert_eq!(epochs.current(), 10);
        let next = epochs.revoke_device("dev-a").unwrap();
        assert_eq!(next, 11);
        assert_eq!(epochs.current(), 11);
        assert!(epochs.is_revoked("dev-a"));
        assert!(!epochs.is_revoked("dev-b"));
    }

    #[test]
    fn cancelled_pairing_cannot_resume() {
        let mut pairing =
            PairingContext::new("pair-1", "acct-a", "dev-a", Duration::from_secs(60), 1).unwrap();

        pairing.cancel().unwrap();

        assert_eq!(
            pairing.verify("acct-a", "dev-a", 1),
            Err(TrustError::PairingCancelled)
        );
        assert_eq!(pairing.cancel(), Err(TrustError::PairingAlreadyCompleted));
    }
}
