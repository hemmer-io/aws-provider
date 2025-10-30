//! Database_migration_service_2016_01_01 Service
//!
//! Auto-generated service module for database_migration_service_2016_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for database_migration_service_2016_01_01
pub struct Database_migration_service_2016_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Database_migration_service_2016_01_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get conversion_configuration resource handler
    pub fn conversion_configuration(&self) -> resources::Conversion_configuration<'_> {
        resources::Conversion_configuration::new(self.provider)
    }
    /// Get replication_subnet_group resource handler
    pub fn replication_subnet_group(&self) -> resources::Replication_subnet_group<'_> {
        resources::Replication_subnet_group::new(self.provider)
    }
    /// Get replication_tasks resource handler
    pub fn replication_tasks(&self) -> resources::Replication_tasks<'_> {
        resources::Replication_tasks::new(self.provider)
    }
    /// Get engine_versions resource handler
    pub fn engine_versions(&self) -> resources::Engine_versions<'_> {
        resources::Engine_versions::new(self.provider)
    }
    /// Get instance_profile resource handler
    pub fn instance_profile(&self) -> resources::Instance_profile<'_> {
        resources::Instance_profile::new(self.provider)
    }
    /// Get orderable_replication_instances resource handler
    pub fn orderable_replication_instances(&self) -> resources::Orderable_replication_instances<'_> {
        resources::Orderable_replication_instances::new(self.provider)
    }
    /// Get data_provider resource handler
    pub fn data_provider(&self) -> resources::Data_provider<'_> {
        resources::Data_provider::new(self.provider)
    }
    /// Get endpoints resource handler
    pub fn endpoints(&self) -> resources::Endpoints<'_> {
        resources::Endpoints::new(self.provider)
    }
    /// Get extension_pack_associations resource handler
    pub fn extension_pack_associations(&self) -> resources::Extension_pack_associations<'_> {
        resources::Extension_pack_associations::new(self.provider)
    }
    /// Get replications resource handler
    pub fn replications(&self) -> resources::Replications<'_> {
        resources::Replications::new(self.provider)
    }
    /// Get metadata_model_exports_to_target resource handler
    pub fn metadata_model_exports_to_target(&self) -> resources::Metadata_model_exports_to_target<'_> {
        resources::Metadata_model_exports_to_target::new(self.provider)
    }
    /// Get fleet_advisor_collectors resource handler
    pub fn fleet_advisor_collectors(&self) -> resources::Fleet_advisor_collectors<'_> {
        resources::Fleet_advisor_collectors::new(self.provider)
    }
    /// Get replication_configs resource handler
    pub fn replication_configs(&self) -> resources::Replication_configs<'_> {
        resources::Replication_configs::new(self.provider)
    }
    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }
    /// Get certificates resource handler
    pub fn certificates(&self) -> resources::Certificates<'_> {
        resources::Certificates::new(self.provider)
    }
    /// Get fleet_advisor_collector resource handler
    pub fn fleet_advisor_collector(&self) -> resources::Fleet_advisor_collector<'_> {
        resources::Fleet_advisor_collector::new(self.provider)
    }
    /// Get replication_instances resource handler
    pub fn replication_instances(&self) -> resources::Replication_instances<'_> {
        resources::Replication_instances::new(self.provider)
    }
    /// Get replication_task_assessment_results resource handler
    pub fn replication_task_assessment_results(&self) -> resources::Replication_task_assessment_results<'_> {
        resources::Replication_task_assessment_results::new(self.provider)
    }
    /// Get fleet_advisor_schema_object_summary resource handler
    pub fn fleet_advisor_schema_object_summary(&self) -> resources::Fleet_advisor_schema_object_summary<'_> {
        resources::Fleet_advisor_schema_object_summary::new(self.provider)
    }
    /// Get replication_subnet_groups resource handler
    pub fn replication_subnet_groups(&self) -> resources::Replication_subnet_groups<'_> {
        resources::Replication_subnet_groups::new(self.provider)
    }
    /// Get migration_project resource handler
    pub fn migration_project(&self) -> resources::Migration_project<'_> {
        resources::Migration_project::new(self.provider)
    }
    /// Get account_attributes resource handler
    pub fn account_attributes(&self) -> resources::Account_attributes<'_> {
        resources::Account_attributes::new(self.provider)
    }
    /// Get recommendation_limitations resource handler
    pub fn recommendation_limitations(&self) -> resources::Recommendation_limitations<'_> {
        resources::Recommendation_limitations::new(self.provider)
    }
    /// Get applicable_individual_assessments resource handler
    pub fn applicable_individual_assessments(&self) -> resources::Applicable_individual_assessments<'_> {
        resources::Applicable_individual_assessments::new(self.provider)
    }
    /// Get connections resource handler
    pub fn connections(&self) -> resources::Connections<'_> {
        resources::Connections::new(self.provider)
    }
    /// Get fleet_advisor_lsa_analysis resource handler
    pub fn fleet_advisor_lsa_analysis(&self) -> resources::Fleet_advisor_lsa_analysis<'_> {
        resources::Fleet_advisor_lsa_analysis::new(self.provider)
    }
    /// Get replication_task_individual_assessments resource handler
    pub fn replication_task_individual_assessments(&self) -> resources::Replication_task_individual_assessments<'_> {
        resources::Replication_task_individual_assessments::new(self.provider)
    }
    /// Get replication_task_assessment_run resource handler
    pub fn replication_task_assessment_run(&self) -> resources::Replication_task_assessment_run<'_> {
        resources::Replication_task_assessment_run::new(self.provider)
    }
    /// Get migration_projects resource handler
    pub fn migration_projects(&self) -> resources::Migration_projects<'_> {
        resources::Migration_projects::new(self.provider)
    }
    /// Get recommendations resource handler
    pub fn recommendations(&self) -> resources::Recommendations<'_> {
        resources::Recommendations::new(self.provider)
    }
    /// Get table_statistics resource handler
    pub fn table_statistics(&self) -> resources::Table_statistics<'_> {
        resources::Table_statistics::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get pending_maintenance_actions resource handler
    pub fn pending_maintenance_actions(&self) -> resources::Pending_maintenance_actions<'_> {
        resources::Pending_maintenance_actions::new(self.provider)
    }
    /// Get schemas resource handler
    pub fn schemas(&self) -> resources::Schemas<'_> {
        resources::Schemas::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get replication_table_statistics resource handler
    pub fn replication_table_statistics(&self) -> resources::Replication_table_statistics<'_> {
        resources::Replication_table_statistics::new(self.provider)
    }
    /// Get data_providers resource handler
    pub fn data_providers(&self) -> resources::Data_providers<'_> {
        resources::Data_providers::new(self.provider)
    }
    /// Get data_migration resource handler
    pub fn data_migration(&self) -> resources::Data_migration<'_> {
        resources::Data_migration::new(self.provider)
    }
    /// Get data_migrations resource handler
    pub fn data_migrations(&self) -> resources::Data_migrations<'_> {
        resources::Data_migrations::new(self.provider)
    }
    /// Get metadata_model_conversions resource handler
    pub fn metadata_model_conversions(&self) -> resources::Metadata_model_conversions<'_> {
        resources::Metadata_model_conversions::new(self.provider)
    }
    /// Get endpoint_types resource handler
    pub fn endpoint_types(&self) -> resources::Endpoint_types<'_> {
        resources::Endpoint_types::new(self.provider)
    }
    /// Get subscriptions_to_event_bridge resource handler
    pub fn subscriptions_to_event_bridge(&self) -> resources::Subscriptions_to_event_bridge<'_> {
        resources::Subscriptions_to_event_bridge::new(self.provider)
    }
    /// Get event_categories resource handler
    pub fn event_categories(&self) -> resources::Event_categories<'_> {
        resources::Event_categories::new(self.provider)
    }
    /// Get replication_instance resource handler
    pub fn replication_instance(&self) -> resources::Replication_instance<'_> {
        resources::Replication_instance::new(self.provider)
    }
    /// Get replication_task_assessment_runs resource handler
    pub fn replication_task_assessment_runs(&self) -> resources::Replication_task_assessment_runs<'_> {
        resources::Replication_task_assessment_runs::new(self.provider)
    }
    /// Get replication_task resource handler
    pub fn replication_task(&self) -> resources::Replication_task<'_> {
        resources::Replication_task::new(self.provider)
    }
    /// Get metadata_model_imports resource handler
    pub fn metadata_model_imports(&self) -> resources::Metadata_model_imports<'_> {
        resources::Metadata_model_imports::new(self.provider)
    }
    /// Get refresh_schemas_status resource handler
    pub fn refresh_schemas_status(&self) -> resources::Refresh_schemas_status<'_> {
        resources::Refresh_schemas_status::new(self.provider)
    }
    /// Get certificate resource handler
    pub fn certificate(&self) -> resources::Certificate<'_> {
        resources::Certificate::new(self.provider)
    }
    /// Get endpoint_settings resource handler
    pub fn endpoint_settings(&self) -> resources::Endpoint_settings<'_> {
        resources::Endpoint_settings::new(self.provider)
    }
    /// Get metadata_model_assessments resource handler
    pub fn metadata_model_assessments(&self) -> resources::Metadata_model_assessments<'_> {
        resources::Metadata_model_assessments::new(self.provider)
    }
    /// Get event_subscription resource handler
    pub fn event_subscription(&self) -> resources::Event_subscription<'_> {
        resources::Event_subscription::new(self.provider)
    }
    /// Get fleet_advisor_schemas resource handler
    pub fn fleet_advisor_schemas(&self) -> resources::Fleet_advisor_schemas<'_> {
        resources::Fleet_advisor_schemas::new(self.provider)
    }
    /// Get metadata_model_exports_as_script resource handler
    pub fn metadata_model_exports_as_script(&self) -> resources::Metadata_model_exports_as_script<'_> {
        resources::Metadata_model_exports_as_script::new(self.provider)
    }
    /// Get instance_profiles resource handler
    pub fn instance_profiles(&self) -> resources::Instance_profiles<'_> {
        resources::Instance_profiles::new(self.provider)
    }
    /// Get fleet_advisor_databases resource handler
    pub fn fleet_advisor_databases(&self) -> resources::Fleet_advisor_databases<'_> {
        resources::Fleet_advisor_databases::new(self.provider)
    }
    /// Get event_subscriptions resource handler
    pub fn event_subscriptions(&self) -> resources::Event_subscriptions<'_> {
        resources::Event_subscriptions::new(self.provider)
    }
    /// Get replication_instance_task_logs resource handler
    pub fn replication_instance_task_logs(&self) -> resources::Replication_instance_task_logs<'_> {
        resources::Replication_instance_task_logs::new(self.provider)
    }
    /// Get replication_config resource handler
    pub fn replication_config(&self) -> resources::Replication_config<'_> {
        resources::Replication_config::new(self.provider)
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
