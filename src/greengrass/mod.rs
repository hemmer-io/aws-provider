//! Greengrass Service
//!
//! Auto-generated service module for greengrass

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for greengrass
pub struct GreengrassService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> GreengrassService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get device_definition_version resource handler
    pub fn device_definition_version(&self) -> resources::Device_definition_version<'_> {
        resources::Device_definition_version::new(self.provider)
    }
    /// Get resource_definition_version resource handler
    pub fn resource_definition_version(&self) -> resources::Resource_definition_version<'_> {
        resources::Resource_definition_version::new(self.provider)
    }
    /// Get deployment_status resource handler
    pub fn deployment_status(&self) -> resources::Deployment_status<'_> {
        resources::Deployment_status::new(self.provider)
    }
    /// Get group_certificate_configuration resource handler
    pub fn group_certificate_configuration(&self) -> resources::Group_certificate_configuration<'_> {
        resources::Group_certificate_configuration::new(self.provider)
    }
    /// Get logger_definition_version resource handler
    pub fn logger_definition_version(&self) -> resources::Logger_definition_version<'_> {
        resources::Logger_definition_version::new(self.provider)
    }
    /// Get function_definition_version resource handler
    pub fn function_definition_version(&self) -> resources::Function_definition_version<'_> {
        resources::Function_definition_version::new(self.provider)
    }
    /// Get resource_definition resource handler
    pub fn resource_definition(&self) -> resources::Resource_definition<'_> {
        resources::Resource_definition::new(self.provider)
    }
    /// Get function_definition resource handler
    pub fn function_definition(&self) -> resources::Function_definition<'_> {
        resources::Function_definition::new(self.provider)
    }
    /// Get core_definition resource handler
    pub fn core_definition(&self) -> resources::Core_definition<'_> {
        resources::Core_definition::new(self.provider)
    }
    /// Get thing_runtime_configuration resource handler
    pub fn thing_runtime_configuration(&self) -> resources::Thing_runtime_configuration<'_> {
        resources::Thing_runtime_configuration::new(self.provider)
    }
    /// Get device_definition resource handler
    pub fn device_definition(&self) -> resources::Device_definition<'_> {
        resources::Device_definition::new(self.provider)
    }
    /// Get connectivity_info resource handler
    pub fn connectivity_info(&self) -> resources::Connectivity_info<'_> {
        resources::Connectivity_info::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get group_version resource handler
    pub fn group_version(&self) -> resources::Group_version<'_> {
        resources::Group_version::new(self.provider)
    }
    /// Get service_role_for_account resource handler
    pub fn service_role_for_account(&self) -> resources::Service_role_for_account<'_> {
        resources::Service_role_for_account::new(self.provider)
    }
    /// Get subscription_definition resource handler
    pub fn subscription_definition(&self) -> resources::Subscription_definition<'_> {
        resources::Subscription_definition::new(self.provider)
    }
    /// Get group_certificate_authority resource handler
    pub fn group_certificate_authority(&self) -> resources::Group_certificate_authority<'_> {
        resources::Group_certificate_authority::new(self.provider)
    }
    /// Get connector_definition resource handler
    pub fn connector_definition(&self) -> resources::Connector_definition<'_> {
        resources::Connector_definition::new(self.provider)
    }
    /// Get software_update_job resource handler
    pub fn software_update_job(&self) -> resources::Software_update_job<'_> {
        resources::Software_update_job::new(self.provider)
    }
    /// Get associated_role resource handler
    pub fn associated_role(&self) -> resources::Associated_role<'_> {
        resources::Associated_role::new(self.provider)
    }
    /// Get connector_definition_version resource handler
    pub fn connector_definition_version(&self) -> resources::Connector_definition_version<'_> {
        resources::Connector_definition_version::new(self.provider)
    }
    /// Get logger_definition resource handler
    pub fn logger_definition(&self) -> resources::Logger_definition<'_> {
        resources::Logger_definition::new(self.provider)
    }
    /// Get bulk_deployment_status resource handler
    pub fn bulk_deployment_status(&self) -> resources::Bulk_deployment_status<'_> {
        resources::Bulk_deployment_status::new(self.provider)
    }
    /// Get core_definition_version resource handler
    pub fn core_definition_version(&self) -> resources::Core_definition_version<'_> {
        resources::Core_definition_version::new(self.provider)
    }
    /// Get subscription_definition_version resource handler
    pub fn subscription_definition_version(&self) -> resources::Subscription_definition_version<'_> {
        resources::Subscription_definition_version::new(self.provider)
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
