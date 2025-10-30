//! Elasticsearch_service_2015_01_01 Service
//!
//! Auto-generated service module for elasticsearch_service_2015_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for elasticsearch_service_2015_01_01
pub struct Elasticsearch_service_2015_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Elasticsearch_service_2015_01_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get inbound_cross_cluster_search_connection resource handler
    pub fn inbound_cross_cluster_search_connection(&self) -> resources::Inbound_cross_cluster_search_connection<'_> {
        resources::Inbound_cross_cluster_search_connection::new(self.provider)
    }
    /// Get outbound_cross_cluster_search_connections resource handler
    pub fn outbound_cross_cluster_search_connections(&self) -> resources::Outbound_cross_cluster_search_connections<'_> {
        resources::Outbound_cross_cluster_search_connections::new(self.provider)
    }
    /// Get domain_auto_tunes resource handler
    pub fn domain_auto_tunes(&self) -> resources::Domain_auto_tunes<'_> {
        resources::Domain_auto_tunes::new(self.provider)
    }
    /// Get upgrade_history resource handler
    pub fn upgrade_history(&self) -> resources::Upgrade_history<'_> {
        resources::Upgrade_history::new(self.provider)
    }
    /// Get compatible_elasticsearch_versions resource handler
    pub fn compatible_elasticsearch_versions(&self) -> resources::Compatible_elasticsearch_versions<'_> {
        resources::Compatible_elasticsearch_versions::new(self.provider)
    }
    /// Get inbound_cross_cluster_search_connections resource handler
    pub fn inbound_cross_cluster_search_connections(&self) -> resources::Inbound_cross_cluster_search_connections<'_> {
        resources::Inbound_cross_cluster_search_connections::new(self.provider)
    }
    /// Get elasticsearch_service_role resource handler
    pub fn elasticsearch_service_role(&self) -> resources::Elasticsearch_service_role<'_> {
        resources::Elasticsearch_service_role::new(self.provider)
    }
    /// Get elasticsearch_domain resource handler
    pub fn elasticsearch_domain(&self) -> resources::Elasticsearch_domain<'_> {
        resources::Elasticsearch_domain::new(self.provider)
    }
    /// Get domain_change_progress resource handler
    pub fn domain_change_progress(&self) -> resources::Domain_change_progress<'_> {
        resources::Domain_change_progress::new(self.provider)
    }
    /// Get reserved_elasticsearch_instances resource handler
    pub fn reserved_elasticsearch_instances(&self) -> resources::Reserved_elasticsearch_instances<'_> {
        resources::Reserved_elasticsearch_instances::new(self.provider)
    }
    /// Get package resource handler
    pub fn package(&self) -> resources::Package<'_> {
        resources::Package::new(self.provider)
    }
    /// Get elasticsearch_domain_config resource handler
    pub fn elasticsearch_domain_config(&self) -> resources::Elasticsearch_domain_config<'_> {
        resources::Elasticsearch_domain_config::new(self.provider)
    }
    /// Get vpc_endpoint resource handler
    pub fn vpc_endpoint(&self) -> resources::Vpc_endpoint<'_> {
        resources::Vpc_endpoint::new(self.provider)
    }
    /// Get outbound_cross_cluster_search_connection resource handler
    pub fn outbound_cross_cluster_search_connection(&self) -> resources::Outbound_cross_cluster_search_connection<'_> {
        resources::Outbound_cross_cluster_search_connection::new(self.provider)
    }
    /// Get vpc_endpoints resource handler
    pub fn vpc_endpoints(&self) -> resources::Vpc_endpoints<'_> {
        resources::Vpc_endpoints::new(self.provider)
    }
    /// Get upgrade_status resource handler
    pub fn upgrade_status(&self) -> resources::Upgrade_status<'_> {
        resources::Upgrade_status::new(self.provider)
    }
    /// Get packages resource handler
    pub fn packages(&self) -> resources::Packages<'_> {
        resources::Packages::new(self.provider)
    }
    /// Get elasticsearch_instance_type_limits resource handler
    pub fn elasticsearch_instance_type_limits(&self) -> resources::Elasticsearch_instance_type_limits<'_> {
        resources::Elasticsearch_instance_type_limits::new(self.provider)
    }
    /// Get package_version_history resource handler
    pub fn package_version_history(&self) -> resources::Package_version_history<'_> {
        resources::Package_version_history::new(self.provider)
    }
    /// Get elasticsearch_domains resource handler
    pub fn elasticsearch_domains(&self) -> resources::Elasticsearch_domains<'_> {
        resources::Elasticsearch_domains::new(self.provider)
    }
    /// Get reserved_elasticsearch_instance_offerings resource handler
    pub fn reserved_elasticsearch_instance_offerings(&self) -> resources::Reserved_elasticsearch_instance_offerings<'_> {
        resources::Reserved_elasticsearch_instance_offerings::new(self.provider)
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
