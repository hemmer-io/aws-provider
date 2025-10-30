//! Amp_2020_08_01 Service
//!
//! Auto-generated service module for amp_2020_08_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for amp_2020_08_01
pub struct Amp_2020_08_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Amp_2020_08_01Service<'a> {
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
