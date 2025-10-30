//! Ivs_realtime_2020_07_14 Service
//!
//! Auto-generated service module for ivs_realtime_2020_07_14

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ivs_realtime_2020_07_14
pub struct Ivs_realtime_2020_07_14Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ivs_realtime_2020_07_14Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get composition resource handler
    pub fn composition(&self) -> resources::Composition<'_> {
        resources::Composition::new(self.provider)
    }
    /// Get encoder_configuration resource handler
    pub fn encoder_configuration(&self) -> resources::Encoder_configuration<'_> {
        resources::Encoder_configuration::new(self.provider)
    }
    /// Get participant_token resource handler
    pub fn participant_token(&self) -> resources::Participant_token<'_> {
        resources::Participant_token::new(self.provider)
    }
    /// Get storage_configuration resource handler
    pub fn storage_configuration(&self) -> resources::Storage_configuration<'_> {
        resources::Storage_configuration::new(self.provider)
    }
    /// Get stage resource handler
    pub fn stage(&self) -> resources::Stage<'_> {
        resources::Stage::new(self.provider)
    }
    /// Get stage_session resource handler
    pub fn stage_session(&self) -> resources::Stage_session<'_> {
        resources::Stage_session::new(self.provider)
    }
    /// Get ingest_configuration resource handler
    pub fn ingest_configuration(&self) -> resources::Ingest_configuration<'_> {
        resources::Ingest_configuration::new(self.provider)
    }
    /// Get public_key resource handler
    pub fn public_key(&self) -> resources::Public_key<'_> {
        resources::Public_key::new(self.provider)
    }
    /// Get participant resource handler
    pub fn participant(&self) -> resources::Participant<'_> {
        resources::Participant::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
