//! Rds Service
//!
//! Auto-generated service module for rds

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rds
pub struct RdsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> RdsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get dbinstance resource handler
    pub fn dbinstance(&self) -> resources::Dbinstance<'_> {
        resources::Dbinstance::new(self.provider)
    }
    /// Get event_subscriptions resource handler
    pub fn event_subscriptions(&self) -> resources::Event_subscriptions<'_> {
        resources::Event_subscriptions::new(self.provider)
    }
    /// Get dbcluster_automated_backups resource handler
    pub fn dbcluster_automated_backups(&self) -> resources::Dbcluster_automated_backups<'_> {
        resources::Dbcluster_automated_backups::new(self.provider)
    }
    /// Get integrations resource handler
    pub fn integrations(&self) -> resources::Integrations<'_> {
        resources::Integrations::new(self.provider)
    }
    /// Get source_regions resource handler
    pub fn source_regions(&self) -> resources::Source_regions<'_> {
        resources::Source_regions::new(self.provider)
    }
    /// Get dbcluster_parameter_group resource handler
    pub fn dbcluster_parameter_group(&self) -> resources::Dbcluster_parameter_group<'_> {
        resources::Dbcluster_parameter_group::new(self.provider)
    }
    /// Get dbproxy_endpoint resource handler
    pub fn dbproxy_endpoint(&self) -> resources::Dbproxy_endpoint<'_> {
        resources::Dbproxy_endpoint::new(self.provider)
    }
    /// Get dbmajor_engine_versions resource handler
    pub fn dbmajor_engine_versions(&self) -> resources::Dbmajor_engine_versions<'_> {
        resources::Dbmajor_engine_versions::new(self.provider)
    }
    /// Get dbcluster_backtracks resource handler
    pub fn dbcluster_backtracks(&self) -> resources::Dbcluster_backtracks<'_> {
        resources::Dbcluster_backtracks::new(self.provider)
    }
    /// Get dbparameter_groups resource handler
    pub fn dbparameter_groups(&self) -> resources::Dbparameter_groups<'_> {
        resources::Dbparameter_groups::new(self.provider)
    }
    /// Get dbsnapshot_attributes resource handler
    pub fn dbsnapshot_attributes(&self) -> resources::Dbsnapshot_attributes<'_> {
        resources::Dbsnapshot_attributes::new(self.provider)
    }
    /// Get valid_dbinstance_modifications resource handler
    pub fn valid_dbinstance_modifications(&self) -> resources::Valid_dbinstance_modifications<'_> {
        resources::Valid_dbinstance_modifications::new(self.provider)
    }
    /// Get dbinstance_automated_backups resource handler
    pub fn dbinstance_automated_backups(&self) -> resources::Dbinstance_automated_backups<'_> {
        resources::Dbinstance_automated_backups::new(self.provider)
    }
    /// Get reserved_dbinstances_offerings resource handler
    pub fn reserved_dbinstances_offerings(&self) -> resources::Reserved_dbinstances_offerings<'_> {
        resources::Reserved_dbinstances_offerings::new(self.provider)
    }
    /// Get dbproxy resource handler
    pub fn dbproxy(&self) -> resources::Dbproxy<'_> {
        resources::Dbproxy::new(self.provider)
    }
    /// Get dbcluster_snapshots resource handler
    pub fn dbcluster_snapshots(&self) -> resources::Dbcluster_snapshots<'_> {
        resources::Dbcluster_snapshots::new(self.provider)
    }
    /// Get dbcluster_automated_backup resource handler
    pub fn dbcluster_automated_backup(&self) -> resources::Dbcluster_automated_backup<'_> {
        resources::Dbcluster_automated_backup::new(self.provider)
    }
    /// Get dbshard_groups resource handler
    pub fn dbshard_groups(&self) -> resources::Dbshard_groups<'_> {
        resources::Dbshard_groups::new(self.provider)
    }
    /// Get dbcluster_parameters resource handler
    pub fn dbcluster_parameters(&self) -> resources::Dbcluster_parameters<'_> {
        resources::Dbcluster_parameters::new(self.provider)
    }
    /// Get blue_green_deployment resource handler
    pub fn blue_green_deployment(&self) -> resources::Blue_green_deployment<'_> {
        resources::Blue_green_deployment::new(self.provider)
    }
    /// Get dbproxies resource handler
    pub fn dbproxies(&self) -> resources::Dbproxies<'_> {
        resources::Dbproxies::new(self.provider)
    }
    /// Get dbsecurity_groups resource handler
    pub fn dbsecurity_groups(&self) -> resources::Dbsecurity_groups<'_> {
        resources::Dbsecurity_groups::new(self.provider)
    }
    /// Get dbproxy_endpoints resource handler
    pub fn dbproxy_endpoints(&self) -> resources::Dbproxy_endpoints<'_> {
        resources::Dbproxy_endpoints::new(self.provider)
    }
    /// Get engine_default_cluster_parameters resource handler
    pub fn engine_default_cluster_parameters(&self) -> resources::Engine_default_cluster_parameters<'_> {
        resources::Engine_default_cluster_parameters::new(self.provider)
    }
    /// Get dbcluster_endpoint resource handler
    pub fn dbcluster_endpoint(&self) -> resources::Dbcluster_endpoint<'_> {
        resources::Dbcluster_endpoint::new(self.provider)
    }
    /// Get dbcluster_parameter_groups resource handler
    pub fn dbcluster_parameter_groups(&self) -> resources::Dbcluster_parameter_groups<'_> {
        resources::Dbcluster_parameter_groups::new(self.provider)
    }
    /// Get dbinstances resource handler
    pub fn dbinstances(&self) -> resources::Dbinstances<'_> {
        resources::Dbinstances::new(self.provider)
    }
    /// Get option_group_options resource handler
    pub fn option_group_options(&self) -> resources::Option_group_options<'_> {
        resources::Option_group_options::new(self.provider)
    }
    /// Get blue_green_deployments resource handler
    pub fn blue_green_deployments(&self) -> resources::Blue_green_deployments<'_> {
        resources::Blue_green_deployments::new(self.provider)
    }
    /// Get orderable_dbinstance_options resource handler
    pub fn orderable_dbinstance_options(&self) -> resources::Orderable_dbinstance_options<'_> {
        resources::Orderable_dbinstance_options::new(self.provider)
    }
    /// Get event_categories resource handler
    pub fn event_categories(&self) -> resources::Event_categories<'_> {
        resources::Event_categories::new(self.provider)
    }
    /// Get global_clusters resource handler
    pub fn global_clusters(&self) -> resources::Global_clusters<'_> {
        resources::Global_clusters::new(self.provider)
    }
    /// Get tenant_database resource handler
    pub fn tenant_database(&self) -> resources::Tenant_database<'_> {
        resources::Tenant_database::new(self.provider)
    }
    /// Get pending_maintenance_actions resource handler
    pub fn pending_maintenance_actions(&self) -> resources::Pending_maintenance_actions<'_> {
        resources::Pending_maintenance_actions::new(self.provider)
    }
    /// Get tenant_databases resource handler
    pub fn tenant_databases(&self) -> resources::Tenant_databases<'_> {
        resources::Tenant_databases::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get dbcluster_snapshot resource handler
    pub fn dbcluster_snapshot(&self) -> resources::Dbcluster_snapshot<'_> {
        resources::Dbcluster_snapshot::new(self.provider)
    }
    /// Get dbcluster resource handler
    pub fn dbcluster(&self) -> resources::Dbcluster<'_> {
        resources::Dbcluster::new(self.provider)
    }
    /// Get dbproxy_target_groups resource handler
    pub fn dbproxy_target_groups(&self) -> resources::Dbproxy_target_groups<'_> {
        resources::Dbproxy_target_groups::new(self.provider)
    }
    /// Get dbrecommendations resource handler
    pub fn dbrecommendations(&self) -> resources::Dbrecommendations<'_> {
        resources::Dbrecommendations::new(self.provider)
    }
    /// Get engine_default_parameters resource handler
    pub fn engine_default_parameters(&self) -> resources::Engine_default_parameters<'_> {
        resources::Engine_default_parameters::new(self.provider)
    }
    /// Get dbproxy_targets resource handler
    pub fn dbproxy_targets(&self) -> resources::Dbproxy_targets<'_> {
        resources::Dbproxy_targets::new(self.provider)
    }
    /// Get option_group resource handler
    pub fn option_group(&self) -> resources::Option_group<'_> {
        resources::Option_group::new(self.provider)
    }
    /// Get dbsnapshot resource handler
    pub fn dbsnapshot(&self) -> resources::Dbsnapshot<'_> {
        resources::Dbsnapshot::new(self.provider)
    }
    /// Get dbparameters resource handler
    pub fn dbparameters(&self) -> resources::Dbparameters<'_> {
        resources::Dbparameters::new(self.provider)
    }
    /// Get dbsnapshot_tenant_databases resource handler
    pub fn dbsnapshot_tenant_databases(&self) -> resources::Dbsnapshot_tenant_databases<'_> {
        resources::Dbsnapshot_tenant_databases::new(self.provider)
    }
    /// Get dbinstance_read_replica resource handler
    pub fn dbinstance_read_replica(&self) -> resources::Dbinstance_read_replica<'_> {
        resources::Dbinstance_read_replica::new(self.provider)
    }
    /// Get account_attributes resource handler
    pub fn account_attributes(&self) -> resources::Account_attributes<'_> {
        resources::Account_attributes::new(self.provider)
    }
    /// Get event_subscription resource handler
    pub fn event_subscription(&self) -> resources::Event_subscription<'_> {
        resources::Event_subscription::new(self.provider)
    }
    /// Get custom_dbengine_version resource handler
    pub fn custom_dbengine_version(&self) -> resources::Custom_dbengine_version<'_> {
        resources::Custom_dbengine_version::new(self.provider)
    }
    /// Get dbsubnet_groups resource handler
    pub fn dbsubnet_groups(&self) -> resources::Dbsubnet_groups<'_> {
        resources::Dbsubnet_groups::new(self.provider)
    }
    /// Get dbsnapshots resource handler
    pub fn dbsnapshots(&self) -> resources::Dbsnapshots<'_> {
        resources::Dbsnapshots::new(self.provider)
    }
    /// Get dbsubnet_group resource handler
    pub fn dbsubnet_group(&self) -> resources::Dbsubnet_group<'_> {
        resources::Dbsubnet_group::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get certificates resource handler
    pub fn certificates(&self) -> resources::Certificates<'_> {
        resources::Certificates::new(self.provider)
    }
    /// Get dblog_files resource handler
    pub fn dblog_files(&self) -> resources::Dblog_files<'_> {
        resources::Dblog_files::new(self.provider)
    }
    /// Get option_groups resource handler
    pub fn option_groups(&self) -> resources::Option_groups<'_> {
        resources::Option_groups::new(self.provider)
    }
    /// Get dbinstance_automated_backup resource handler
    pub fn dbinstance_automated_backup(&self) -> resources::Dbinstance_automated_backup<'_> {
        resources::Dbinstance_automated_backup::new(self.provider)
    }
    /// Get export_tasks resource handler
    pub fn export_tasks(&self) -> resources::Export_tasks<'_> {
        resources::Export_tasks::new(self.provider)
    }
    /// Get dbcluster_endpoints resource handler
    pub fn dbcluster_endpoints(&self) -> resources::Dbcluster_endpoints<'_> {
        resources::Dbcluster_endpoints::new(self.provider)
    }
    /// Get global_cluster resource handler
    pub fn global_cluster(&self) -> resources::Global_cluster<'_> {
        resources::Global_cluster::new(self.provider)
    }
    /// Get dbparameter_group resource handler
    pub fn dbparameter_group(&self) -> resources::Dbparameter_group<'_> {
        resources::Dbparameter_group::new(self.provider)
    }
    /// Get dbcluster_snapshot_attributes resource handler
    pub fn dbcluster_snapshot_attributes(&self) -> resources::Dbcluster_snapshot_attributes<'_> {
        resources::Dbcluster_snapshot_attributes::new(self.provider)
    }
    /// Get reserved_dbinstances resource handler
    pub fn reserved_dbinstances(&self) -> resources::Reserved_dbinstances<'_> {
        resources::Reserved_dbinstances::new(self.provider)
    }
    /// Get dbclusters resource handler
    pub fn dbclusters(&self) -> resources::Dbclusters<'_> {
        resources::Dbclusters::new(self.provider)
    }
    /// Get dbsecurity_group resource handler
    pub fn dbsecurity_group(&self) -> resources::Dbsecurity_group<'_> {
        resources::Dbsecurity_group::new(self.provider)
    }
    /// Get dbshard_group resource handler
    pub fn dbshard_group(&self) -> resources::Dbshard_group<'_> {
        resources::Dbshard_group::new(self.provider)
    }
    /// Get dbengine_versions resource handler
    pub fn dbengine_versions(&self) -> resources::Dbengine_versions<'_> {
        resources::Dbengine_versions::new(self.provider)
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
