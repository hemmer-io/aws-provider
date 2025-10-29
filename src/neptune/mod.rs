//! Neptune Service
//!
//! Auto-generated service module for neptune

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for neptune
pub struct NeptuneService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> NeptuneService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get dbcluster_snapshots resource handler
    pub fn dbcluster_snapshots(&self) -> resources::Dbcluster_snapshots<'_> {
        resources::Dbcluster_snapshots::new(self.provider)
    }
    /// Get dbcluster resource handler
    pub fn dbcluster(&self) -> resources::Dbcluster<'_> {
        resources::Dbcluster::new(self.provider)
    }
    /// Get dbparameter_group resource handler
    pub fn dbparameter_group(&self) -> resources::Dbparameter_group<'_> {
        resources::Dbparameter_group::new(self.provider)
    }
    /// Get global_cluster resource handler
    pub fn global_cluster(&self) -> resources::Global_cluster<'_> {
        resources::Global_cluster::new(self.provider)
    }
    /// Get dbcluster_endpoints resource handler
    pub fn dbcluster_endpoints(&self) -> resources::Dbcluster_endpoints<'_> {
        resources::Dbcluster_endpoints::new(self.provider)
    }
    /// Get dbparameters resource handler
    pub fn dbparameters(&self) -> resources::Dbparameters<'_> {
        resources::Dbparameters::new(self.provider)
    }
    /// Get dbsubnet_group resource handler
    pub fn dbsubnet_group(&self) -> resources::Dbsubnet_group<'_> {
        resources::Dbsubnet_group::new(self.provider)
    }
    /// Get dbcluster_parameter_group resource handler
    pub fn dbcluster_parameter_group(&self) -> resources::Dbcluster_parameter_group<'_> {
        resources::Dbcluster_parameter_group::new(self.provider)
    }
    /// Get event_subscriptions resource handler
    pub fn event_subscriptions(&self) -> resources::Event_subscriptions<'_> {
        resources::Event_subscriptions::new(self.provider)
    }
    /// Get valid_dbinstance_modifications resource handler
    pub fn valid_dbinstance_modifications(&self) -> resources::Valid_dbinstance_modifications<'_> {
        resources::Valid_dbinstance_modifications::new(self.provider)
    }
    /// Get event_categories resource handler
    pub fn event_categories(&self) -> resources::Event_categories<'_> {
        resources::Event_categories::new(self.provider)
    }
    /// Get global_clusters resource handler
    pub fn global_clusters(&self) -> resources::Global_clusters<'_> {
        resources::Global_clusters::new(self.provider)
    }
    /// Get event_subscription resource handler
    pub fn event_subscription(&self) -> resources::Event_subscription<'_> {
        resources::Event_subscription::new(self.provider)
    }
    /// Get dbcluster_endpoint resource handler
    pub fn dbcluster_endpoint(&self) -> resources::Dbcluster_endpoint<'_> {
        resources::Dbcluster_endpoint::new(self.provider)
    }
    /// Get dbinstance resource handler
    pub fn dbinstance(&self) -> resources::Dbinstance<'_> {
        resources::Dbinstance::new(self.provider)
    }
    /// Get dbparameter_groups resource handler
    pub fn dbparameter_groups(&self) -> resources::Dbparameter_groups<'_> {
        resources::Dbparameter_groups::new(self.provider)
    }
    /// Get engine_default_cluster_parameters resource handler
    pub fn engine_default_cluster_parameters(&self) -> resources::Engine_default_cluster_parameters<'_> {
        resources::Engine_default_cluster_parameters::new(self.provider)
    }
    /// Get dbengine_versions resource handler
    pub fn dbengine_versions(&self) -> resources::Dbengine_versions<'_> {
        resources::Dbengine_versions::new(self.provider)
    }
    /// Get pending_maintenance_actions resource handler
    pub fn pending_maintenance_actions(&self) -> resources::Pending_maintenance_actions<'_> {
        resources::Pending_maintenance_actions::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get dbcluster_snapshot resource handler
    pub fn dbcluster_snapshot(&self) -> resources::Dbcluster_snapshot<'_> {
        resources::Dbcluster_snapshot::new(self.provider)
    }
    /// Get orderable_dbinstance_options resource handler
    pub fn orderable_dbinstance_options(&self) -> resources::Orderable_dbinstance_options<'_> {
        resources::Orderable_dbinstance_options::new(self.provider)
    }
    /// Get dbcluster_snapshot_attributes resource handler
    pub fn dbcluster_snapshot_attributes(&self) -> resources::Dbcluster_snapshot_attributes<'_> {
        resources::Dbcluster_snapshot_attributes::new(self.provider)
    }
    /// Get dbclusters resource handler
    pub fn dbclusters(&self) -> resources::Dbclusters<'_> {
        resources::Dbclusters::new(self.provider)
    }
    /// Get dbcluster_parameters resource handler
    pub fn dbcluster_parameters(&self) -> resources::Dbcluster_parameters<'_> {
        resources::Dbcluster_parameters::new(self.provider)
    }
    /// Get dbsubnet_groups resource handler
    pub fn dbsubnet_groups(&self) -> resources::Dbsubnet_groups<'_> {
        resources::Dbsubnet_groups::new(self.provider)
    }
    /// Get dbinstances resource handler
    pub fn dbinstances(&self) -> resources::Dbinstances<'_> {
        resources::Dbinstances::new(self.provider)
    }
    /// Get engine_default_parameters resource handler
    pub fn engine_default_parameters(&self) -> resources::Engine_default_parameters<'_> {
        resources::Engine_default_parameters::new(self.provider)
    }
    /// Get dbcluster_parameter_groups resource handler
    pub fn dbcluster_parameter_groups(&self) -> resources::Dbcluster_parameter_groups<'_> {
        resources::Dbcluster_parameter_groups::new(self.provider)
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
