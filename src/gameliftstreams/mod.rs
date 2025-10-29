//! Gameliftstreams Service
//!
//! Auto-generated service module for gameliftstreams

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gameliftstreams
pub struct GameliftstreamsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> GameliftstreamsService<'a> {
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
