//! Greengrassv2_2020_11_30 Service
//!
//! Auto-generated service module for greengrassv2_2020_11_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for greengrassv2_2020_11_30
pub struct Greengrassv2_2020_11_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Greengrassv2_2020_11_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get core_device resource handler
    pub fn core_device(&self) -> resources::Core_device<'_> {
        resources::Core_device::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get service_role_for_account resource handler
    pub fn service_role_for_account(&self) -> resources::Service_role_for_account<'_> {
        resources::Service_role_for_account::new(self.provider)
    }
    /// Get component_version resource handler
    pub fn component_version(&self) -> resources::Component_version<'_> {
        resources::Component_version::new(self.provider)
    }
    /// Get component_version_artifact resource handler
    pub fn component_version_artifact(&self) -> resources::Component_version_artifact<'_> {
        resources::Component_version_artifact::new(self.provider)
    }
    /// Get component resource handler
    pub fn component(&self) -> resources::Component<'_> {
        resources::Component::new(self.provider)
    }
    /// Get connectivity_info resource handler
    pub fn connectivity_info(&self) -> resources::Connectivity_info<'_> {
        resources::Connectivity_info::new(self.provider)
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
