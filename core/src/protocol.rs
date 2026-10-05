    pub fn payload_len(&self) -> u32 {
        self.payload_len
    }

    pub fn flags(&self) -> u16 {
        self.flags
    }

    pub fn validate_version(&self, minimum_version: u16) -> Result<u16, ProtocolError> {
        validate_version(self.protocol_version, minimum_version)
    }

    pub fn validate_payload_len(&self, actual_len: usize) -> Result<(), ProtocolError> {
        if actual_len > MAX_PAYLOAD_BYTES {
            return Err(ProtocolError::PayloadTooLarge);
        }
        if usize::try_from(self.payload_len).ok() != Some(actual_len) {
            return Err(ProtocolError::PayloadLengthMismatch);
        }
        Ok(())
    }
}

pub fn validate_version(offered: u16, minimum_version: u16) -> Result<u16, ProtocolError> {
    if offered < minimum_version || offered < CURRENT_PROTOCOL_VERSION {
        return Err(ProtocolError::DowngradeRejected);
    }
    if offered > CURRENT_PROTOCOL_VERSION {
        return Err(ProtocolError::UnsupportedVersion);
    }
    Ok(offered)
}

fn validate_id(id: &OpaqueId) -> Result<(), ProtocolError> {
    if id.0.is_empty() {
        return Err(ProtocolError::EmptyIdentifier);
    }
    if id.0.len() > MAX_ID_BYTES {
        return Err(ProtocolError::IdentifierTooLarge);
    }
    Ok(())
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SendSequence {
    next: u64,
}

#[cfg(test)]
impl SendSequence {
    pub const fn new() -> Self {
        Self { next: 0 }
    }

    pub fn next(&mut self) -> Result<u64, ProtocolError> {
        let current = self.next;
        self.next = self
            .next
            .checked_add(1)
            .ok_or(ProtocolError::SequenceExhausted)?;
        Ok(current)
    }
}

#[cfg(test)]
impl Default for SendSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiveSequenceWindow {
    highest: Option<u64>,
    bitmap: u64,
}

#[cfg(test)]
impl ReceiveSequenceWindow {
    pub const WINDOW_BITS: u32 = 64;

    pub const fn new() -> Self {
        Self {
            highest: None,
            bitmap: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn accept_authenticated(&mut self, sequence: u64) -> Result<(), ProtocolError> {
        match self.highest {
            None => {
                self.highest = Some(sequence);
                self.bitmap = 1;
                Ok(())
            }
            Some(highest) if sequence > highest => {
                let shift = sequence - highest;
                self.highest = Some(sequence);
                self.bitmap = if shift >= Self::WINDOW_BITS as u64 {
                    1
                } else {
                    (self.bitmap << shift) | 1
                };
                Ok(())
            }
            Some(highest) => {
                let age = highest - sequence;
                if age >= Self::WINDOW_BITS as u64 {
                    return Err(ProtocolError::SequenceTooOld);
                }
                let bit = 1u64 << age;
                if self.bitmap & bit != 0 {
                    return Err(ProtocolError::DuplicateSequence);
                }
                self.bitmap |= bit;
                Ok(())
            }
        }
    }
}
