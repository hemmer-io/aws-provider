//! License_manager_2018_08_01 Service
//!
//! Auto-generated service module for license_manager_2018_08_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for license_manager_2018_08_01
pub struct License_manager_2018_08_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> License_manager_2018_08_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get license_conversion_task_for_resource resource handler
    pub fn license_conversion_task_for_resource(&self) -> resources::License_conversion_task_for_resource<'_> {
        resources::License_conversion_task_for_resource::new(self.provider)
    }
    /// Get license_version resource handler
    pub fn license_version(&self) -> resources::License_version<'_> {
        resources::License_version::new(self.provider)
    }
    /// Get token resource handler
    pub fn token(&self) -> resources::Token<'_> {
        resources::Token::new(self.provider)
    }
    /// Get license_manager_report_generator resource handler
    pub fn license_manager_report_generator(&self) -> resources::License_manager_report_generator<'_> {
        resources::License_manager_report_generator::new(self.provider)
    }
    /// Get license resource handler
    pub fn license(&self) -> resources::License<'_> {
        resources::License::new(self.provider)
    }
    /// Get grant resource handler
    pub fn grant(&self) -> resources::Grant<'_> {
        resources::Grant::new(self.provider)
    }
    /// Get service_settings resource handler
    pub fn service_settings(&self) -> resources::Service_settings<'_> {
        resources::Service_settings::new(self.provider)
    }
    /// Get license_usage resource handler
    pub fn license_usage(&self) -> resources::License_usage<'_> {
        resources::License_usage::new(self.provider)
    }
    /// Get license_specifications_for_resource resource handler
    pub fn license_specifications_for_resource(&self) -> resources::License_specifications_for_resource<'_> {
        resources::License_specifications_for_resource::new(self.provider)
    }
    /// Get license_conversion_task resource handler
    pub fn license_conversion_task(&self) -> resources::License_conversion_task<'_> {
        resources::License_conversion_task::new(self.provider)
    }
    /// Get grant_version resource handler
    pub fn grant_version(&self) -> resources::Grant_version<'_> {
        resources::Grant_version::new(self.provider)
    }
    /// Get access_token resource handler
    pub fn access_token(&self) -> resources::Access_token<'_> {
        resources::Access_token::new(self.provider)
    }
    /// Get license_configuration resource handler
    pub fn license_configuration(&self) -> resources::License_configuration<'_> {
        resources::License_configuration::new(self.provider)
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
