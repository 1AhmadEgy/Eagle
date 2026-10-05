use std::collections::BTreeMap;

use crate::MessageId;

pub const MAX_REPLAY_WINDOW: usize = 1024;
pub const MAX_FUTURE_SKEW_MS: u64 = 120_000;
pub const MAX_MESSAGE_AGE_MS: u64 = 7 * 24 * 60 * 60 * 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayError {
    InvalidWindow,
    Duplicate,
    SequenceCollision,
    TooOld,
    EpochChanged,
    EpochRollback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessError {
    CreatedInFuture,
    TooOld,
    Expired,
    InvalidExpiry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreshnessPolicy {
    max_age_ms: u64,
    max_future_skew_ms: u64,
}

impl FreshnessPolicy {
    pub const fn bounded_default() -> Self {
        Self {
            max_age_ms: MAX_MESSAGE_AGE_MS,
            max_future_skew_ms: MAX_FUTURE_SKEW_MS,
        }
    }

    pub const fn new(max_age_ms: u64, max_future_skew_ms: u64) -> Self {
        Self {
            max_age_ms,
            max_future_skew_ms,
        }
    }

    pub const fn max_age_ms(&self) -> u64 {
        self.max_age_ms
    }

    pub const fn max_future_skew_ms(&self) -> u64 {
        self.max_future_skew_ms
    }

    pub fn validate(
        &self,
        now_epoch_ms: u64,
        created_at_epoch_ms: u64,
        expires_at_epoch_ms: Option<u64>,
    ) -> Result<(), FreshnessError> {
        let latest_allowed_creation = now_epoch_ms.saturating_add(self.max_future_skew_ms);
        if created_at_epoch_ms > latest_allowed_creation {
            return Err(FreshnessError::CreatedInFuture);
        }

        let oldest_allowed_creation = now_epoch_ms.saturating_sub(self.max_age_ms);
        if created_at_epoch_ms < oldest_allowed_creation {
            return Err(FreshnessError::TooOld);
        }

        if let Some(expires_at) = expires_at_epoch_ms {
            if expires_at <= created_at_epoch_ms {
                return Err(FreshnessError::InvalidExpiry);
            }
            if now_epoch_ms >= expires_at {
                return Err(FreshnessError::Expired);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayWindow {
    epoch: Option<u64>,
    highest_sequence: Option<u64>,
    entries: BTreeMap<u64, MessageId>,
    window_size: usize,
}

impl ReplayWindow {
    pub fn new(window_size: usize) -> Result<Self, ReplayError> {
        if window_size == 0 || window_size > MAX_REPLAY_WINDOW {
            return Err(ReplayError::InvalidWindow);
        }

        Ok(Self {
            epoch: None,
            highest_sequence: None,
            entries: BTreeMap::new(),
            window_size,
        })
    }

    pub const fn window_size(&self) -> usize {
        self.window_size
    }

    pub const fn epoch(&self) -> Option<u64> {
        self.epoch
    }

    pub const fn highest_sequence(&self) -> Option<u64> {
        self.highest_sequence
    }

    pub fn observe(
        &mut self,
        epoch: u64,
        sequence: u64,
        message_id: MessageId,
    ) -> Result<(), ReplayError> {
        match self.epoch {
            None => self.epoch = Some(epoch),
            Some(current) if current != epoch => return Err(ReplayError::EpochChanged),
            Some(_) => {}
        }

        match self.highest_sequence {
            None => {
                self.highest_sequence = Some(sequence);
                self.entries.insert(sequence, message_id);
                Ok(())
            }
            Some(highest) if sequence > highest => {
                self.highest_sequence = Some(sequence);
                self.entries.insert(sequence, message_id);
                self.prune();
                Ok(())
            }
            Some(highest) => {
                let distance = highest.saturating_sub(sequence) as usize;
                if distance >= self.window_size {
                    return Err(ReplayError::TooOld);
                }

                match self.entries.get(&sequence) {
                    Some(existing) if existing == &message_id => Err(ReplayError::Duplicate),
                    Some(_) => Err(ReplayError::SequenceCollision),
                    None => {
                        self.entries.insert(sequence, message_id);
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn contains(&self, sequence: u64, message_id: &MessageId) -> bool {
        self.entries.get(&sequence) == Some(message_id)
    }

    pub fn advance_epoch(&mut self, new_epoch: u64) -> Result<(), ReplayError> {
        match self.epoch {
            None => {
                self.epoch = Some(new_epoch);
                self.highest_sequence = None;
                self.entries.clear();
                Ok(())
            }
            Some(current) if new_epoch > current => {
                self.epoch = Some(new_epoch);
                self.highest_sequence = None;
                self.entries.clear();
                Ok(())
            }
            Some(_) => Err(ReplayError::EpochRollback),
        }
    }

    fn prune(&mut self) {
        let Some(highest) = self.highest_sequence else {
            return;
        };
        let minimum = highest.saturating_sub(self.window_size.saturating_sub(1) as u64);
        self.entries.retain(|sequence, _| *sequence >= minimum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(value: u8) -> MessageId {
        MessageId::new([value; 16])
    }

    #[test]
    fn invalid_window_is_rejected() {
        assert_eq!(ReplayWindow::new(0), Err(ReplayError::InvalidWindow));
        assert_eq!(
            ReplayWindow::new(MAX_REPLAY_WINDOW + 1),
            Err(ReplayError::InvalidWindow)
        );
    }

    #[test]
    fn duplicate_is_idempotently_rejected_without_new_entry() {
        let mut window = ReplayWindow::new(4).unwrap();
        assert_eq!(window.observe(7, 1, message(1)), Ok(()));
        assert_eq!(
            window.observe(7, 1, message(1)),
            Err(ReplayError::Duplicate)
        );
        assert!(window.contains(1, &message(1)));
    }

    #[test]
    fn sequence_reuse_with_different_message_is_rejected() {
        let mut window = ReplayWindow::new(4).unwrap();
        assert_eq!(window.observe(1, 4, message(1)), Ok(()));
        assert_eq!(
            window.observe(1, 4, message(2)),
            Err(ReplayError::SequenceCollision)
        );
    }

    #[test]
    fn bounded_out_of_order_delivery_is_accepted() {
        let mut window = ReplayWindow::new(4).unwrap();
        assert_eq!(window.observe(1, 10, message(10)), Ok(()));
        assert_eq!(window.observe(1, 8, message(8)), Ok(()));
        assert_eq!(window.observe(1, 9, message(9)), Ok(()));
        assert_eq!(window.observe(1, 6, message(6)), Err(ReplayError::TooOld));
    }

    #[test]
    fn epoch_changes_fail_closed() {
        let mut window = ReplayWindow::new(4).unwrap();
        assert_eq!(window.observe(1, 1, message(1)), Ok(()));
        assert_eq!(
            window.observe(2, 1, message(1)),
            Err(ReplayError::EpochChanged)
        );
        assert_eq!(window.epoch(), Some(1));
        assert_eq!(window.highest_sequence(), Some(1));
    }

    #[test]
    fn epoch_can_advance_only_monotonically() {
        let mut window = ReplayWindow::new(4).unwrap();
        assert_eq!(window.observe(1, 7, message(7)), Ok(()));
        assert_eq!(window.advance_epoch(2), Ok(()));
        assert_eq!(window.epoch(), Some(2));
        assert_eq!(window.highest_sequence(), None);
        assert!(!window.contains(7, &message(7)));
        assert_eq!(window.advance_epoch(2), Err(ReplayError::EpochRollback));
        assert_eq!(window.advance_epoch(1), Err(ReplayError::EpochRollback));
    }

    #[test]
    fn freshness_rejects_future_old_and_expired_messages() {
        let policy = FreshnessPolicy::new(1_000, 100);

        assert_eq!(policy.validate(10_000, 10_050, None), Ok(()));
        assert_eq!(
            policy.validate(10_000, 10_101, Some(10_151)),
            Ok(())
        );
        assert_eq!(
            policy.validate(10_000, 10_101, None),
            Ok(())
        );
        assert_eq!(
            policy.validate(10_000, 10_101, Some(10_101)),
            Err(FreshnessError::InvalidExpiry)
        );
        assert_eq!(
            policy.validate(10_200, 10_050, Some(10_150)),
            Err(FreshnessError::Expired)
        );
        assert_eq!(
            policy.validate(10_000, 10_101, None),
            Ok(())
        );
        assert_eq!(
            policy.validate(10_000, 10_101, None),
            Ok(())
        );
        assert_eq!(
            policy.validate(10_101, 10_202, None),
            Err(FreshnessError::CreatedInFuture)
        );
        assert_eq!(
            policy.validate(10_000, 8_999, None),
            Err(FreshnessError::TooOld)
        );
    }
}
