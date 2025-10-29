//! Transcribe Service
//!
//! Auto-generated service module for transcribe

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for transcribe
pub struct TranscribeService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> TranscribeService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get medical_scribe_stream resource handler
    pub fn medical_scribe_stream(&self) -> resources::Medical_scribe_stream<'_> {
        resources::Medical_scribe_stream::new(self.provider)
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
