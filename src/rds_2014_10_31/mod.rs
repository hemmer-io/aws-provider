//! Rds_2014_10_31 Service
//!
//! Auto-generated service module for rds_2014_10_31

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rds_2014_10_31
pub struct Rds_2014_10_31Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Rds_2014_10_31Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get db_proxy resource handler
    pub fn db_proxy(&self) -> resources::Db_proxy<'_> {
        resources::Db_proxy::new(self.provider)
    }
    /// Get db_cluster_automated_backups resource handler
    pub fn db_cluster_automated_backups(&self) -> resources::Db_cluster_automated_backups<'_> {
        resources::Db_cluster_automated_backups::new(self.provider)
    }
    /// Get source_regions resource handler
    pub fn source_regions(&self) -> resources::Source_regions<'_> {
        resources::Source_regions::new(self.provider)
    }
    /// Get db_cluster_backtracks resource handler
    pub fn db_cluster_backtracks(&self) -> resources::Db_cluster_backtracks<'_> {
        resources::Db_cluster_backtracks::new(self.provider)
    }
    /// Get db_cluster_parameter_group resource handler
    pub fn db_cluster_parameter_group(&self) -> resources::Db_cluster_parameter_group<'_> {
        resources::Db_cluster_parameter_group::new(self.provider)
    }
    /// Get db_subnet_group resource handler
    pub fn db_subnet_group(&self) -> resources::Db_subnet_group<'_> {
        resources::Db_subnet_group::new(self.provider)
    }
    /// Get event_subscriptions resource handler
    pub fn event_subscriptions(&self) -> resources::Event_subscriptions<'_> {
        resources::Event_subscriptions::new(self.provider)
    }
    /// Get orderable_db_instance_options resource handler
    pub fn orderable_db_instance_options(&self) -> resources::Orderable_db_instance_options<'_> {
        resources::Orderable_db_instance_options::new(self.provider)
    }
    /// Get db_proxy_endpoints resource handler
    pub fn db_proxy_endpoints(&self) -> resources::Db_proxy_endpoints<'_> {
        resources::Db_proxy_endpoints::new(self.provider)
    }
    /// Get db_instance_automated_backup resource handler
    pub fn db_instance_automated_backup(&self) -> resources::Db_instance_automated_backup<'_> {
        resources::Db_instance_automated_backup::new(self.provider)
    }
    /// Get engine_default_parameters resource handler
    pub fn engine_default_parameters(&self) -> resources::Engine_default_parameters<'_> {
        resources::Engine_default_parameters::new(self.provider)
    }
    /// Get tenant_database resource handler
    pub fn tenant_database(&self) -> resources::Tenant_database<'_> {
        resources::Tenant_database::new(self.provider)
    }
    /// Get db_instance_automated_backups resource handler
    pub fn db_instance_automated_backups(&self) -> resources::Db_instance_automated_backups<'_> {
        resources::Db_instance_automated_backups::new(self.provider)
    }
    /// Get blue_green_deployment resource handler
    pub fn blue_green_deployment(&self) -> resources::Blue_green_deployment<'_> {
        resources::Blue_green_deployment::new(self.provider)
    }
    /// Get db_log_files resource handler
    pub fn db_log_files(&self) -> resources::Db_log_files<'_> {
        resources::Db_log_files::new(self.provider)
    }
    /// Get db_snapshot resource handler
    pub fn db_snapshot(&self) -> resources::Db_snapshot<'_> {
        resources::Db_snapshot::new(self.provider)
    }
    /// Get reserved_db_instances resource handler
    pub fn reserved_db_instances(&self) -> resources::Reserved_db_instances<'_> {
        resources::Reserved_db_instances::new(self.provider)
    }
    /// Get certificates resource handler
    pub fn certificates(&self) -> resources::Certificates<'_> {
        resources::Certificates::new(self.provider)
    }
    /// Get db_security_groups resource handler
    pub fn db_security_groups(&self) -> resources::Db_security_groups<'_> {
        resources::Db_security_groups::new(self.provider)
    }
    /// Get event_categories resource handler
    pub fn event_categories(&self) -> resources::Event_categories<'_> {
        resources::Event_categories::new(self.provider)
    }
    /// Get db_shard_group resource handler
    pub fn db_shard_group(&self) -> resources::Db_shard_group<'_> {
        resources::Db_shard_group::new(self.provider)
    }
    /// Get db_cluster_parameter_groups resource handler
    pub fn db_cluster_parameter_groups(&self) -> resources::Db_cluster_parameter_groups<'_> {
        resources::Db_cluster_parameter_groups::new(self.provider)
    }
    /// Get option_group resource handler
    pub fn option_group(&self) -> resources::Option_group<'_> {
        resources::Option_group::new(self.provider)
    }
    /// Get option_groups resource handler
    pub fn option_groups(&self) -> resources::Option_groups<'_> {
        resources::Option_groups::new(self.provider)
    }
    /// Get reserved_db_instances_offerings resource handler
    pub fn reserved_db_instances_offerings(&self) -> resources::Reserved_db_instances_offerings<'_> {
        resources::Reserved_db_instances_offerings::new(self.provider)
    }
    /// Get valid_db_instance_modifications resource handler
    pub fn valid_db_instance_modifications(&self) -> resources::Valid_db_instance_modifications<'_> {
        resources::Valid_db_instance_modifications::new(self.provider)
    }
    /// Get tenant_databases resource handler
    pub fn tenant_databases(&self) -> resources::Tenant_databases<'_> {
        resources::Tenant_databases::new(self.provider)
    }
    /// Get db_major_engine_versions resource handler
    pub fn db_major_engine_versions(&self) -> resources::Db_major_engine_versions<'_> {
        resources::Db_major_engine_versions::new(self.provider)
    }
    /// Get db_instances resource handler
    pub fn db_instances(&self) -> resources::Db_instances<'_> {
        resources::Db_instances::new(self.provider)
    }
    /// Get db_parameter_group resource handler
    pub fn db_parameter_group(&self) -> resources::Db_parameter_group<'_> {
        resources::Db_parameter_group::new(self.provider)
    }
    /// Get db_shard_groups resource handler
    pub fn db_shard_groups(&self) -> resources::Db_shard_groups<'_> {
        resources::Db_shard_groups::new(self.provider)
    }
    /// Get global_clusters resource handler
    pub fn global_clusters(&self) -> resources::Global_clusters<'_> {
        resources::Global_clusters::new(self.provider)
    }
    /// Get custom_db_engine_version resource handler
    pub fn custom_db_engine_version(&self) -> resources::Custom_db_engine_version<'_> {
        resources::Custom_db_engine_version::new(self.provider)
    }
    /// Get db_cluster_endpoint resource handler
    pub fn db_cluster_endpoint(&self) -> resources::Db_cluster_endpoint<'_> {
        resources::Db_cluster_endpoint::new(self.provider)
    }
    /// Get db_security_group resource handler
    pub fn db_security_group(&self) -> resources::Db_security_group<'_> {
        resources::Db_security_group::new(self.provider)
    }
    /// Get db_proxy_targets resource handler
    pub fn db_proxy_targets(&self) -> resources::Db_proxy_targets<'_> {
        resources::Db_proxy_targets::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get engine_default_cluster_parameters resource handler
    pub fn engine_default_cluster_parameters(&self) -> resources::Engine_default_cluster_parameters<'_> {
        resources::Engine_default_cluster_parameters::new(self.provider)
    }
    /// Get db_engine_versions resource handler
    pub fn db_engine_versions(&self) -> resources::Db_engine_versions<'_> {
        resources::Db_engine_versions::new(self.provider)
    }
    /// Get db_clusters resource handler
    pub fn db_clusters(&self) -> resources::Db_clusters<'_> {
        resources::Db_clusters::new(self.provider)
    }
    /// Get export_tasks resource handler
    pub fn export_tasks(&self) -> resources::Export_tasks<'_> {
        resources::Export_tasks::new(self.provider)
    }
    /// Get db_cluster_automated_backup resource handler
    pub fn db_cluster_automated_backup(&self) -> resources::Db_cluster_automated_backup<'_> {
        resources::Db_cluster_automated_backup::new(self.provider)
    }
    /// Get pending_maintenance_actions resource handler
    pub fn pending_maintenance_actions(&self) -> resources::Pending_maintenance_actions<'_> {
        resources::Pending_maintenance_actions::new(self.provider)
    }
    /// Get db_cluster_parameters resource handler
    pub fn db_cluster_parameters(&self) -> resources::Db_cluster_parameters<'_> {
        resources::Db_cluster_parameters::new(self.provider)
    }
    /// Get db_snapshot_tenant_databases resource handler
    pub fn db_snapshot_tenant_databases(&self) -> resources::Db_snapshot_tenant_databases<'_> {
        resources::Db_snapshot_tenant_databases::new(self.provider)
    }
    /// Get db_proxy_target_groups resource handler
    pub fn db_proxy_target_groups(&self) -> resources::Db_proxy_target_groups<'_> {
        resources::Db_proxy_target_groups::new(self.provider)
    }
    /// Get db_cluster_snapshots resource handler
    pub fn db_cluster_snapshots(&self) -> resources::Db_cluster_snapshots<'_> {
        resources::Db_cluster_snapshots::new(self.provider)
    }
    /// Get db_cluster_snapshot_attributes resource handler
    pub fn db_cluster_snapshot_attributes(&self) -> resources::Db_cluster_snapshot_attributes<'_> {
        resources::Db_cluster_snapshot_attributes::new(self.provider)
    }
    /// Get event_subscription resource handler
    pub fn event_subscription(&self) -> resources::Event_subscription<'_> {
        resources::Event_subscription::new(self.provider)
    }
    /// Get db_snapshot_attributes resource handler
    pub fn db_snapshot_attributes(&self) -> resources::Db_snapshot_attributes<'_> {
        resources::Db_snapshot_attributes::new(self.provider)
    }
    /// Get db_cluster_snapshot resource handler
    pub fn db_cluster_snapshot(&self) -> resources::Db_cluster_snapshot<'_> {
        resources::Db_cluster_snapshot::new(self.provider)
    }
    /// Get option_group_options resource handler
    pub fn option_group_options(&self) -> resources::Option_group_options<'_> {
        resources::Option_group_options::new(self.provider)
    }
    /// Get db_subnet_groups resource handler
    pub fn db_subnet_groups(&self) -> resources::Db_subnet_groups<'_> {
        resources::Db_subnet_groups::new(self.provider)
    }
    /// Get db_cluster resource handler
    pub fn db_cluster(&self) -> resources::Db_cluster<'_> {
        resources::Db_cluster::new(self.provider)
    }
    /// Get integrations resource handler
    pub fn integrations(&self) -> resources::Integrations<'_> {
        resources::Integrations::new(self.provider)
    }
    /// Get db_proxy_endpoint resource handler
    pub fn db_proxy_endpoint(&self) -> resources::Db_proxy_endpoint<'_> {
        resources::Db_proxy_endpoint::new(self.provider)
    }
    /// Get account_attributes resource handler
    pub fn account_attributes(&self) -> resources::Account_attributes<'_> {
        resources::Account_attributes::new(self.provider)
    }
    /// Get blue_green_deployments resource handler
    pub fn blue_green_deployments(&self) -> resources::Blue_green_deployments<'_> {
        resources::Blue_green_deployments::new(self.provider)
    }
    /// Get db_cluster_endpoints resource handler
    pub fn db_cluster_endpoints(&self) -> resources::Db_cluster_endpoints<'_> {
        resources::Db_cluster_endpoints::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get db_snapshots resource handler
    pub fn db_snapshots(&self) -> resources::Db_snapshots<'_> {
        resources::Db_snapshots::new(self.provider)
    }
    /// Get db_instance resource handler
    pub fn db_instance(&self) -> resources::Db_instance<'_> {
        resources::Db_instance::new(self.provider)
    }
    /// Get db_proxies resource handler
    pub fn db_proxies(&self) -> resources::Db_proxies<'_> {
        resources::Db_proxies::new(self.provider)
    }
    /// Get db_recommendations resource handler
    pub fn db_recommendations(&self) -> resources::Db_recommendations<'_> {
        resources::Db_recommendations::new(self.provider)
    }
    /// Get db_parameter_groups resource handler
    pub fn db_parameter_groups(&self) -> resources::Db_parameter_groups<'_> {
        resources::Db_parameter_groups::new(self.provider)
    }
    /// Get db_parameters resource handler
    pub fn db_parameters(&self) -> resources::Db_parameters<'_> {
        resources::Db_parameters::new(self.provider)
    }
    /// Get db_instance_read_replica resource handler
    pub fn db_instance_read_replica(&self) -> resources::Db_instance_read_replica<'_> {
        resources::Db_instance_read_replica::new(self.provider)
    }
    /// Get global_cluster resource handler
    pub fn global_cluster(&self) -> resources::Global_cluster<'_> {
        resources::Global_cluster::new(self.provider)
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
