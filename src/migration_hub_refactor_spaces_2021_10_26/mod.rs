//! Migration_hub_refactor_spaces_2021_10_26 Service
//!
//! Auto-generated service module for migration_hub_refactor_spaces_2021_10_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for migration_hub_refactor_spaces_2021_10_26
pub struct Migration_hub_refactor_spaces_2021_10_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Migration_hub_refactor_spaces_2021_10_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get route resource handler
    pub fn route(&self) -> resources::Route<'_> {
        resources::Route::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
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
