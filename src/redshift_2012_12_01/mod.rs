//! Redshift_2012_12_01 Service
//!
//! Auto-generated service module for redshift_2012_12_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for redshift_2012_12_01
pub struct Redshift_2012_12_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Redshift_2012_12_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get logging_status resource handler
    pub fn logging_status(&self) -> resources::Logging_status<'_> {
        resources::Logging_status::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get hsm_client_certificates resource handler
    pub fn hsm_client_certificates(&self) -> resources::Hsm_client_certificates<'_> {
        resources::Hsm_client_certificates::new(self.provider)
    }
    /// Get default_cluster_parameters resource handler
    pub fn default_cluster_parameters(&self) -> resources::Default_cluster_parameters<'_> {
        resources::Default_cluster_parameters::new(self.provider)
    }
    /// Get authentication_profiles resource handler
    pub fn authentication_profiles(&self) -> resources::Authentication_profiles<'_> {
        resources::Authentication_profiles::new(self.provider)
    }
    /// Get reserved_node_exchange_status resource handler
    pub fn reserved_node_exchange_status(&self) -> resources::Reserved_node_exchange_status<'_> {
        resources::Reserved_node_exchange_status::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get snapshot_schedule resource handler
    pub fn snapshot_schedule(&self) -> resources::Snapshot_schedule<'_> {
        resources::Snapshot_schedule::new(self.provider)
    }
    /// Get usage_limits resource handler
    pub fn usage_limits(&self) -> resources::Usage_limits<'_> {
        resources::Usage_limits::new(self.provider)
    }
    /// Get data_shares_for_producer resource handler
    pub fn data_shares_for_producer(&self) -> resources::Data_shares_for_producer<'_> {
        resources::Data_shares_for_producer::new(self.provider)
    }
    /// Get snapshot_copy_grants resource handler
    pub fn snapshot_copy_grants(&self) -> resources::Snapshot_copy_grants<'_> {
        resources::Snapshot_copy_grants::new(self.provider)
    }
    /// Get snapshot_schedules resource handler
    pub fn snapshot_schedules(&self) -> resources::Snapshot_schedules<'_> {
        resources::Snapshot_schedules::new(self.provider)
    }
    /// Get redshift_idc_applications resource handler
    pub fn redshift_idc_applications(&self) -> resources::Redshift_idc_applications<'_> {
        resources::Redshift_idc_applications::new(self.provider)
    }
    /// Get orderable_cluster_options resource handler
    pub fn orderable_cluster_options(&self) -> resources::Orderable_cluster_options<'_> {
        resources::Orderable_cluster_options::new(self.provider)
    }
    /// Get cluster_versions resource handler
    pub fn cluster_versions(&self) -> resources::Cluster_versions<'_> {
        resources::Cluster_versions::new(self.provider)
    }
    /// Get cluster_security_groups resource handler
    pub fn cluster_security_groups(&self) -> resources::Cluster_security_groups<'_> {
        resources::Cluster_security_groups::new(self.provider)
    }
    /// Get reserved_nodes resource handler
    pub fn reserved_nodes(&self) -> resources::Reserved_nodes<'_> {
        resources::Reserved_nodes::new(self.provider)
    }
    /// Get cluster_tracks resource handler
    pub fn cluster_tracks(&self) -> resources::Cluster_tracks<'_> {
        resources::Cluster_tracks::new(self.provider)
    }
    /// Get account_attributes resource handler
    pub fn account_attributes(&self) -> resources::Account_attributes<'_> {
        resources::Account_attributes::new(self.provider)
    }
    /// Get cluster_snapshot resource handler
    pub fn cluster_snapshot(&self) -> resources::Cluster_snapshot<'_> {
        resources::Cluster_snapshot::new(self.provider)
    }
    /// Get cluster_db_revisions resource handler
    pub fn cluster_db_revisions(&self) -> resources::Cluster_db_revisions<'_> {
        resources::Cluster_db_revisions::new(self.provider)
    }
    /// Get cluster_snapshots resource handler
    pub fn cluster_snapshots(&self) -> resources::Cluster_snapshots<'_> {
        resources::Cluster_snapshots::new(self.provider)
    }
    /// Get cluster_subnet_groups resource handler
    pub fn cluster_subnet_groups(&self) -> resources::Cluster_subnet_groups<'_> {
        resources::Cluster_subnet_groups::new(self.provider)
    }
    /// Get data_shares resource handler
    pub fn data_shares(&self) -> resources::Data_shares<'_> {
        resources::Data_shares::new(self.provider)
    }
    /// Get event_subscription resource handler
    pub fn event_subscription(&self) -> resources::Event_subscription<'_> {
        resources::Event_subscription::new(self.provider)
    }
    /// Get event_categories resource handler
    pub fn event_categories(&self) -> resources::Event_categories<'_> {
        resources::Event_categories::new(self.provider)
    }
    /// Get hsm_configurations resource handler
    pub fn hsm_configurations(&self) -> resources::Hsm_configurations<'_> {
        resources::Hsm_configurations::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get reserved_node_exchange_offerings resource handler
    pub fn reserved_node_exchange_offerings(&self) -> resources::Reserved_node_exchange_offerings<'_> {
        resources::Reserved_node_exchange_offerings::new(self.provider)
    }
    /// Get partner_status resource handler
    pub fn partner_status(&self) -> resources::Partner_status<'_> {
        resources::Partner_status::new(self.provider)
    }
    /// Get authentication_profile resource handler
    pub fn authentication_profile(&self) -> resources::Authentication_profile<'_> {
        resources::Authentication_profile::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get data_shares_for_consumer resource handler
    pub fn data_shares_for_consumer(&self) -> resources::Data_shares_for_consumer<'_> {
        resources::Data_shares_for_consumer::new(self.provider)
    }
    /// Get clusters resource handler
    pub fn clusters(&self) -> resources::Clusters<'_> {
        resources::Clusters::new(self.provider)
    }
    /// Get scheduled_action resource handler
    pub fn scheduled_action(&self) -> resources::Scheduled_action<'_> {
        resources::Scheduled_action::new(self.provider)
    }
    /// Get cluster_credentials resource handler
    pub fn cluster_credentials(&self) -> resources::Cluster_credentials<'_> {
        resources::Cluster_credentials::new(self.provider)
    }
    /// Get cluster_parameters resource handler
    pub fn cluster_parameters(&self) -> resources::Cluster_parameters<'_> {
        resources::Cluster_parameters::new(self.provider)
    }
    /// Get usage_limit resource handler
    pub fn usage_limit(&self) -> resources::Usage_limit<'_> {
        resources::Usage_limit::new(self.provider)
    }
    /// Get cluster_security_group resource handler
    pub fn cluster_security_group(&self) -> resources::Cluster_security_group<'_> {
        resources::Cluster_security_group::new(self.provider)
    }
    /// Get cluster_subnet_group resource handler
    pub fn cluster_subnet_group(&self) -> resources::Cluster_subnet_group<'_> {
        resources::Cluster_subnet_group::new(self.provider)
    }
    /// Get scheduled_actions resource handler
    pub fn scheduled_actions(&self) -> resources::Scheduled_actions<'_> {
        resources::Scheduled_actions::new(self.provider)
    }
    /// Get hsm_configuration resource handler
    pub fn hsm_configuration(&self) -> resources::Hsm_configuration<'_> {
        resources::Hsm_configuration::new(self.provider)
    }
    /// Get redshift_idc_application resource handler
    pub fn redshift_idc_application(&self) -> resources::Redshift_idc_application<'_> {
        resources::Redshift_idc_application::new(self.provider)
    }
    /// Get snapshot_copy_grant resource handler
    pub fn snapshot_copy_grant(&self) -> resources::Snapshot_copy_grant<'_> {
        resources::Snapshot_copy_grant::new(self.provider)
    }
    /// Get inbound_integrations resource handler
    pub fn inbound_integrations(&self) -> resources::Inbound_integrations<'_> {
        resources::Inbound_integrations::new(self.provider)
    }
    /// Get event_subscriptions resource handler
    pub fn event_subscriptions(&self) -> resources::Event_subscriptions<'_> {
        resources::Event_subscriptions::new(self.provider)
    }
    /// Get table_restore_status resource handler
    pub fn table_restore_status(&self) -> resources::Table_restore_status<'_> {
        resources::Table_restore_status::new(self.provider)
    }
    /// Get node_configuration_options resource handler
    pub fn node_configuration_options(&self) -> resources::Node_configuration_options<'_> {
        resources::Node_configuration_options::new(self.provider)
    }
    /// Get partner resource handler
    pub fn partner(&self) -> resources::Partner<'_> {
        resources::Partner::new(self.provider)
    }
    /// Get storage resource handler
    pub fn storage(&self) -> resources::Storage<'_> {
        resources::Storage::new(self.provider)
    }
    /// Get reserved_node_offerings resource handler
    pub fn reserved_node_offerings(&self) -> resources::Reserved_node_offerings<'_> {
        resources::Reserved_node_offerings::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get cluster_parameter_groups resource handler
    pub fn cluster_parameter_groups(&self) -> resources::Cluster_parameter_groups<'_> {
        resources::Cluster_parameter_groups::new(self.provider)
    }
    /// Get custom_domain_associations resource handler
    pub fn custom_domain_associations(&self) -> resources::Custom_domain_associations<'_> {
        resources::Custom_domain_associations::new(self.provider)
    }
    /// Get custom_domain_association resource handler
    pub fn custom_domain_association(&self) -> resources::Custom_domain_association<'_> {
        resources::Custom_domain_association::new(self.provider)
    }
    /// Get cluster_parameter_group resource handler
    pub fn cluster_parameter_group(&self) -> resources::Cluster_parameter_group<'_> {
        resources::Cluster_parameter_group::new(self.provider)
    }
    /// Get reserved_node_exchange_configuration_options resource handler
    pub fn reserved_node_exchange_configuration_options(&self) -> resources::Reserved_node_exchange_configuration_options<'_> {
        resources::Reserved_node_exchange_configuration_options::new(self.provider)
    }
    /// Get endpoint_authorization resource handler
    pub fn endpoint_authorization(&self) -> resources::Endpoint_authorization<'_> {
        resources::Endpoint_authorization::new(self.provider)
    }
    /// Get cluster_credentials_with_iam resource handler
    pub fn cluster_credentials_with_iam(&self) -> resources::Cluster_credentials_with_iam<'_> {
        resources::Cluster_credentials_with_iam::new(self.provider)
    }
    /// Get resize resource handler
    pub fn resize(&self) -> resources::Resize<'_> {
        resources::Resize::new(self.provider)
    }
    /// Get endpoint_access resource handler
    pub fn endpoint_access(&self) -> resources::Endpoint_access<'_> {
        resources::Endpoint_access::new(self.provider)
    }
    /// Get integrations resource handler
    pub fn integrations(&self) -> resources::Integrations<'_> {
        resources::Integrations::new(self.provider)
    }
    /// Get partners resource handler
    pub fn partners(&self) -> resources::Partners<'_> {
        resources::Partners::new(self.provider)
    }
    /// Get hsm_client_certificate resource handler
    pub fn hsm_client_certificate(&self) -> resources::Hsm_client_certificate<'_> {
        resources::Hsm_client_certificate::new(self.provider)
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
