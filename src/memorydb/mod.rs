//! Memorydb Service
//!
//! Auto-generated service module for memorydb

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for memorydb
pub struct MemorydbService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MemorydbService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get acls resource handler
    pub fn acls(&self) -> resources::Acls<'_> {
        resources::Acls::new(self.provider)
    }
    /// Get clusters resource handler
    pub fn clusters(&self) -> resources::Clusters<'_> {
        resources::Clusters::new(self.provider)
    }
    /// Get parameter_groups resource handler
    pub fn parameter_groups(&self) -> resources::Parameter_groups<'_> {
        resources::Parameter_groups::new(self.provider)
    }
    /// Get subnet_groups resource handler
    pub fn subnet_groups(&self) -> resources::Subnet_groups<'_> {
        resources::Subnet_groups::new(self.provider)
    }
    /// Get users resource handler
    pub fn users(&self) -> resources::Users<'_> {
        resources::Users::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get subnet_group resource handler
    pub fn subnet_group(&self) -> resources::Subnet_group<'_> {
        resources::Subnet_group::new(self.provider)
    }
    /// Get acl resource handler
    pub fn acl(&self) -> resources::Acl<'_> {
        resources::Acl::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get multi_region_parameters resource handler
    pub fn multi_region_parameters(&self) -> resources::Multi_region_parameters<'_> {
        resources::Multi_region_parameters::new(self.provider)
    }
    /// Get reserved_nodes_offerings resource handler
    pub fn reserved_nodes_offerings(&self) -> resources::Reserved_nodes_offerings<'_> {
        resources::Reserved_nodes_offerings::new(self.provider)
    }
    /// Get service_updates resource handler
    pub fn service_updates(&self) -> resources::Service_updates<'_> {
        resources::Service_updates::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get parameter_group resource handler
    pub fn parameter_group(&self) -> resources::Parameter_group<'_> {
        resources::Parameter_group::new(self.provider)
    }
    /// Get reserved_nodes resource handler
    pub fn reserved_nodes(&self) -> resources::Reserved_nodes<'_> {
        resources::Reserved_nodes::new(self.provider)
    }
    /// Get multi_region_cluster resource handler
    pub fn multi_region_cluster(&self) -> resources::Multi_region_cluster<'_> {
        resources::Multi_region_cluster::new(self.provider)
    }
    /// Get engine_versions resource handler
    pub fn engine_versions(&self) -> resources::Engine_versions<'_> {
        resources::Engine_versions::new(self.provider)
    }
    /// Get multi_region_clusters resource handler
    pub fn multi_region_clusters(&self) -> resources::Multi_region_clusters<'_> {
        resources::Multi_region_clusters::new(self.provider)
    }
    /// Get multi_region_parameter_groups resource handler
    pub fn multi_region_parameter_groups(&self) -> resources::Multi_region_parameter_groups<'_> {
        resources::Multi_region_parameter_groups::new(self.provider)
    }
    /// Get parameters resource handler
    pub fn parameters(&self) -> resources::Parameters<'_> {
        resources::Parameters::new(self.provider)
    }
    /// Get snapshots resource handler
    pub fn snapshots(&self) -> resources::Snapshots<'_> {
        resources::Snapshots::new(self.provider)
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
