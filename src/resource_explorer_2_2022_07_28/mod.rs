//! Resource_explorer_2_2022_07_28 Service
//!
//! Auto-generated service module for resource_explorer_2_2022_07_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for resource_explorer_2_2022_07_28
pub struct Resource_explorer_2_2022_07_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Resource_explorer_2_2022_07_28Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get default_view resource handler
    pub fn default_view(&self) -> resources::Default_view<'_> {
        resources::Default_view::new(self.provider)
    }
    /// Get resource_explorer_setup resource handler
    pub fn resource_explorer_setup(&self) -> resources::Resource_explorer_setup<'_> {
        resources::Resource_explorer_setup::new(self.provider)
    }
    /// Get managed_view resource handler
    pub fn managed_view(&self) -> resources::Managed_view<'_> {
        resources::Managed_view::new(self.provider)
    }
    /// Get index resource handler
    pub fn index(&self) -> resources::Index<'_> {
        resources::Index::new(self.provider)
    }
    /// Get account_level_service_configuration resource handler
    pub fn account_level_service_configuration(&self) -> resources::Account_level_service_configuration<'_> {
        resources::Account_level_service_configuration::new(self.provider)
    }
    /// Get service_view resource handler
    pub fn service_view(&self) -> resources::Service_view<'_> {
        resources::Service_view::new(self.provider)
    }
    /// Get service_index resource handler
    pub fn service_index(&self) -> resources::Service_index<'_> {
        resources::Service_index::new(self.provider)
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
