use std::collections::VecDeque;

use crate::MessageId;

pub const MAX_TRACKED_MESSAGE_IDS: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayError {
    DuplicateMessage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayTracker {
    recent: VecDeque<MessageId>,
}

impl ReplayTracker {
    pub fn new() -> Self {
        Self {
            recent: VecDeque::with_capacity(MAX_TRACKED_MESSAGE_IDS),
        }
    }

    pub fn len(&self) -> usize {
        self.recent.len()
    }

    pub fn is_empty(&self) -> bool {
        self.recent.is_empty()
    }

    pub fn accept(&mut self, message_id: MessageId) -> Result<(), ReplayError> {
        if self.recent.iter().any(|known| known == &message_id) {
            return Err(ReplayError::DuplicateMessage);
        }

        if self.recent.len() == MAX_TRACKED_MESSAGE_IDS {
            let _ = self.recent.pop_front();
        }
        self.recent.push_back(message_id);
        Ok(())
    }
}

impl Default for ReplayTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message_id(value: u16) -> MessageId {
        let mut bytes = [0u8; 16];
        bytes[0] = (value & 0xff) as u8;
        bytes[1] = (value >> 8) as u8;
        MessageId::new(bytes)
    }

    #[test]
    fn duplicate_message_is_rejected() {
        let mut tracker = ReplayTracker::new();
        assert_eq!(tracker.accept(message_id(1)), Ok(()));
        assert_eq!(
            tracker.accept(message_id(1)),
            Err(ReplayError::DuplicateMessage)
        );
        assert_eq!(tracker.len(), 1);
    }

    #[test]
    fn tracker_is_bounded() {
        let mut tracker = ReplayTracker::new();
        for value in 0..=255u16 {
            assert_eq!(tracker.accept(message_id(value)), Ok(()));
        }
        assert_eq!(tracker.len(), 256);
    }

    #[test]
    fn oldest_entries_are_evicted_only_after_capacity_is_reached() {
        let mut tracker = ReplayTracker::new();
        for value in 0..MAX_TRACKED_MESSAGE_IDS as u16 {
            assert_eq!(tracker.accept(message_id(value)), Ok(()));
        }

        assert_eq!(
            tracker.accept(message_id(0)),
            Err(ReplayError::DuplicateMessage)
        );
        assert_eq!(
            tracker.accept(message_id(250)),
            Err(ReplayError::DuplicateMessage)
        );
        assert_eq!(
            tracker.accept(message_id(MAX_TRACKED_MESSAGE_IDS as u16)),
            Ok(())
        );
        assert_eq!(tracker.len(), MAX_TRACKED_MESSAGE_IDS);
        assert_eq!(tracker.accept(message_id(0)), Ok(()));
    }
}
