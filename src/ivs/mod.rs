//! Ivs Service
//!
//! Auto-generated service module for ivs

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ivs
pub struct IvsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> IvsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get recording_configuration resource handler
    pub fn recording_configuration(&self) -> resources::Recording_configuration<'_> {
        resources::Recording_configuration::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get playback_restriction_policy resource handler
    pub fn playback_restriction_policy(&self) -> resources::Playback_restriction_policy<'_> {
        resources::Playback_restriction_policy::new(self.provider)
    }
    /// Get metadata resource handler
    pub fn metadata(&self) -> resources::Metadata<'_> {
        resources::Metadata::new(self.provider)
    }
    /// Get playback_key_pair resource handler
    pub fn playback_key_pair(&self) -> resources::Playback_key_pair<'_> {
        resources::Playback_key_pair::new(self.provider)
    }
    /// Get stream resource handler
    pub fn stream(&self) -> resources::Stream<'_> {
        resources::Stream::new(self.provider)
    }
    /// Get stream_key resource handler
    pub fn stream_key(&self) -> resources::Stream_key<'_> {
        resources::Stream_key::new(self.provider)
    }
    /// Get stream_session resource handler
    pub fn stream_session(&self) -> resources::Stream_session<'_> {
        resources::Stream_session::new(self.provider)
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
