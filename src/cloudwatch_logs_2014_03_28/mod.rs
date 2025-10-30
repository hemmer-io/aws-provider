//! Cloudwatch_logs_2014_03_28 Service
//!
//! Auto-generated service module for cloudwatch_logs_2014_03_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudwatch_logs_2014_03_28
pub struct Cloudwatch_logs_2014_03_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudwatch_logs_2014_03_28Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get log_groups resource handler
    pub fn log_groups(&self) -> resources::Log_groups<'_> {
        resources::Log_groups::new(self.provider)
    }
    /// Get log_group_fields resource handler
    pub fn log_group_fields(&self) -> resources::Log_group_fields<'_> {
        resources::Log_group_fields::new(self.provider)
    }
    /// Get delivery_destination resource handler
    pub fn delivery_destination(&self) -> resources::Delivery_destination<'_> {
        resources::Delivery_destination::new(self.provider)
    }
    /// Get field_indexes resource handler
    pub fn field_indexes(&self) -> resources::Field_indexes<'_> {
        resources::Field_indexes::new(self.provider)
    }
    /// Get index_policy resource handler
    pub fn index_policy(&self) -> resources::Index_policy<'_> {
        resources::Index_policy::new(self.provider)
    }
    /// Get log_record resource handler
    pub fn log_record(&self) -> resources::Log_record<'_> {
        resources::Log_record::new(self.provider)
    }
    /// Get log_group resource handler
    pub fn log_group(&self) -> resources::Log_group<'_> {
        resources::Log_group::new(self.provider)
    }
    /// Get anomaly resource handler
    pub fn anomaly(&self) -> resources::Anomaly<'_> {
        resources::Anomaly::new(self.provider)
    }
    /// Get export_task resource handler
    pub fn export_task(&self) -> resources::Export_task<'_> {
        resources::Export_task::new(self.provider)
    }
    /// Get delivery_destinations resource handler
    pub fn delivery_destinations(&self) -> resources::Delivery_destinations<'_> {
        resources::Delivery_destinations::new(self.provider)
    }
    /// Get delivery_configuration resource handler
    pub fn delivery_configuration(&self) -> resources::Delivery_configuration<'_> {
        resources::Delivery_configuration::new(self.provider)
    }
    /// Get metric_filter resource handler
    pub fn metric_filter(&self) -> resources::Metric_filter<'_> {
        resources::Metric_filter::new(self.provider)
    }
    /// Get log_anomaly_detector resource handler
    pub fn log_anomaly_detector(&self) -> resources::Log_anomaly_detector<'_> {
        resources::Log_anomaly_detector::new(self.provider)
    }
    /// Get data_protection_policy resource handler
    pub fn data_protection_policy(&self) -> resources::Data_protection_policy<'_> {
        resources::Data_protection_policy::new(self.provider)
    }
    /// Get log_object resource handler
    pub fn log_object(&self) -> resources::Log_object<'_> {
        resources::Log_object::new(self.provider)
    }
    /// Get metric_filters resource handler
    pub fn metric_filters(&self) -> resources::Metric_filters<'_> {
        resources::Metric_filters::new(self.provider)
    }
    /// Get query_results resource handler
    pub fn query_results(&self) -> resources::Query_results<'_> {
        resources::Query_results::new(self.provider)
    }
    /// Get destination_policy resource handler
    pub fn destination_policy(&self) -> resources::Destination_policy<'_> {
        resources::Destination_policy::new(self.provider)
    }
    /// Get delivery_destination_policy resource handler
    pub fn delivery_destination_policy(&self) -> resources::Delivery_destination_policy<'_> {
        resources::Delivery_destination_policy::new(self.provider)
    }
    /// Get resource_policies resource handler
    pub fn resource_policies(&self) -> resources::Resource_policies<'_> {
        resources::Resource_policies::new(self.provider)
    }
    /// Get delivery resource handler
    pub fn delivery(&self) -> resources::Delivery<'_> {
        resources::Delivery::new(self.provider)
    }
    /// Get query_definition resource handler
    pub fn query_definition(&self) -> resources::Query_definition<'_> {
        resources::Query_definition::new(self.provider)
    }
    /// Get transformer resource handler
    pub fn transformer(&self) -> resources::Transformer<'_> {
        resources::Transformer::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get log_streams resource handler
    pub fn log_streams(&self) -> resources::Log_streams<'_> {
        resources::Log_streams::new(self.provider)
    }
    /// Get export_tasks resource handler
    pub fn export_tasks(&self) -> resources::Export_tasks<'_> {
        resources::Export_tasks::new(self.provider)
    }
    /// Get delivery_sources resource handler
    pub fn delivery_sources(&self) -> resources::Delivery_sources<'_> {
        resources::Delivery_sources::new(self.provider)
    }
    /// Get destination resource handler
    pub fn destination(&self) -> resources::Destination<'_> {
        resources::Destination::new(self.provider)
    }
    /// Get retention_policy resource handler
    pub fn retention_policy(&self) -> resources::Retention_policy<'_> {
        resources::Retention_policy::new(self.provider)
    }
    /// Get subscription_filters resource handler
    pub fn subscription_filters(&self) -> resources::Subscription_filters<'_> {
        resources::Subscription_filters::new(self.provider)
    }
    /// Get account_policy resource handler
    pub fn account_policy(&self) -> resources::Account_policy<'_> {
        resources::Account_policy::new(self.provider)
    }
    /// Get account_policies resource handler
    pub fn account_policies(&self) -> resources::Account_policies<'_> {
        resources::Account_policies::new(self.provider)
    }
    /// Get log_stream resource handler
    pub fn log_stream(&self) -> resources::Log_stream<'_> {
        resources::Log_stream::new(self.provider)
    }
    /// Get log_events resource handler
    pub fn log_events(&self) -> resources::Log_events<'_> {
        resources::Log_events::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get deliveries resource handler
    pub fn deliveries(&self) -> resources::Deliveries<'_> {
        resources::Deliveries::new(self.provider)
    }
    /// Get index_policies resource handler
    pub fn index_policies(&self) -> resources::Index_policies<'_> {
        resources::Index_policies::new(self.provider)
    }
    /// Get queries resource handler
    pub fn queries(&self) -> resources::Queries<'_> {
        resources::Queries::new(self.provider)
    }
    /// Get destinations resource handler
    pub fn destinations(&self) -> resources::Destinations<'_> {
        resources::Destinations::new(self.provider)
    }
    /// Get subscription_filter resource handler
    pub fn subscription_filter(&self) -> resources::Subscription_filter<'_> {
        resources::Subscription_filter::new(self.provider)
    }
    /// Get query_definitions resource handler
    pub fn query_definitions(&self) -> resources::Query_definitions<'_> {
        resources::Query_definitions::new(self.provider)
    }
    /// Get configuration_templates resource handler
    pub fn configuration_templates(&self) -> resources::Configuration_templates<'_> {
        resources::Configuration_templates::new(self.provider)
    }
    /// Get delivery_source resource handler
    pub fn delivery_source(&self) -> resources::Delivery_source<'_> {
        resources::Delivery_source::new(self.provider)
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
