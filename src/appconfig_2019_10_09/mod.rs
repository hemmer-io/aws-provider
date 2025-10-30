//! Appconfig_2019_10_09 Service
//!
//! Auto-generated service module for appconfig_2019_10_09

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appconfig_2019_10_09
pub struct Appconfig_2019_10_09Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Appconfig_2019_10_09Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get account_settings resource handler
    pub fn account_settings(&self) -> resources::Account_settings<'_> {
        resources::Account_settings::new(self.provider)
    }
    /// Get configuration resource handler
    pub fn configuration(&self) -> resources::Configuration<'_> {
        resources::Configuration::new(self.provider)
    }
    /// Get hosted_configuration_version resource handler
    pub fn hosted_configuration_version(&self) -> resources::Hosted_configuration_version<'_> {
        resources::Hosted_configuration_version::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get extension resource handler
    pub fn extension(&self) -> resources::Extension<'_> {
        resources::Extension::new(self.provider)
    }
    /// Get configuration_profile resource handler
    pub fn configuration_profile(&self) -> resources::Configuration_profile<'_> {
        resources::Configuration_profile::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get extension_association resource handler
    pub fn extension_association(&self) -> resources::Extension_association<'_> {
        resources::Extension_association::new(self.provider)
    }
    /// Get deployment_strategy resource handler
    pub fn deployment_strategy(&self) -> resources::Deployment_strategy<'_> {
        resources::Deployment_strategy::new(self.provider)
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
