//! Ssm Service
//!
//! Auto-generated service module for ssm

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ssm
pub struct SsmService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SsmService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get configuration_definition resource handler
    pub fn configuration_definition(&self) -> resources::Configuration_definition<'_> {
        resources::Configuration_definition::new(self.provider)
    }
    /// Get service_settings resource handler
    pub fn service_settings(&self) -> resources::Service_settings<'_> {
        resources::Service_settings::new(self.provider)
    }
    /// Get configuration_manager resource handler
    pub fn configuration_manager(&self) -> resources::Configuration_manager<'_> {
        resources::Configuration_manager::new(self.provider)
    }
    /// Get configuration resource handler
    pub fn configuration(&self) -> resources::Configuration<'_> {
        resources::Configuration::new(self.provider)
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
