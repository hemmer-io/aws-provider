//! Gameliftstreams_2018_05_10 Service
//!
//! Auto-generated service module for gameliftstreams_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gameliftstreams_2018_05_10
pub struct Gameliftstreams_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Gameliftstreams_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get stream_session_connection resource handler
    pub fn stream_session_connection(&self) -> resources::Stream_session_connection<'_> {
        resources::Stream_session_connection::new(self.provider)
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
