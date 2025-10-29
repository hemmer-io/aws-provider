//! Device Service
//!
//! Auto-generated service module for device

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for device
pub struct DeviceService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> DeviceService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get device_pool resource handler
    pub fn device_pool(&self) -> resources::Device_pool<'_> {
        resources::Device_pool::new(self.provider)
    }
    /// Get test resource handler
    pub fn test(&self) -> resources::Test<'_> {
        resources::Test::new(self.provider)
    }
    /// Get run resource handler
    pub fn run(&self) -> resources::Run<'_> {
        resources::Run::new(self.provider)
    }
    /// Get device_pool_compatibility resource handler
    pub fn device_pool_compatibility(&self) -> resources::Device_pool_compatibility<'_> {
        resources::Device_pool_compatibility::new(self.provider)
    }
    /// Get network_profile resource handler
    pub fn network_profile(&self) -> resources::Network_profile<'_> {
        resources::Network_profile::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get test_grid_project resource handler
    pub fn test_grid_project(&self) -> resources::Test_grid_project<'_> {
        resources::Test_grid_project::new(self.provider)
    }
    /// Get vpceconfiguration resource handler
    pub fn vpceconfiguration(&self) -> resources::Vpceconfiguration<'_> {
        resources::Vpceconfiguration::new(self.provider)
    }
    /// Get device_instance resource handler
    pub fn device_instance(&self) -> resources::Device_instance<'_> {
        resources::Device_instance::new(self.provider)
    }
    /// Get instance_profile resource handler
    pub fn instance_profile(&self) -> resources::Instance_profile<'_> {
        resources::Instance_profile::new(self.provider)
    }
    /// Get account_settings resource handler
    pub fn account_settings(&self) -> resources::Account_settings<'_> {
        resources::Account_settings::new(self.provider)
    }
    /// Get test_grid_url resource handler
    pub fn test_grid_url(&self) -> resources::Test_grid_url<'_> {
        resources::Test_grid_url::new(self.provider)
    }
    /// Get remote_access_session resource handler
    pub fn remote_access_session(&self) -> resources::Remote_access_session<'_> {
        resources::Remote_access_session::new(self.provider)
    }
    /// Get upload resource handler
    pub fn upload(&self) -> resources::Upload<'_> {
        resources::Upload::new(self.provider)
    }
    /// Get offering_status resource handler
    pub fn offering_status(&self) -> resources::Offering_status<'_> {
        resources::Offering_status::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get suite resource handler
    pub fn suite(&self) -> resources::Suite<'_> {
        resources::Suite::new(self.provider)
    }
    /// Get test_grid_session resource handler
    pub fn test_grid_session(&self) -> resources::Test_grid_session<'_> {
        resources::Test_grid_session::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
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
