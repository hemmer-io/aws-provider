//! Ssm_incidents_2018_05_10 Service
//!
//! Auto-generated service module for ssm_incidents_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ssm_incidents_2018_05_10
pub struct Ssm_incidents_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ssm_incidents_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get response_plan resource handler
    pub fn response_plan(&self) -> resources::Response_plan<'_> {
        resources::Response_plan::new(self.provider)
    }
    /// Get timeline_event resource handler
    pub fn timeline_event(&self) -> resources::Timeline_event<'_> {
        resources::Timeline_event::new(self.provider)
    }
    /// Get replication_set resource handler
    pub fn replication_set(&self) -> resources::Replication_set<'_> {
        resources::Replication_set::new(self.provider)
    }
    /// Get deletion_protection resource handler
    pub fn deletion_protection(&self) -> resources::Deletion_protection<'_> {
        resources::Deletion_protection::new(self.provider)
    }
    /// Get incident_record resource handler
    pub fn incident_record(&self) -> resources::Incident_record<'_> {
        resources::Incident_record::new(self.provider)
    }
    /// Get resource_policies resource handler
    pub fn resource_policies(&self) -> resources::Resource_policies<'_> {
        resources::Resource_policies::new(self.provider)
    }
    /// Get related_items resource handler
    pub fn related_items(&self) -> resources::Related_items<'_> {
        resources::Related_items::new(self.provider)
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
