//! Kinesis Service
//!
//! Auto-generated service module for kinesis

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kinesis
pub struct KinesisService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> KinesisService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get media_for_fragment_list resource handler
    pub fn media_for_fragment_list(&self) -> resources::Media_for_fragment_list<'_> {
        resources::Media_for_fragment_list::new(self.provider)
    }
    /// Get hls_streaming_session_url resource handler
    pub fn hls_streaming_session_url(&self) -> resources::Hls_streaming_session_url<'_> {
        resources::Hls_streaming_session_url::new(self.provider)
    }
    /// Get clip resource handler
    pub fn clip(&self) -> resources::Clip<'_> {
        resources::Clip::new(self.provider)
    }
    /// Get images resource handler
    pub fn images(&self) -> resources::Images<'_> {
        resources::Images::new(self.provider)
    }
    /// Get dash_streaming_session_url resource handler
    pub fn dash_streaming_session_url(&self) -> resources::Dash_streaming_session_url<'_> {
        resources::Dash_streaming_session_url::new(self.provider)
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
