//! Sso_admin_2020_07_20 Service
//!
//! Auto-generated service module for sso_admin_2020_07_20

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sso_admin_2020_07_20
pub struct Sso_admin_2020_07_20Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sso_admin_2020_07_20Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get account_assignment resource handler
    pub fn account_assignment(&self) -> resources::Account_assignment<'_> {
        resources::Account_assignment::new(self.provider)
    }
    /// Get inline_policy_from_permission_set resource handler
    pub fn inline_policy_from_permission_set(&self) -> resources::Inline_policy_from_permission_set<'_> {
        resources::Inline_policy_from_permission_set::new(self.provider)
    }
    /// Get permission_set resource handler
    pub fn permission_set(&self) -> resources::Permission_set<'_> {
        resources::Permission_set::new(self.provider)
    }
    /// Get trusted_token_issuer resource handler
    pub fn trusted_token_issuer(&self) -> resources::Trusted_token_issuer<'_> {
        resources::Trusted_token_issuer::new(self.provider)
    }
    /// Get inline_policy_for_permission_set resource handler
    pub fn inline_policy_for_permission_set(&self) -> resources::Inline_policy_for_permission_set<'_> {
        resources::Inline_policy_for_permission_set::new(self.provider)
    }
    /// Get application_session_configuration resource handler
    pub fn application_session_configuration(&self) -> resources::Application_session_configuration<'_> {
        resources::Application_session_configuration::new(self.provider)
    }
    /// Get application_assignment resource handler
    pub fn application_assignment(&self) -> resources::Application_assignment<'_> {
        resources::Application_assignment::new(self.provider)
    }
    /// Get application_assignment_configuration resource handler
    pub fn application_assignment_configuration(&self) -> resources::Application_assignment_configuration<'_> {
        resources::Application_assignment_configuration::new(self.provider)
    }
    /// Get account_assignment_deletion_status resource handler
    pub fn account_assignment_deletion_status(&self) -> resources::Account_assignment_deletion_status<'_> {
        resources::Account_assignment_deletion_status::new(self.provider)
    }
    /// Get permissions_boundary_from_permission_set resource handler
    pub fn permissions_boundary_from_permission_set(&self) -> resources::Permissions_boundary_from_permission_set<'_> {
        resources::Permissions_boundary_from_permission_set::new(self.provider)
    }
    /// Get account_assignment_creation_status resource handler
    pub fn account_assignment_creation_status(&self) -> resources::Account_assignment_creation_status<'_> {
        resources::Account_assignment_creation_status::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get application_provider resource handler
    pub fn application_provider(&self) -> resources::Application_provider<'_> {
        resources::Application_provider::new(self.provider)
    }
    /// Get inline_policy_to_permission_set resource handler
    pub fn inline_policy_to_permission_set(&self) -> resources::Inline_policy_to_permission_set<'_> {
        resources::Inline_policy_to_permission_set::new(self.provider)
    }
    /// Get permission_set_provisioning_status resource handler
    pub fn permission_set_provisioning_status(&self) -> resources::Permission_set_provisioning_status<'_> {
        resources::Permission_set_provisioning_status::new(self.provider)
    }
    /// Get instance_access_control_attribute_configuration resource handler
    pub fn instance_access_control_attribute_configuration(&self) -> resources::Instance_access_control_attribute_configuration<'_> {
        resources::Instance_access_control_attribute_configuration::new(self.provider)
    }
    /// Get permissions_boundary_for_permission_set resource handler
    pub fn permissions_boundary_for_permission_set(&self) -> resources::Permissions_boundary_for_permission_set<'_> {
        resources::Permissions_boundary_for_permission_set::new(self.provider)
    }
    /// Get permissions_boundary_to_permission_set resource handler
    pub fn permissions_boundary_to_permission_set(&self) -> resources::Permissions_boundary_to_permission_set<'_> {
        resources::Permissions_boundary_to_permission_set::new(self.provider)
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
