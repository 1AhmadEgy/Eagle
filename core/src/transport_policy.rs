#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportPath {
    Direct,
    Relay,
    ServerFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerBinding {
    Unauthenticated,
    EagleDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    ApplicationDataRequiresDirectPath,
    PeerIdentityRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportPolicy {
    application_data_direct_only: bool,
    require_peer_identity_binding: bool,
}

impl Default for TransportPolicy {
    fn default() -> Self {
        Self {
            application_data_direct_only: true,
            require_peer_identity_binding: true,
        }
    }
}

impl TransportPolicy {
    pub const fn new() -> Self {
        Self {
            application_data_direct_only: true,
            require_peer_identity_binding: true,
        }
    }

    pub const fn application_data_direct_only(&self) -> bool {
        self.application_data_direct_only
    }

    pub const fn require_peer_identity_binding(&self) -> bool {
        self.require_peer_identity_binding
    }

    pub fn authorize_application_data(
        &self,
        path: TransportPath,
        peer: PeerBinding,
    ) -> Result<(), TransportError> {
        if self.application_data_direct_only && path != TransportPath::Direct {
            return Err(TransportError::ApplicationDataRequiresDirectPath);
        }

        if self.require_peer_identity_binding && peer != PeerBinding::EagleDevice {
            return Err(TransportError::PeerIdentityRequired);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_direct_only_and_identity_bound() {
        let policy = TransportPolicy::new();
        assert!(policy.application_data_direct_only());
        assert!(policy.require_peer_identity_binding());

        assert_eq!(
            policy.authorize_application_data(TransportPath::Direct, PeerBinding::EagleDevice,),
            Ok(())
        );
    }

    #[test]
    fn relay_and_server_fallback_are_rejected_for_application_data() {
        let policy = TransportPolicy::new();

        assert_eq!(
            policy.authorize_application_data(TransportPath::Relay, PeerBinding::EagleDevice,),
            Err(TransportError::ApplicationDataRequiresDirectPath)
        );
        assert_eq!(
            policy.authorize_application_data(
                TransportPath::ServerFallback,
                PeerBinding::EagleDevice,
            ),
            Err(TransportError::ApplicationDataRequiresDirectPath)
        );
    }

    #[test]
    fn unauthenticated_direct_peer_is_rejected() {
        let policy = TransportPolicy::new();

        assert_eq!(
            policy.authorize_application_data(TransportPath::Direct, PeerBinding::Unauthenticated,),
            Err(TransportError::PeerIdentityRequired)
        );
    }
}
