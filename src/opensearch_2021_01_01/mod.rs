//! Opensearch_2021_01_01 Service
//!
//! Auto-generated service module for opensearch_2021_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for opensearch_2021_01_01
pub struct Opensearch_2021_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Opensearch_2021_01_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get inbound_connection resource handler
    pub fn inbound_connection(&self) -> resources::Inbound_connection<'_> {
        resources::Inbound_connection::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get upgrade_status resource handler
    pub fn upgrade_status(&self) -> resources::Upgrade_status<'_> {
        resources::Upgrade_status::new(self.provider)
    }
    /// Get upgrade_history resource handler
    pub fn upgrade_history(&self) -> resources::Upgrade_history<'_> {
        resources::Upgrade_history::new(self.provider)
    }
    /// Get inbound_connections resource handler
    pub fn inbound_connections(&self) -> resources::Inbound_connections<'_> {
        resources::Inbound_connections::new(self.provider)
    }
    /// Get domain_change_progress resource handler
    pub fn domain_change_progress(&self) -> resources::Domain_change_progress<'_> {
        resources::Domain_change_progress::new(self.provider)
    }
    /// Get domain_config resource handler
    pub fn domain_config(&self) -> resources::Domain_config<'_> {
        resources::Domain_config::new(self.provider)
    }
    /// Get reserved_instances resource handler
    pub fn reserved_instances(&self) -> resources::Reserved_instances<'_> {
        resources::Reserved_instances::new(self.provider)
    }
    /// Get outbound_connections resource handler
    pub fn outbound_connections(&self) -> resources::Outbound_connections<'_> {
        resources::Outbound_connections::new(self.provider)
    }
    /// Get domain_nodes resource handler
    pub fn domain_nodes(&self) -> resources::Domain_nodes<'_> {
        resources::Domain_nodes::new(self.provider)
    }
    /// Get dry_run_progress resource handler
    pub fn dry_run_progress(&self) -> resources::Dry_run_progress<'_> {
        resources::Dry_run_progress::new(self.provider)
    }
    /// Get reserved_instance_offerings resource handler
    pub fn reserved_instance_offerings(&self) -> resources::Reserved_instance_offerings<'_> {
        resources::Reserved_instance_offerings::new(self.provider)
    }
    /// Get direct_query_data_source resource handler
    pub fn direct_query_data_source(&self) -> resources::Direct_query_data_source<'_> {
        resources::Direct_query_data_source::new(self.provider)
    }
    /// Get outbound_connection resource handler
    pub fn outbound_connection(&self) -> resources::Outbound_connection<'_> {
        resources::Outbound_connection::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get package resource handler
    pub fn package(&self) -> resources::Package<'_> {
        resources::Package::new(self.provider)
    }
    /// Get data_source resource handler
    pub fn data_source(&self) -> resources::Data_source<'_> {
        resources::Data_source::new(self.provider)
    }
    /// Get domain_auto_tunes resource handler
    pub fn domain_auto_tunes(&self) -> resources::Domain_auto_tunes<'_> {
        resources::Domain_auto_tunes::new(self.provider)
    }
    /// Get instance_type_limits resource handler
    pub fn instance_type_limits(&self) -> resources::Instance_type_limits<'_> {
        resources::Instance_type_limits::new(self.provider)
    }
    /// Get packages resource handler
    pub fn packages(&self) -> resources::Packages<'_> {
        resources::Packages::new(self.provider)
    }
    /// Get domains resource handler
    pub fn domains(&self) -> resources::Domains<'_> {
        resources::Domains::new(self.provider)
    }
    /// Get vpc_endpoints resource handler
    pub fn vpc_endpoints(&self) -> resources::Vpc_endpoints<'_> {
        resources::Vpc_endpoints::new(self.provider)
    }
    /// Get domain_maintenance_status resource handler
    pub fn domain_maintenance_status(&self) -> resources::Domain_maintenance_status<'_> {
        resources::Domain_maintenance_status::new(self.provider)
    }
    /// Get scheduled_action resource handler
    pub fn scheduled_action(&self) -> resources::Scheduled_action<'_> {
        resources::Scheduled_action::new(self.provider)
    }
    /// Get package_version_history resource handler
    pub fn package_version_history(&self) -> resources::Package_version_history<'_> {
        resources::Package_version_history::new(self.provider)
    }
    /// Get domain_health resource handler
    pub fn domain_health(&self) -> resources::Domain_health<'_> {
        resources::Domain_health::new(self.provider)
    }
    /// Get compatible_versions resource handler
    pub fn compatible_versions(&self) -> resources::Compatible_versions<'_> {
        resources::Compatible_versions::new(self.provider)
    }
    /// Get vpc_endpoint resource handler
    pub fn vpc_endpoint(&self) -> resources::Vpc_endpoint<'_> {
        resources::Vpc_endpoint::new(self.provider)
    }
    /// Get package_scope resource handler
    pub fn package_scope(&self) -> resources::Package_scope<'_> {
        resources::Package_scope::new(self.provider)
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
