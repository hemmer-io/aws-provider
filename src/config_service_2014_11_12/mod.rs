//! Config_service_2014_11_12 Service
//!
//! Auto-generated service module for config_service_2014_11_12

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for config_service_2014_11_12
pub struct Config_service_2014_11_12Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Config_service_2014_11_12Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get compliance_summary_by_config_rule resource handler
    pub fn compliance_summary_by_config_rule(&self) -> resources::Compliance_summary_by_config_rule<'_> {
        resources::Compliance_summary_by_config_rule::new(self.provider)
    }
    /// Get stored_query resource handler
    pub fn stored_query(&self) -> resources::Stored_query<'_> {
        resources::Stored_query::new(self.provider)
    }
    /// Get aggregate_config_rule_compliance_summary resource handler
    pub fn aggregate_config_rule_compliance_summary(&self) -> resources::Aggregate_config_rule_compliance_summary<'_> {
        resources::Aggregate_config_rule_compliance_summary::new(self.provider)
    }
    /// Get aggregation_authorization resource handler
    pub fn aggregation_authorization(&self) -> resources::Aggregation_authorization<'_> {
        resources::Aggregation_authorization::new(self.provider)
    }
    /// Get conformance_pack_compliance resource handler
    pub fn conformance_pack_compliance(&self) -> resources::Conformance_pack_compliance<'_> {
        resources::Conformance_pack_compliance::new(self.provider)
    }
    /// Get delivery_channel_status resource handler
    pub fn delivery_channel_status(&self) -> resources::Delivery_channel_status<'_> {
        resources::Delivery_channel_status::new(self.provider)
    }
    /// Get compliance_by_config_rule resource handler
    pub fn compliance_by_config_rule(&self) -> resources::Compliance_by_config_rule<'_> {
        resources::Compliance_by_config_rule::new(self.provider)
    }
    /// Get organization_config_rules resource handler
    pub fn organization_config_rules(&self) -> resources::Organization_config_rules<'_> {
        resources::Organization_config_rules::new(self.provider)
    }
    /// Get pending_aggregation_request resource handler
    pub fn pending_aggregation_request(&self) -> resources::Pending_aggregation_request<'_> {
        resources::Pending_aggregation_request::new(self.provider)
    }
    /// Get remediation_configuration resource handler
    pub fn remediation_configuration(&self) -> resources::Remediation_configuration<'_> {
        resources::Remediation_configuration::new(self.provider)
    }
    /// Get remediation_exceptions resource handler
    pub fn remediation_exceptions(&self) -> resources::Remediation_exceptions<'_> {
        resources::Remediation_exceptions::new(self.provider)
    }
    /// Get organization_conformance_pack_statuses resource handler
    pub fn organization_conformance_pack_statuses(&self) -> resources::Organization_conformance_pack_statuses<'_> {
        resources::Organization_conformance_pack_statuses::new(self.provider)
    }
    /// Get aggregate_conformance_pack_compliance_summary resource handler
    pub fn aggregate_conformance_pack_compliance_summary(&self) -> resources::Aggregate_conformance_pack_compliance_summary<'_> {
        resources::Aggregate_conformance_pack_compliance_summary::new(self.provider)
    }
    /// Get config_rules resource handler
    pub fn config_rules(&self) -> resources::Config_rules<'_> {
        resources::Config_rules::new(self.provider)
    }
    /// Get configuration_aggregator resource handler
    pub fn configuration_aggregator(&self) -> resources::Configuration_aggregator<'_> {
        resources::Configuration_aggregator::new(self.provider)
    }
    /// Get aggregate_compliance_by_config_rules resource handler
    pub fn aggregate_compliance_by_config_rules(&self) -> resources::Aggregate_compliance_by_config_rules<'_> {
        resources::Aggregate_compliance_by_config_rules::new(self.provider)
    }
    /// Get resource_config_history resource handler
    pub fn resource_config_history(&self) -> resources::Resource_config_history<'_> {
        resources::Resource_config_history::new(self.provider)
    }
    /// Get aggregate_compliance_by_conformance_packs resource handler
    pub fn aggregate_compliance_by_conformance_packs(&self) -> resources::Aggregate_compliance_by_conformance_packs<'_> {
        resources::Aggregate_compliance_by_conformance_packs::new(self.provider)
    }
    /// Get pending_aggregation_requests resource handler
    pub fn pending_aggregation_requests(&self) -> resources::Pending_aggregation_requests<'_> {
        resources::Pending_aggregation_requests::new(self.provider)
    }
    /// Get retention_configuration resource handler
    pub fn retention_configuration(&self) -> resources::Retention_configuration<'_> {
        resources::Retention_configuration::new(self.provider)
    }
    /// Get compliance_summary_by_resource_type resource handler
    pub fn compliance_summary_by_resource_type(&self) -> resources::Compliance_summary_by_resource_type<'_> {
        resources::Compliance_summary_by_resource_type::new(self.provider)
    }
    /// Get compliance_details_by_resource resource handler
    pub fn compliance_details_by_resource(&self) -> resources::Compliance_details_by_resource<'_> {
        resources::Compliance_details_by_resource::new(self.provider)
    }
    /// Get conformance_pack_compliance_summary resource handler
    pub fn conformance_pack_compliance_summary(&self) -> resources::Conformance_pack_compliance_summary<'_> {
        resources::Conformance_pack_compliance_summary::new(self.provider)
    }
    /// Get organization_conformance_pack resource handler
    pub fn organization_conformance_pack(&self) -> resources::Organization_conformance_pack<'_> {
        resources::Organization_conformance_pack::new(self.provider)
    }
    /// Get delivery_channel resource handler
    pub fn delivery_channel(&self) -> resources::Delivery_channel<'_> {
        resources::Delivery_channel::new(self.provider)
    }
    /// Get organization_custom_rule_policy resource handler
    pub fn organization_custom_rule_policy(&self) -> resources::Organization_custom_rule_policy<'_> {
        resources::Organization_custom_rule_policy::new(self.provider)
    }
    /// Get aggregate_compliance_details_by_config_rule resource handler
    pub fn aggregate_compliance_details_by_config_rule(&self) -> resources::Aggregate_compliance_details_by_config_rule<'_> {
        resources::Aggregate_compliance_details_by_config_rule::new(self.provider)
    }
    /// Get compliance_details_by_config_rule resource handler
    pub fn compliance_details_by_config_rule(&self) -> resources::Compliance_details_by_config_rule<'_> {
        resources::Compliance_details_by_config_rule::new(self.provider)
    }
    /// Get delivery_channels resource handler
    pub fn delivery_channels(&self) -> resources::Delivery_channels<'_> {
        resources::Delivery_channels::new(self.provider)
    }
    /// Get resource_evaluation_summary resource handler
    pub fn resource_evaluation_summary(&self) -> resources::Resource_evaluation_summary<'_> {
        resources::Resource_evaluation_summary::new(self.provider)
    }
    /// Get custom_rule_policy resource handler
    pub fn custom_rule_policy(&self) -> resources::Custom_rule_policy<'_> {
        resources::Custom_rule_policy::new(self.provider)
    }
    /// Get evaluations resource handler
    pub fn evaluations(&self) -> resources::Evaluations<'_> {
        resources::Evaluations::new(self.provider)
    }
    /// Get evaluation_results resource handler
    pub fn evaluation_results(&self) -> resources::Evaluation_results<'_> {
        resources::Evaluation_results::new(self.provider)
    }
    /// Get retention_configurations resource handler
    pub fn retention_configurations(&self) -> resources::Retention_configurations<'_> {
        resources::Retention_configurations::new(self.provider)
    }
    /// Get configuration_aggregators resource handler
    pub fn configuration_aggregators(&self) -> resources::Configuration_aggregators<'_> {
        resources::Configuration_aggregators::new(self.provider)
    }
    /// Get configuration_recorders resource handler
    pub fn configuration_recorders(&self) -> resources::Configuration_recorders<'_> {
        resources::Configuration_recorders::new(self.provider)
    }
    /// Get service_linked_configuration_recorder resource handler
    pub fn service_linked_configuration_recorder(&self) -> resources::Service_linked_configuration_recorder<'_> {
        resources::Service_linked_configuration_recorder::new(self.provider)
    }
    /// Get external_evaluation resource handler
    pub fn external_evaluation(&self) -> resources::External_evaluation<'_> {
        resources::External_evaluation::new(self.provider)
    }
    /// Get conformance_pack_compliance_details resource handler
    pub fn conformance_pack_compliance_details(&self) -> resources::Conformance_pack_compliance_details<'_> {
        resources::Conformance_pack_compliance_details::new(self.provider)
    }
    /// Get discovered_resource_counts resource handler
    pub fn discovered_resource_counts(&self) -> resources::Discovered_resource_counts<'_> {
        resources::Discovered_resource_counts::new(self.provider)
    }
    /// Get conformance_packs resource handler
    pub fn conformance_packs(&self) -> resources::Conformance_packs<'_> {
        resources::Conformance_packs::new(self.provider)
    }
    /// Get aggregation_authorizations resource handler
    pub fn aggregation_authorizations(&self) -> resources::Aggregation_authorizations<'_> {
        resources::Aggregation_authorizations::new(self.provider)
    }
    /// Get organization_conformance_pack_detailed_status resource handler
    pub fn organization_conformance_pack_detailed_status(&self) -> resources::Organization_conformance_pack_detailed_status<'_> {
        resources::Organization_conformance_pack_detailed_status::new(self.provider)
    }
    /// Get compliance_by_resource resource handler
    pub fn compliance_by_resource(&self) -> resources::Compliance_by_resource<'_> {
        resources::Compliance_by_resource::new(self.provider)
    }
    /// Get remediation_configurations resource handler
    pub fn remediation_configurations(&self) -> resources::Remediation_configurations<'_> {
        resources::Remediation_configurations::new(self.provider)
    }
    /// Get resource_config resource handler
    pub fn resource_config(&self) -> resources::Resource_config<'_> {
        resources::Resource_config::new(self.provider)
    }
    /// Get remediation_execution_status resource handler
    pub fn remediation_execution_status(&self) -> resources::Remediation_execution_status<'_> {
        resources::Remediation_execution_status::new(self.provider)
    }
    /// Get aggregate_discovered_resource_counts resource handler
    pub fn aggregate_discovered_resource_counts(&self) -> resources::Aggregate_discovered_resource_counts<'_> {
        resources::Aggregate_discovered_resource_counts::new(self.provider)
    }
    /// Get config_rule_evaluation_status resource handler
    pub fn config_rule_evaluation_status(&self) -> resources::Config_rule_evaluation_status<'_> {
        resources::Config_rule_evaluation_status::new(self.provider)
    }
    /// Get organization_conformance_packs resource handler
    pub fn organization_conformance_packs(&self) -> resources::Organization_conformance_packs<'_> {
        resources::Organization_conformance_packs::new(self.provider)
    }
    /// Get organization_config_rule resource handler
    pub fn organization_config_rule(&self) -> resources::Organization_config_rule<'_> {
        resources::Organization_config_rule::new(self.provider)
    }
    /// Get aggregate_resource_config resource handler
    pub fn aggregate_resource_config(&self) -> resources::Aggregate_resource_config<'_> {
        resources::Aggregate_resource_config::new(self.provider)
    }
    /// Get conformance_pack_status resource handler
    pub fn conformance_pack_status(&self) -> resources::Conformance_pack_status<'_> {
        resources::Conformance_pack_status::new(self.provider)
    }
    /// Get organization_config_rule_statuses resource handler
    pub fn organization_config_rule_statuses(&self) -> resources::Organization_config_rule_statuses<'_> {
        resources::Organization_config_rule_statuses::new(self.provider)
    }
    /// Get configuration_aggregator_sources_status resource handler
    pub fn configuration_aggregator_sources_status(&self) -> resources::Configuration_aggregator_sources_status<'_> {
        resources::Configuration_aggregator_sources_status::new(self.provider)
    }
    /// Get conformance_pack resource handler
    pub fn conformance_pack(&self) -> resources::Conformance_pack<'_> {
        resources::Conformance_pack::new(self.provider)
    }
    /// Get configuration_recorder resource handler
    pub fn configuration_recorder(&self) -> resources::Configuration_recorder<'_> {
        resources::Configuration_recorder::new(self.provider)
    }
    /// Get organization_config_rule_detailed_status resource handler
    pub fn organization_config_rule_detailed_status(&self) -> resources::Organization_config_rule_detailed_status<'_> {
        resources::Organization_config_rule_detailed_status::new(self.provider)
    }
    /// Get config_rule resource handler
    pub fn config_rule(&self) -> resources::Config_rule<'_> {
        resources::Config_rule::new(self.provider)
    }
    /// Get configuration_recorder_status resource handler
    pub fn configuration_recorder_status(&self) -> resources::Configuration_recorder_status<'_> {
        resources::Configuration_recorder_status::new(self.provider)
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
