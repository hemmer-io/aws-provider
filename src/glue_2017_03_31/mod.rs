//! Glue_2017_03_31 Service
//!
//! Auto-generated service module for glue_2017_03_31

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for glue_2017_03_31
pub struct Glue_2017_03_31Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Glue_2017_03_31Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get catalog resource handler
    pub fn catalog(&self) -> resources::Catalog<'_> {
        resources::Catalog::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get data_quality_model_result resource handler
    pub fn data_quality_model_result(&self) -> resources::Data_quality_model_result<'_> {
        resources::Data_quality_model_result::new(self.provider)
    }
    /// Get crawlers resource handler
    pub fn crawlers(&self) -> resources::Crawlers<'_> {
        resources::Crawlers::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get column_statistics_task_run resource handler
    pub fn column_statistics_task_run(&self) -> resources::Column_statistics_task_run<'_> {
        resources::Column_statistics_task_run::new(self.provider)
    }
    /// Get triggers resource handler
    pub fn triggers(&self) -> resources::Triggers<'_> {
        resources::Triggers::new(self.provider)
    }
    /// Get schema_by_definition resource handler
    pub fn schema_by_definition(&self) -> resources::Schema_by_definition<'_> {
        resources::Schema_by_definition::new(self.provider)
    }
    /// Get user_defined_functions resource handler
    pub fn user_defined_functions(&self) -> resources::User_defined_functions<'_> {
        resources::User_defined_functions::new(self.provider)
    }
    /// Get workflow_runs resource handler
    pub fn workflow_runs(&self) -> resources::Workflow_runs<'_> {
        resources::Workflow_runs::new(self.provider)
    }
    /// Get ml_task_runs resource handler
    pub fn ml_task_runs(&self) -> resources::Ml_task_runs<'_> {
        resources::Ml_task_runs::new(self.provider)
    }
    /// Get table resource handler
    pub fn table(&self) -> resources::Table<'_> {
        resources::Table::new(self.provider)
    }
    /// Get blueprint resource handler
    pub fn blueprint(&self) -> resources::Blueprint<'_> {
        resources::Blueprint::new(self.provider)
    }
    /// Get connections resource handler
    pub fn connections(&self) -> resources::Connections<'_> {
        resources::Connections::new(self.provider)
    }
    /// Get data_quality_profile_annotation resource handler
    pub fn data_quality_profile_annotation(&self) -> resources::Data_quality_profile_annotation<'_> {
        resources::Data_quality_profile_annotation::new(self.provider)
    }
    /// Get plan resource handler
    pub fn plan(&self) -> resources::Plan<'_> {
        resources::Plan::new(self.provider)
    }
    /// Get crawler resource handler
    pub fn crawler(&self) -> resources::Crawler<'_> {
        resources::Crawler::new(self.provider)
    }
    /// Get unfiltered_table_metadata resource handler
    pub fn unfiltered_table_metadata(&self) -> resources::Unfiltered_table_metadata<'_> {
        resources::Unfiltered_table_metadata::new(self.provider)
    }
    /// Get column_statistics_task_settings resource handler
    pub fn column_statistics_task_settings(&self) -> resources::Column_statistics_task_settings<'_> {
        resources::Column_statistics_task_settings::new(self.provider)
    }
    /// Get connection_type resource handler
    pub fn connection_type(&self) -> resources::Connection_type<'_> {
        resources::Connection_type::new(self.provider)
    }
    /// Get security_configurations resource handler
    pub fn security_configurations(&self) -> resources::Security_configurations<'_> {
        resources::Security_configurations::new(self.provider)
    }
    /// Get custom_entity_type resource handler
    pub fn custom_entity_type(&self) -> resources::Custom_entity_type<'_> {
        resources::Custom_entity_type::new(self.provider)
    }
    /// Get registry resource handler
    pub fn registry(&self) -> resources::Registry<'_> {
        resources::Registry::new(self.provider)
    }
    /// Get trigger resource handler
    pub fn trigger(&self) -> resources::Trigger<'_> {
        resources::Trigger::new(self.provider)
    }
    /// Get schema_versions resource handler
    pub fn schema_versions(&self) -> resources::Schema_versions<'_> {
        resources::Schema_versions::new(self.provider)
    }
    /// Get data_quality_rule_recommendation_run resource handler
    pub fn data_quality_rule_recommendation_run(&self) -> resources::Data_quality_rule_recommendation_run<'_> {
        resources::Data_quality_rule_recommendation_run::new(self.provider)
    }
    /// Get job_runs resource handler
    pub fn job_runs(&self) -> resources::Job_runs<'_> {
        resources::Job_runs::new(self.provider)
    }
    /// Get unfiltered_partitions_metadata resource handler
    pub fn unfiltered_partitions_metadata(&self) -> resources::Unfiltered_partitions_metadata<'_> {
        resources::Unfiltered_partitions_metadata::new(self.provider)
    }
    /// Get inbound_integrations resource handler
    pub fn inbound_integrations(&self) -> resources::Inbound_integrations<'_> {
        resources::Inbound_integrations::new(self.provider)
    }
    /// Get job_run resource handler
    pub fn job_run(&self) -> resources::Job_run<'_> {
        resources::Job_run::new(self.provider)
    }
    /// Get data_catalog_encryption_settings resource handler
    pub fn data_catalog_encryption_settings(&self) -> resources::Data_catalog_encryption_settings<'_> {
        resources::Data_catalog_encryption_settings::new(self.provider)
    }
    /// Get column_statistics_for_table resource handler
    pub fn column_statistics_for_table(&self) -> resources::Column_statistics_for_table<'_> {
        resources::Column_statistics_for_table::new(self.provider)
    }
    /// Get column_statistics_task_runs resource handler
    pub fn column_statistics_task_runs(&self) -> resources::Column_statistics_task_runs<'_> {
        resources::Column_statistics_task_runs::new(self.provider)
    }
    /// Get dev_endpoint resource handler
    pub fn dev_endpoint(&self) -> resources::Dev_endpoint<'_> {
        resources::Dev_endpoint::new(self.provider)
    }
    /// Get schema resource handler
    pub fn schema(&self) -> resources::Schema<'_> {
        resources::Schema::new(self.provider)
    }
    /// Get schema_version_metadata resource handler
    pub fn schema_version_metadata(&self) -> resources::Schema_version_metadata<'_> {
        resources::Schema_version_metadata::new(self.provider)
    }
    /// Get table_versions resource handler
    pub fn table_versions(&self) -> resources::Table_versions<'_> {
        resources::Table_versions::new(self.provider)
    }
    /// Get partition resource handler
    pub fn partition(&self) -> resources::Partition<'_> {
        resources::Partition::new(self.provider)
    }
    /// Get column_statistics_for_partition resource handler
    pub fn column_statistics_for_partition(&self) -> resources::Column_statistics_for_partition<'_> {
        resources::Column_statistics_for_partition::new(self.provider)
    }
    /// Get workflow_run_properties resource handler
    pub fn workflow_run_properties(&self) -> resources::Workflow_run_properties<'_> {
        resources::Workflow_run_properties::new(self.provider)
    }
    /// Get integration_table_properties resource handler
    pub fn integration_table_properties(&self) -> resources::Integration_table_properties<'_> {
        resources::Integration_table_properties::new(self.provider)
    }
    /// Get integrations resource handler
    pub fn integrations(&self) -> resources::Integrations<'_> {
        resources::Integrations::new(self.provider)
    }
    /// Get blueprint_runs resource handler
    pub fn blueprint_runs(&self) -> resources::Blueprint_runs<'_> {
        resources::Blueprint_runs::new(self.provider)
    }
    /// Get integration_resource_property resource handler
    pub fn integration_resource_property(&self) -> resources::Integration_resource_property<'_> {
        resources::Integration_resource_property::new(self.provider)
    }
    /// Get job_bookmark resource handler
    pub fn job_bookmark(&self) -> resources::Job_bookmark<'_> {
        resources::Job_bookmark::new(self.provider)
    }
    /// Get unfiltered_partition_metadata resource handler
    pub fn unfiltered_partition_metadata(&self) -> resources::Unfiltered_partition_metadata<'_> {
        resources::Unfiltered_partition_metadata::new(self.provider)
    }
    /// Get workflow resource handler
    pub fn workflow(&self) -> resources::Workflow<'_> {
        resources::Workflow::new(self.provider)
    }
    /// Get data_quality_result resource handler
    pub fn data_quality_result(&self) -> resources::Data_quality_result<'_> {
        resources::Data_quality_result::new(self.provider)
    }
    /// Get jobs resource handler
    pub fn jobs(&self) -> resources::Jobs<'_> {
        resources::Jobs::new(self.provider)
    }
    /// Get schema_versions_diff resource handler
    pub fn schema_versions_diff(&self) -> resources::Schema_versions_diff<'_> {
        resources::Schema_versions_diff::new(self.provider)
    }
    /// Get glue_identity_center_configuration resource handler
    pub fn glue_identity_center_configuration(&self) -> resources::Glue_identity_center_configuration<'_> {
        resources::Glue_identity_center_configuration::new(self.provider)
    }
    /// Get workflow_run resource handler
    pub fn workflow_run(&self) -> resources::Workflow_run<'_> {
        resources::Workflow_run::new(self.provider)
    }
    /// Get source_control_from_job resource handler
    pub fn source_control_from_job(&self) -> resources::Source_control_from_job<'_> {
        resources::Source_control_from_job::new(self.provider)
    }
    /// Get catalogs resource handler
    pub fn catalogs(&self) -> resources::Catalogs<'_> {
        resources::Catalogs::new(self.provider)
    }
    /// Get resource_policies resource handler
    pub fn resource_policies(&self) -> resources::Resource_policies<'_> {
        resources::Resource_policies::new(self.provider)
    }
    /// Get tables resource handler
    pub fn tables(&self) -> resources::Tables<'_> {
        resources::Tables::new(self.provider)
    }
    /// Get table_version resource handler
    pub fn table_version(&self) -> resources::Table_version<'_> {
        resources::Table_version::new(self.provider)
    }
    /// Get data_quality_ruleset_evaluation_run resource handler
    pub fn data_quality_ruleset_evaluation_run(&self) -> resources::Data_quality_ruleset_evaluation_run<'_> {
        resources::Data_quality_ruleset_evaluation_run::new(self.provider)
    }
    /// Get catalog_import_status resource handler
    pub fn catalog_import_status(&self) -> resources::Catalog_import_status<'_> {
        resources::Catalog_import_status::new(self.provider)
    }
    /// Get data_quality_ruleset resource handler
    pub fn data_quality_ruleset(&self) -> resources::Data_quality_ruleset<'_> {
        resources::Data_quality_ruleset::new(self.provider)
    }
    /// Get partition_indexes resource handler
    pub fn partition_indexes(&self) -> resources::Partition_indexes<'_> {
        resources::Partition_indexes::new(self.provider)
    }
    /// Get schema_version resource handler
    pub fn schema_version(&self) -> resources::Schema_version<'_> {
        resources::Schema_version::new(self.provider)
    }
    /// Get security_configuration resource handler
    pub fn security_configuration(&self) -> resources::Security_configuration<'_> {
        resources::Security_configuration::new(self.provider)
    }
    /// Get statement resource handler
    pub fn statement(&self) -> resources::Statement<'_> {
        resources::Statement::new(self.provider)
    }
    /// Get script resource handler
    pub fn script(&self) -> resources::Script<'_> {
        resources::Script::new(self.provider)
    }
    /// Get partitions resource handler
    pub fn partitions(&self) -> resources::Partitions<'_> {
        resources::Partitions::new(self.provider)
    }
    /// Get partition_index resource handler
    pub fn partition_index(&self) -> resources::Partition_index<'_> {
        resources::Partition_index::new(self.provider)
    }
    /// Get entity_records resource handler
    pub fn entity_records(&self) -> resources::Entity_records<'_> {
        resources::Entity_records::new(self.provider)
    }
    /// Get crawler_schedule resource handler
    pub fn crawler_schedule(&self) -> resources::Crawler_schedule<'_> {
        resources::Crawler_schedule::new(self.provider)
    }
    /// Get job_from_source_control resource handler
    pub fn job_from_source_control(&self) -> resources::Job_from_source_control<'_> {
        resources::Job_from_source_control::new(self.provider)
    }
    /// Get ml_transform resource handler
    pub fn ml_transform(&self) -> resources::Ml_transform<'_> {
        resources::Ml_transform::new(self.provider)
    }
    /// Get data_quality_model resource handler
    pub fn data_quality_model(&self) -> resources::Data_quality_model<'_> {
        resources::Data_quality_model::new(self.provider)
    }
    /// Get table_optimizer resource handler
    pub fn table_optimizer(&self) -> resources::Table_optimizer<'_> {
        resources::Table_optimizer::new(self.provider)
    }
    /// Get crawler_metrics resource handler
    pub fn crawler_metrics(&self) -> resources::Crawler_metrics<'_> {
        resources::Crawler_metrics::new(self.provider)
    }
    /// Get ml_transforms resource handler
    pub fn ml_transforms(&self) -> resources::Ml_transforms<'_> {
        resources::Ml_transforms::new(self.provider)
    }
    /// Get databases resource handler
    pub fn databases(&self) -> resources::Databases<'_> {
        resources::Databases::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get database resource handler
    pub fn database(&self) -> resources::Database<'_> {
        resources::Database::new(self.provider)
    }
    /// Get user_defined_function resource handler
    pub fn user_defined_function(&self) -> resources::User_defined_function<'_> {
        resources::User_defined_function::new(self.provider)
    }
    /// Get usage_profile resource handler
    pub fn usage_profile(&self) -> resources::Usage_profile<'_> {
        resources::Usage_profile::new(self.provider)
    }
    /// Get dataflow_graph resource handler
    pub fn dataflow_graph(&self) -> resources::Dataflow_graph<'_> {
        resources::Dataflow_graph::new(self.provider)
    }
    /// Get classifier resource handler
    pub fn classifier(&self) -> resources::Classifier<'_> {
        resources::Classifier::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get blueprint_run resource handler
    pub fn blueprint_run(&self) -> resources::Blueprint_run<'_> {
        resources::Blueprint_run::new(self.provider)
    }
    /// Get dev_endpoints resource handler
    pub fn dev_endpoints(&self) -> resources::Dev_endpoints<'_> {
        resources::Dev_endpoints::new(self.provider)
    }
    /// Get ml_task_run resource handler
    pub fn ml_task_run(&self) -> resources::Ml_task_run<'_> {
        resources::Ml_task_run::new(self.provider)
    }
    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
    }
    /// Get entity resource handler
    pub fn entity(&self) -> resources::Entity<'_> {
        resources::Entity::new(self.provider)
    }
    /// Get classifiers resource handler
    pub fn classifiers(&self) -> resources::Classifiers<'_> {
        resources::Classifiers::new(self.provider)
    }
    /// Get mapping resource handler
    pub fn mapping(&self) -> resources::Mapping<'_> {
        resources::Mapping::new(self.provider)
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
