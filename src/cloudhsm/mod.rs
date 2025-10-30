//! Cloudhsm Service
//!
//! Auto-generated service module for cloudhsm

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudhsm
pub struct CloudhsmService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CloudhsmService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get clusters resource handler
    pub fn clusters(&self) -> resources::Clusters<'_> {
        resources::Clusters::new(self.provider)
    }
    /// Get hsm resource handler
    pub fn hsm(&self) -> resources::Hsm<'_> {
        resources::Hsm::new(self.provider)
    }
    /// Get backups resource handler
    pub fn backups(&self) -> resources::Backups<'_> {
        resources::Backups::new(self.provider)
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
