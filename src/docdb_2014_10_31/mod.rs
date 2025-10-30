//! Docdb_2014_10_31 Service
//!
//! Auto-generated service module for docdb_2014_10_31

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for docdb_2014_10_31
pub struct Docdb_2014_10_31Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Docdb_2014_10_31Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get db_cluster_snapshot resource handler
    pub fn db_cluster_snapshot(&self) -> resources::Db_cluster_snapshot<'_> {
        resources::Db_cluster_snapshot::new(self.provider)
    }
    /// Get db_engine_versions resource handler
    pub fn db_engine_versions(&self) -> resources::Db_engine_versions<'_> {
        resources::Db_engine_versions::new(self.provider)
    }
    /// Get db_subnet_groups resource handler
    pub fn db_subnet_groups(&self) -> resources::Db_subnet_groups<'_> {
        resources::Db_subnet_groups::new(self.provider)
    }
    /// Get event_categories resource handler
    pub fn event_categories(&self) -> resources::Event_categories<'_> {
        resources::Event_categories::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get global_cluster resource handler
    pub fn global_cluster(&self) -> resources::Global_cluster<'_> {
        resources::Global_cluster::new(self.provider)
    }
    /// Get db_clusters resource handler
    pub fn db_clusters(&self) -> resources::Db_clusters<'_> {
        resources::Db_clusters::new(self.provider)
    }
    /// Get db_cluster resource handler
    pub fn db_cluster(&self) -> resources::Db_cluster<'_> {
        resources::Db_cluster::new(self.provider)
    }
    /// Get db_cluster_snapshot_attributes resource handler
    pub fn db_cluster_snapshot_attributes(&self) -> resources::Db_cluster_snapshot_attributes<'_> {
        resources::Db_cluster_snapshot_attributes::new(self.provider)
    }
    /// Get event_subscription resource handler
    pub fn event_subscription(&self) -> resources::Event_subscription<'_> {
        resources::Event_subscription::new(self.provider)
    }
    /// Get engine_default_cluster_parameters resource handler
    pub fn engine_default_cluster_parameters(&self) -> resources::Engine_default_cluster_parameters<'_> {
        resources::Engine_default_cluster_parameters::new(self.provider)
    }
    /// Get db_cluster_parameter_groups resource handler
    pub fn db_cluster_parameter_groups(&self) -> resources::Db_cluster_parameter_groups<'_> {
        resources::Db_cluster_parameter_groups::new(self.provider)
    }
    /// Get db_cluster_snapshots resource handler
    pub fn db_cluster_snapshots(&self) -> resources::Db_cluster_snapshots<'_> {
        resources::Db_cluster_snapshots::new(self.provider)
    }
    /// Get event_subscriptions resource handler
    pub fn event_subscriptions(&self) -> resources::Event_subscriptions<'_> {
        resources::Event_subscriptions::new(self.provider)
    }
    /// Get orderable_db_instance_options resource handler
    pub fn orderable_db_instance_options(&self) -> resources::Orderable_db_instance_options<'_> {
        resources::Orderable_db_instance_options::new(self.provider)
    }
    /// Get db_cluster_parameters resource handler
    pub fn db_cluster_parameters(&self) -> resources::Db_cluster_parameters<'_> {
        resources::Db_cluster_parameters::new(self.provider)
    }
    /// Get db_instances resource handler
    pub fn db_instances(&self) -> resources::Db_instances<'_> {
        resources::Db_instances::new(self.provider)
    }
    /// Get db_subnet_group resource handler
    pub fn db_subnet_group(&self) -> resources::Db_subnet_group<'_> {
        resources::Db_subnet_group::new(self.provider)
    }
    /// Get db_instance resource handler
    pub fn db_instance(&self) -> resources::Db_instance<'_> {
        resources::Db_instance::new(self.provider)
    }
    /// Get pending_maintenance_actions resource handler
    pub fn pending_maintenance_actions(&self) -> resources::Pending_maintenance_actions<'_> {
        resources::Pending_maintenance_actions::new(self.provider)
    }
    /// Get certificates resource handler
    pub fn certificates(&self) -> resources::Certificates<'_> {
        resources::Certificates::new(self.provider)
    }
    /// Get global_clusters resource handler
    pub fn global_clusters(&self) -> resources::Global_clusters<'_> {
        resources::Global_clusters::new(self.provider)
    }
    /// Get db_cluster_parameter_group resource handler
    pub fn db_cluster_parameter_group(&self) -> resources::Db_cluster_parameter_group<'_> {
        resources::Db_cluster_parameter_group::new(self.provider)
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
