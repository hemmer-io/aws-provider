//! Amp Service
//!
//! Auto-generated service module for amp

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for amp
pub struct AmpService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AmpService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get default_scraper_configuration resource handler
    pub fn default_scraper_configuration(&self) -> resources::Default_scraper_configuration<'_> {
        resources::Default_scraper_configuration::new(self.provider)
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
