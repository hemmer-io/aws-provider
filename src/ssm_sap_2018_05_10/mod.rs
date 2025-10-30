//! Ssm_sap_2018_05_10 Service
//!
//! Auto-generated service module for ssm_sap_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ssm_sap_2018_05_10
pub struct Ssm_sap_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ssm_sap_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get component resource handler
    pub fn component(&self) -> resources::Component<'_> {
        resources::Component::new(self.provider)
    }
    /// Get configuration_check_operation resource handler
    pub fn configuration_check_operation(&self) -> resources::Configuration_check_operation<'_> {
        resources::Configuration_check_operation::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get application_settings resource handler
    pub fn application_settings(&self) -> resources::Application_settings<'_> {
        resources::Application_settings::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get resource_permission resource handler
    pub fn resource_permission(&self) -> resources::Resource_permission<'_> {
        resources::Resource_permission::new(self.provider)
    }
    /// Get database resource handler
    pub fn database(&self) -> resources::Database<'_> {
        resources::Database::new(self.provider)
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
