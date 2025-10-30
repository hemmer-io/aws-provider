//! Organizations_2016_11_28 Service
//!
//! Auto-generated service module for organizations_2016_11_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for organizations_2016_11_28
pub struct Organizations_2016_11_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Organizations_2016_11_28Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get organizational_unit resource handler
    pub fn organizational_unit(&self) -> resources::Organizational_unit<'_> {
        resources::Organizational_unit::new(self.provider)
    }
    /// Get effective_policy resource handler
    pub fn effective_policy(&self) -> resources::Effective_policy<'_> {
        resources::Effective_policy::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
    }
    /// Get gov_cloud_account resource handler
    pub fn gov_cloud_account(&self) -> resources::Gov_cloud_account<'_> {
        resources::Gov_cloud_account::new(self.provider)
    }
    /// Get organization resource handler
    pub fn organization(&self) -> resources::Organization<'_> {
        resources::Organization::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get create_account_status resource handler
    pub fn create_account_status(&self) -> resources::Create_account_status<'_> {
        resources::Create_account_status::new(self.provider)
    }
    /// Get handshake resource handler
    pub fn handshake(&self) -> resources::Handshake<'_> {
        resources::Handshake::new(self.provider)
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
