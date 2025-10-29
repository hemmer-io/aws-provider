//! Devops Service
//!
//! Auto-generated service module for devops

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for devops
pub struct DevopsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> DevopsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get event_sources_config resource handler
    pub fn event_sources_config(&self) -> resources::Event_sources_config<'_> {
        resources::Event_sources_config::new(self.provider)
    }
    /// Get organization_health resource handler
    pub fn organization_health(&self) -> resources::Organization_health<'_> {
        resources::Organization_health::new(self.provider)
    }
    /// Get resource_collection resource handler
    pub fn resource_collection(&self) -> resources::Resource_collection<'_> {
        resources::Resource_collection::new(self.provider)
    }
    /// Get organization_resource_collection_health resource handler
    pub fn organization_resource_collection_health(&self) -> resources::Organization_resource_collection_health<'_> {
        resources::Organization_resource_collection_health::new(self.provider)
    }
    /// Get organization_overview resource handler
    pub fn organization_overview(&self) -> resources::Organization_overview<'_> {
        resources::Organization_overview::new(self.provider)
    }
    /// Get account_health resource handler
    pub fn account_health(&self) -> resources::Account_health<'_> {
        resources::Account_health::new(self.provider)
    }
    /// Get cost_estimation resource handler
    pub fn cost_estimation(&self) -> resources::Cost_estimation<'_> {
        resources::Cost_estimation::new(self.provider)
    }
    /// Get service_integration resource handler
    pub fn service_integration(&self) -> resources::Service_integration<'_> {
        resources::Service_integration::new(self.provider)
    }
    /// Get resource_collection_health resource handler
    pub fn resource_collection_health(&self) -> resources::Resource_collection_health<'_> {
        resources::Resource_collection_health::new(self.provider)
    }
    /// Get insight resource handler
    pub fn insight(&self) -> resources::Insight<'_> {
        resources::Insight::new(self.provider)
    }
    /// Get feedback resource handler
    pub fn feedback(&self) -> resources::Feedback<'_> {
        resources::Feedback::new(self.provider)
    }
    /// Get anomaly resource handler
    pub fn anomaly(&self) -> resources::Anomaly<'_> {
        resources::Anomaly::new(self.provider)
    }
    /// Get account_overview resource handler
    pub fn account_overview(&self) -> resources::Account_overview<'_> {
        resources::Account_overview::new(self.provider)
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
