use eagle_core::{
    DeliveryMetadata, FreshnessPolicy, InboundReplayGuard, MessageId, ReplayError, ReplayWindow,
};

fn message_id(n: u64) -> MessageId {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&n.to_be_bytes());
    MessageId::new(bytes)
}

#[test]
fn replay_window_stress_preserves_window_and_duplicate_invariants() {
    let mut window = ReplayWindow::new(64).unwrap();

    for sequence in 0..4096u64 {
        assert_eq!(window.observe(1, sequence, message_id(sequence)), Ok(()));
        assert_eq!(
            window.observe(1, sequence, message_id(sequence)),
            Err(ReplayError::Duplicate)
        );

        if sequence >= 64 {
            let too_old = sequence - 64;
            assert_eq!(
                window.observe(1, too_old, message_id(too_old)),
                Err(ReplayError::TooOld)
            );
        }
    }

    assert_eq!(window.epoch(), Some(1));
    assert_eq!(window.highest_sequence(), Some(4095));
}

#[test]
fn replay_epoch_reset_is_monotonic() {
    let mut window = ReplayWindow::new(8).unwrap();
    assert_eq!(window.observe(4, 7, message_id(7)), Ok(()));
    assert_eq!(window.advance_epoch(5), Ok(()));
    assert_eq!(window.highest_sequence(), None);

    for sequence in 0..8u64 {
        assert_eq!(
            window.observe(5, sequence, message_id(100 + sequence)),
            Ok(())
        );
    }

    assert_eq!(window.advance_epoch(5), Err(ReplayError::EpochRollback));
    assert_eq!(window.advance_epoch(4), Err(ReplayError::EpochRollback));
}

#[test]
fn freshness_and_replay_are_independently_bounded() {
    let policy = FreshnessPolicy::new(1_000, 50);
    let mut guard = InboundReplayGuard::new(8, policy).unwrap();

    let metadata = DeliveryMetadata::new(1, 1, 1_000, Some(1_500));
    assert!(guard.accept(metadata, message_id(1), 1_100).is_ok());
    assert!(guard.accept(metadata, message_id(1), 1_100).is_err());

    let future = DeliveryMetadata::new(1, 2, 1_200, Some(1_500));
    assert!(guard.accept(future, message_id(2), 1_100).is_err());

    assert_eq!(guard.replay().highest_sequence(), Some(1));
}
