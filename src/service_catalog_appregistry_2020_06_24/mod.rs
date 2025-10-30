//! Service_catalog_appregistry_2020_06_24 Service
//!
//! Auto-generated service module for service_catalog_appregistry_2020_06_24

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for service_catalog_appregistry_2020_06_24
pub struct Service_catalog_appregistry_2020_06_24Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Service_catalog_appregistry_2020_06_24Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get attribute_group resource handler
    pub fn attribute_group(&self) -> resources::Attribute_group<'_> {
        resources::Attribute_group::new(self.provider)
    }
    /// Get associated_resource resource handler
    pub fn associated_resource(&self) -> resources::Associated_resource<'_> {
        resources::Associated_resource::new(self.provider)
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
