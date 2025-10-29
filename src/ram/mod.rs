//! Ram Service
//!
//! Auto-generated service module for ram

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ram
pub struct RamService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> RamService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get permission_version resource handler
    pub fn permission_version(&self) -> resources::Permission_version<'_> {
        resources::Permission_version::new(self.provider)
    }
    /// Get resource_shares resource handler
    pub fn resource_shares(&self) -> resources::Resource_shares<'_> {
        resources::Resource_shares::new(self.provider)
    }
    /// Get resource_share resource handler
    pub fn resource_share(&self) -> resources::Resource_share<'_> {
        resources::Resource_share::new(self.provider)
    }
    /// Get resource_share_invitations resource handler
    pub fn resource_share_invitations(&self) -> resources::Resource_share_invitations<'_> {
        resources::Resource_share_invitations::new(self.provider)
    }
    /// Get resource_share_associations resource handler
    pub fn resource_share_associations(&self) -> resources::Resource_share_associations<'_> {
        resources::Resource_share_associations::new(self.provider)
    }
    /// Get resource_policies resource handler
    pub fn resource_policies(&self) -> resources::Resource_policies<'_> {
        resources::Resource_policies::new(self.provider)
    }
    /// Get permission resource handler
    pub fn permission(&self) -> resources::Permission<'_> {
        resources::Permission::new(self.provider)
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
