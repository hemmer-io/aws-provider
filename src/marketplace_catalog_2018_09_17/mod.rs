//! Marketplace_catalog_2018_09_17 Service
//!
//! Auto-generated service module for marketplace_catalog_2018_09_17

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for marketplace_catalog_2018_09_17
pub struct Marketplace_catalog_2018_09_17Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Marketplace_catalog_2018_09_17Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get entity resource handler
    pub fn entity(&self) -> resources::Entity<'_> {
        resources::Entity::new(self.provider)
    }
    /// Get change_set resource handler
    pub fn change_set(&self) -> resources::Change_set<'_> {
        resources::Change_set::new(self.provider)
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
