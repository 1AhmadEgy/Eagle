use crate::{ContentBinding, FreshnessError, FreshnessPolicy, MessageId, ReplayError, ReplayWindow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeliveryMetadata {
    epoch: u64,
    sequence: u64,
    created_at_epoch_ms: u64,
    expires_at_epoch_ms: Option<u64>,
}

impl DeliveryMetadata {
    pub const fn new(
        epoch: u64,
        sequence: u64,
        created_at_epoch_ms: u64,
        expires_at_epoch_ms: Option<u64>,
    ) -> Self {
        Self {
            epoch,
            sequence,
            created_at_epoch_ms,
            expires_at_epoch_ms,
        }
    }

    pub const fn epoch(&self) -> u64 {
        self.epoch
    }

    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    pub const fn created_at_epoch_ms(&self) -> u64 {
        self.created_at_epoch_ms
    }

    pub const fn expires_at_epoch_ms(&self) -> Option<u64> {
        self.expires_at_epoch_ms
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryGuardError {
    Replay(ReplayError),
    Freshness(FreshnessError),
}

impl From<ReplayError> for DeliveryGuardError {
    fn from(value: ReplayError) -> Self {
        Self::Replay(value)
    }
}

impl From<FreshnessError> for DeliveryGuardError {
    fn from(value: FreshnessError) -> Self {
        Self::Freshness(value)
    }
}

/// Replay/freshness enforcement for a single authenticated sender/session scope.
///
/// Callers must invoke this guard only after the selected cryptographic/session
/// implementation has authenticated the message and established the epoch context.
/// This guard is not an authentication primitive and never substitutes for
/// cryptographic verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundReplayGuard {
    replay: ReplayWindow,
    freshness: FreshnessPolicy,
}

impl InboundReplayGuard {
    pub fn new(window_size: usize, freshness: FreshnessPolicy) -> Result<Self, ReplayError> {
        Ok(Self {
            replay: ReplayWindow::new(window_size)?,
            freshness,
        })
    }

    /// Accept delivery metadata after cryptographic authentication has succeeded.
    ///
    /// Freshness is evaluated before replay state is mutated.
    pub fn accept(
        &mut self,
        metadata: DeliveryMetadata,
        message_id: MessageId,
        content_binding: ContentBinding,
        now_epoch_ms: u64,
    ) -> Result<(), DeliveryGuardError> {
        self.freshness.validate(
            now_epoch_ms,
            metadata.created_at_epoch_ms,
            metadata.expires_at_epoch_ms,
        )?;
        self.replay.observe(
            metadata.epoch,
            metadata.sequence,
            message_id,
            content_binding,
        )?;
        Ok(())
    }

    pub(crate) fn advance_epoch(&mut self, new_epoch: u64) -> Result<(), ReplayError> {
        self.replay.advance_epoch(new_epoch)
    }

    pub fn replay(&self) -> &ReplayWindow {
        &self.replay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata(epoch: u64, sequence: u64) -> DeliveryMetadata {
        DeliveryMetadata::new(epoch, sequence, 10_000, Some(10_100))
    }

    fn guard() -> InboundReplayGuard {
        InboundReplayGuard::new(4, FreshnessPolicy::new(1_000, 100)).unwrap()
    }

    #[test]
    fn acceptance_is_freshness_and_replay_gated() {
        let mut guard = guard();
        assert_eq!(
            guard.accept(
                metadata(1, 1),
                MessageId::new([1; 16]),
                ContentBinding::new([1; 32]),
                10_050,
            ),
            Ok(())
        );
        assert_eq!(
            guard.accept(
                metadata(1, 1),
                MessageId::new([1; 16]),
                ContentBinding::new([1; 32]),
                10_050,
            ),
            Err(DeliveryGuardError::Replay(ReplayError::Duplicate))
        );
        assert_eq!(
            guard.accept(
                metadata(1, 1),
                MessageId::new([1; 16]),
                ContentBinding::new([2; 32]),
                10_050,
            ),
            Err(DeliveryGuardError::Replay(
                ReplayError::ContentBindingMismatch
            ))
        );
    }

    #[test]
    fn stale_delivery_is_rejected_before_replay_state_changes() {
        let mut guard = guard();
        let stale = DeliveryMetadata::new(1, 1, 8_999, Some(9_999));
        assert_eq!(
            guard.accept(
                stale,
                MessageId::new([2; 16]),
                ContentBinding::new([2; 32]),
                10_000,
            ),
            Err(DeliveryGuardError::Freshness(FreshnessError::TooOld))
        );
        assert_eq!(guard.replay().highest_sequence(), None);
    }

    #[test]
    fn epoch_transition_is_explicit() {
        let mut guard = guard();
        assert_eq!(
            guard.accept(
                metadata(1, 1),
                MessageId::new([1; 16]),
                ContentBinding::new([1; 32]),
                10_050,
            ),
            Ok(())
        );
        assert_eq!(guard.advance_epoch(2), Ok(()));
        assert_eq!(
            guard.accept(
                metadata(2, 1),
                MessageId::new([2; 16]),
                ContentBinding::new([2; 32]),
                10_050,
            ),
            Ok(())
        );
        assert_eq!(
            guard.advance_epoch(1),
            Err(ReplayError::EpochRollback)
        );
    }

}