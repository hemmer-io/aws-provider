//! Elastic_beanstalk_2010_12_01 Service
//!
//! Auto-generated service module for elastic_beanstalk_2010_12_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for elastic_beanstalk_2010_12_01
pub struct Elastic_beanstalk_2010_12_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Elastic_beanstalk_2010_12_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get platform_version resource handler
    pub fn platform_version(&self) -> resources::Platform_version<'_> {
        resources::Platform_version::new(self.provider)
    }
    /// Get configuration_template resource handler
    pub fn configuration_template(&self) -> resources::Configuration_template<'_> {
        resources::Configuration_template::new(self.provider)
    }
    /// Get applications resource handler
    pub fn applications(&self) -> resources::Applications<'_> {
        resources::Applications::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get environments resource handler
    pub fn environments(&self) -> resources::Environments<'_> {
        resources::Environments::new(self.provider)
    }
    /// Get application_versions resource handler
    pub fn application_versions(&self) -> resources::Application_versions<'_> {
        resources::Application_versions::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get environment_configuration resource handler
    pub fn environment_configuration(&self) -> resources::Environment_configuration<'_> {
        resources::Environment_configuration::new(self.provider)
    }
    /// Get configuration_settings resource handler
    pub fn configuration_settings(&self) -> resources::Configuration_settings<'_> {
        resources::Configuration_settings::new(self.provider)
    }
    /// Get account_attributes resource handler
    pub fn account_attributes(&self) -> resources::Account_attributes<'_> {
        resources::Account_attributes::new(self.provider)
    }
    /// Get application_version resource handler
    pub fn application_version(&self) -> resources::Application_version<'_> {
        resources::Application_version::new(self.provider)
    }
    /// Get environment_managed_action_history resource handler
    pub fn environment_managed_action_history(&self) -> resources::Environment_managed_action_history<'_> {
        resources::Environment_managed_action_history::new(self.provider)
    }
    /// Get configuration_options resource handler
    pub fn configuration_options(&self) -> resources::Configuration_options<'_> {
        resources::Configuration_options::new(self.provider)
    }
    /// Get storage_location resource handler
    pub fn storage_location(&self) -> resources::Storage_location<'_> {
        resources::Storage_location::new(self.provider)
    }
    /// Get environment_managed_actions resource handler
    pub fn environment_managed_actions(&self) -> resources::Environment_managed_actions<'_> {
        resources::Environment_managed_actions::new(self.provider)
    }
    /// Get instances_health resource handler
    pub fn instances_health(&self) -> resources::Instances_health<'_> {
        resources::Instances_health::new(self.provider)
    }
    /// Get tags_for_resource resource handler
    pub fn tags_for_resource(&self) -> resources::Tags_for_resource<'_> {
        resources::Tags_for_resource::new(self.provider)
    }
    /// Get environment_resources resource handler
    pub fn environment_resources(&self) -> resources::Environment_resources<'_> {
        resources::Environment_resources::new(self.provider)
    }
    /// Get environment_health resource handler
    pub fn environment_health(&self) -> resources::Environment_health<'_> {
        resources::Environment_health::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get application_resource_lifecycle resource handler
    pub fn application_resource_lifecycle(&self) -> resources::Application_resource_lifecycle<'_> {
        resources::Application_resource_lifecycle::new(self.provider)
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
