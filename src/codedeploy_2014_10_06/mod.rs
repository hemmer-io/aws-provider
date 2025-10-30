//! Codedeploy_2014_10_06 Service
//!
//! Auto-generated service module for codedeploy_2014_10_06

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codedeploy_2014_10_06
pub struct Codedeploy_2014_10_06Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Codedeploy_2014_10_06Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get lifecycle_event_hook_execution_status resource handler
    pub fn lifecycle_event_hook_execution_status(&self) -> resources::Lifecycle_event_hook_execution_status<'_> {
        resources::Lifecycle_event_hook_execution_status::new(self.provider)
    }
    /// Get deployment_group resource handler
    pub fn deployment_group(&self) -> resources::Deployment_group<'_> {
        resources::Deployment_group::new(self.provider)
    }
    /// Get deployment_config resource handler
    pub fn deployment_config(&self) -> resources::Deployment_config<'_> {
        resources::Deployment_config::new(self.provider)
    }
    /// Get deployment_target resource handler
    pub fn deployment_target(&self) -> resources::Deployment_target<'_> {
        resources::Deployment_target::new(self.provider)
    }
    /// Get git_hub_account_token resource handler
    pub fn git_hub_account_token(&self) -> resources::Git_hub_account_token<'_> {
        resources::Git_hub_account_token::new(self.provider)
    }
    /// Get deployment_instance resource handler
    pub fn deployment_instance(&self) -> resources::Deployment_instance<'_> {
        resources::Deployment_instance::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get resources_by_external_id resource handler
    pub fn resources_by_external_id(&self) -> resources::Resources_by_external_id<'_> {
        resources::Resources_by_external_id::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get application_revision resource handler
    pub fn application_revision(&self) -> resources::Application_revision<'_> {
        resources::Application_revision::new(self.provider)
    }
    /// Get on_premises_instance resource handler
    pub fn on_premises_instance(&self) -> resources::On_premises_instance<'_> {
        resources::On_premises_instance::new(self.provider)
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
