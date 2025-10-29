//! Iottwinmaker Service
//!
//! Auto-generated service module for iottwinmaker

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iottwinmaker
pub struct IottwinmakerService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> IottwinmakerService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get property_value resource handler
    pub fn property_value(&self) -> resources::Property_value<'_> {
        resources::Property_value::new(self.provider)
    }
    /// Get sync_job resource handler
    pub fn sync_job(&self) -> resources::Sync_job<'_> {
        resources::Sync_job::new(self.provider)
    }
    /// Get component_type resource handler
    pub fn component_type(&self) -> resources::Component_type<'_> {
        resources::Component_type::new(self.provider)
    }
    /// Get metadata_transfer_job resource handler
    pub fn metadata_transfer_job(&self) -> resources::Metadata_transfer_job<'_> {
        resources::Metadata_transfer_job::new(self.provider)
    }
    /// Get property_value_history resource handler
    pub fn property_value_history(&self) -> resources::Property_value_history<'_> {
        resources::Property_value_history::new(self.provider)
    }
    /// Get scene resource handler
    pub fn scene(&self) -> resources::Scene<'_> {
        resources::Scene::new(self.provider)
    }
    /// Get pricing_plan resource handler
    pub fn pricing_plan(&self) -> resources::Pricing_plan<'_> {
        resources::Pricing_plan::new(self.provider)
    }
    /// Get entity resource handler
    pub fn entity(&self) -> resources::Entity<'_> {
        resources::Entity::new(self.provider)
    }
    /// Get workspace resource handler
    pub fn workspace(&self) -> resources::Workspace<'_> {
        resources::Workspace::new(self.provider)
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
