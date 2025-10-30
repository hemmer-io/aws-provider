//! Polly_2016_06_10 Service
//!
//! Auto-generated service module for polly_2016_06_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for polly_2016_06_10
pub struct Polly_2016_06_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Polly_2016_06_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get speech_synthesis_task resource handler
    pub fn speech_synthesis_task(&self) -> resources::Speech_synthesis_task<'_> {
        resources::Speech_synthesis_task::new(self.provider)
    }
    /// Get lexicon resource handler
    pub fn lexicon(&self) -> resources::Lexicon<'_> {
        resources::Lexicon::new(self.provider)
    }
    /// Get voices resource handler
    pub fn voices(&self) -> resources::Voices<'_> {
        resources::Voices::new(self.provider)
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
