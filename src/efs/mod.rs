//! Efs Service
//!
//! Auto-generated service module for efs

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for efs
pub struct EfsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> EfsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get replication_configurations resource handler
    pub fn replication_configurations(&self) -> resources::Replication_configurations<'_> {
        resources::Replication_configurations::new(self.provider)
    }
    /// Get access_points resource handler
    pub fn access_points(&self) -> resources::Access_points<'_> {
        resources::Access_points::new(self.provider)
    }
    /// Get file_systems resource handler
    pub fn file_systems(&self) -> resources::File_systems<'_> {
        resources::File_systems::new(self.provider)
    }
    /// Get file_system resource handler
    pub fn file_system(&self) -> resources::File_system<'_> {
        resources::File_system::new(self.provider)
    }
    /// Get backup_policy resource handler
    pub fn backup_policy(&self) -> resources::Backup_policy<'_> {
        resources::Backup_policy::new(self.provider)
    }
    /// Get file_system_protection resource handler
    pub fn file_system_protection(&self) -> resources::File_system_protection<'_> {
        resources::File_system_protection::new(self.provider)
    }
    /// Get mount_target resource handler
    pub fn mount_target(&self) -> resources::Mount_target<'_> {
        resources::Mount_target::new(self.provider)
    }
    /// Get file_system_policy resource handler
    pub fn file_system_policy(&self) -> resources::File_system_policy<'_> {
        resources::File_system_policy::new(self.provider)
    }
    /// Get replication_configuration resource handler
    pub fn replication_configuration(&self) -> resources::Replication_configuration<'_> {
        resources::Replication_configuration::new(self.provider)
    }
    /// Get access_point resource handler
    pub fn access_point(&self) -> resources::Access_point<'_> {
        resources::Access_point::new(self.provider)
    }
    /// Get mount_targets resource handler
    pub fn mount_targets(&self) -> resources::Mount_targets<'_> {
        resources::Mount_targets::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get account_preferences resource handler
    pub fn account_preferences(&self) -> resources::Account_preferences<'_> {
        resources::Account_preferences::new(self.provider)
    }
    /// Get mount_target_security_groups resource handler
    pub fn mount_target_security_groups(&self) -> resources::Mount_target_security_groups<'_> {
        resources::Mount_target_security_groups::new(self.provider)
    }
    /// Get lifecycle_configuration resource handler
    pub fn lifecycle_configuration(&self) -> resources::Lifecycle_configuration<'_> {
        resources::Lifecycle_configuration::new(self.provider)
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
