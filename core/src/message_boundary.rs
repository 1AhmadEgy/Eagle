use std::vec::Vec;

use crate::{
    serialize_envelope, EncryptedEnvelope, PeerBinding, ReplayError, ReplayTracker,
    SecurityContext, SerializationError, TransportError, TransportPath, TransportPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoundaryError {
    Unauthorized,
    Transport(TransportError),
    Serialization(SerializationError),
    Replay(ReplayError),
}

impl From<TransportError> for MessageBoundaryError {
    fn from(value: TransportError) -> Self {
        Self::Transport(value)
    }
}

impl From<SerializationError> for MessageBoundaryError {
    fn from(value: SerializationError) -> Self {
        Self::Serialization(value)
    }
}

impl From<ReplayError> for MessageBoundaryError {
    fn from(value: ReplayError) -> Self {
        Self::Replay(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageBoundary {
    transport: TransportPolicy,
}

impl MessageBoundary {
    pub const fn new() -> Self {
        Self {
            transport: TransportPolicy::new(),
        }
    }

    pub fn prepare_outbound(
        &self,
        context: &SecurityContext,
        envelope: &EncryptedEnvelope,
        path: TransportPath,
        peer: PeerBinding,
    ) -> Result<Vec<u8>, MessageBoundaryError> {
        context
            .authorize()
            .map_err(|_| MessageBoundaryError::Unauthorized)?;

        self.transport.authorize_application_data(path, peer)?;

        serialize_envelope(envelope).map_err(Into::into)
    }

    fn authorize_inbound(
        &self,
        context: &SecurityContext,
        path: TransportPath,
        peer: PeerBinding,
    ) -> Result<(), MessageBoundaryError> {
        context
            .authorize()
            .map_err(|_| MessageBoundaryError::Unauthorized)?;

        self.transport
            .authorize_application_data(path, peer)
            .map_err(Into::into)
    }

    pub fn accept_inbound_bytes(
        &self,
        context: &SecurityContext,
        path: TransportPath,
        peer: PeerBinding,
        bytes: &[u8],
        replay: &mut ReplayTracker,
    ) -> Result<EncryptedEnvelope, MessageBoundaryError> {
        self.authorize_inbound(context, path, peer)?;

        let envelope = crate::deserialize_envelope(bytes)?;

        self.authorize_inbound_envelope(context, path, peer, &envelope, replay)?;

        Ok(envelope)
    }

    pub fn authorize_inbound_envelope(
        &self,
        context: &SecurityContext,
        path: TransportPath,
        peer: PeerBinding,
        envelope: &EncryptedEnvelope,
        replay: &mut ReplayTracker,
    ) -> Result<(), MessageBoundaryError> {
        self.authorize_inbound(context, path, peer)?;

        envelope
            .validate(context.negotiated_protocol())
            .map_err(|_| {
                MessageBoundaryError::Serialization(SerializationError::InvalidEnvelope)
            })?;

        replay.accept(envelope.message_id().clone())?;
        Ok(())
    }
}

impl Default for MessageBoundary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MessageId, OpaqueId, CURRENT_PROTOCOL_VERSION};

    fn envelope() -> EncryptedEnvelope {
        EncryptedEnvelope::new(
            MessageId::new([1; 16]),
            OpaqueId::new(vec![2; 8]).unwrap(),
            OpaqueId::new(vec![3; 8]).unwrap(),
            None,
            vec![0xAA; 32],
            CURRENT_PROTOCOL_VERSION,
            42,
        )
        .unwrap()
    }

    fn established_context() -> SecurityContext {
        let mut context = SecurityContext::new(1, 1).unwrap();
        context.begin_authentication().unwrap();
        context.accept_verified_authentication().unwrap();
        context.establish().unwrap();
        context
    }

    #[test]
    fn outbound_requires_authorized_direct_identity_bound_path() {
        let boundary = MessageBoundary::new();
        let context = established_context();

        let bytes = boundary
            .prepare_outbound(
                &context,
                &envelope(),
                TransportPath::Direct,
                PeerBinding::EagleDevice,
            )
            .unwrap();

        assert_eq!(crate::deserialize_envelope(&bytes).unwrap(), envelope());
    }

    #[test]
    fn outbound_rejects_relay_before_serialization() {
        let boundary = MessageBoundary::new();
        let context = established_context();

        assert_eq!(
            boundary.prepare_outbound(
                &context,
                &envelope(),
                TransportPath::Relay,
                PeerBinding::EagleDevice,
            ),
            Err(MessageBoundaryError::Transport(
                TransportError::ApplicationDataRequiresDirectPath
            ))
        );
    }

    #[test]
    fn inbound_bytes_are_parsed_only_inside_the_boundary() {
        let boundary = MessageBoundary::new();
        let context = established_context();
        let envelope = envelope();
        let bytes = crate::serialize_envelope(&envelope).unwrap();
        let mut replay = ReplayTracker::new();

        assert_eq!(
            boundary
                .accept_inbound_bytes(
                    &context,
                    TransportPath::Direct,
                    PeerBinding::EagleDevice,
                    &bytes,
                    &mut replay,
                )
                .unwrap(),
            envelope
        );

        let mut malformed = bytes.clone();
        malformed.push(0x00);
        assert!(matches!(
            boundary.accept_inbound_bytes(
                &context,
                TransportPath::Direct,
                PeerBinding::EagleDevice,
                &malformed,
                &mut replay,
            ),
            Err(MessageBoundaryError::Serialization(
                SerializationError::TrailingData
            ))
        ));
    }

    #[test]
    fn inbound_envelope_rejects_duplicate_message_ids() {
        let boundary = MessageBoundary::new();
        let context = established_context();
        let envelope = envelope();
        let mut replay = ReplayTracker::new();

        assert_eq!(
            boundary.authorize_inbound_envelope(
                &context,
                TransportPath::Direct,
                PeerBinding::EagleDevice,
                &envelope,
                &mut replay,
            ),
            Ok(())
        );
        assert_eq!(
            boundary.authorize_inbound_envelope(
                &context,
                TransportPath::Direct,
                PeerBinding::EagleDevice,
                &envelope,
                &mut replay,
            ),
            Err(MessageBoundaryError::Replay(ReplayError::DuplicateMessage))
        );
    }

    #[test]
    fn inbound_rejects_unbound_peer_and_untrusted_context() {
        let boundary = MessageBoundary::new();
        let context = SecurityContext::new(1, 1).unwrap();

        assert_eq!(
            boundary.authorize_inbound(&context, TransportPath::Direct, PeerBinding::EagleDevice,),
            Err(MessageBoundaryError::Unauthorized)
        );

        let context = established_context();
        assert_eq!(
            boundary.authorize_inbound(
                &context,
                TransportPath::Direct,
                PeerBinding::Unauthenticated,
            ),
            Err(MessageBoundaryError::Transport(
                TransportError::PeerIdentityRequired
            ))
        );
    }
}
