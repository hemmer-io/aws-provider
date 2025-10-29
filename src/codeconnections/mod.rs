//! Codeconnections Service
//!
//! Auto-generated service module for codeconnections

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codeconnections
pub struct CodeconnectionsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CodeconnectionsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get host resource handler
    pub fn host(&self) -> resources::Host<'_> {
        resources::Host::new(self.provider)
    }
    /// Get repository_sync_status resource handler
    pub fn repository_sync_status(&self) -> resources::Repository_sync_status<'_> {
        resources::Repository_sync_status::new(self.provider)
    }
    /// Get sync_blocker_summary resource handler
    pub fn sync_blocker_summary(&self) -> resources::Sync_blocker_summary<'_> {
        resources::Sync_blocker_summary::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get resource_sync_status resource handler
    pub fn resource_sync_status(&self) -> resources::Resource_sync_status<'_> {
        resources::Resource_sync_status::new(self.provider)
    }
    /// Get sync_configuration resource handler
    pub fn sync_configuration(&self) -> resources::Sync_configuration<'_> {
        resources::Sync_configuration::new(self.provider)
    }
    /// Get repository_link resource handler
    pub fn repository_link(&self) -> resources::Repository_link<'_> {
        resources::Repository_link::new(self.provider)
    }
    /// Get sync_blocker resource handler
    pub fn sync_blocker(&self) -> resources::Sync_blocker<'_> {
        resources::Sync_blocker::new(self.provider)
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
