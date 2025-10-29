//! Networkmanager Service
//!
//! Auto-generated service module for networkmanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for networkmanager
pub struct NetworkmanagerService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> NetworkmanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get links resource handler
    pub fn links(&self) -> resources::Links<'_> {
        resources::Links::new(self.provider)
    }
    /// Get core_network_policy_version resource handler
    pub fn core_network_policy_version(&self) -> resources::Core_network_policy_version<'_> {
        resources::Core_network_policy_version::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get direct_connect_gateway_attachment resource handler
    pub fn direct_connect_gateway_attachment(&self) -> resources::Direct_connect_gateway_attachment<'_> {
        resources::Direct_connect_gateway_attachment::new(self.provider)
    }
    /// Get transit_gateway_peering resource handler
    pub fn transit_gateway_peering(&self) -> resources::Transit_gateway_peering<'_> {
        resources::Transit_gateway_peering::new(self.provider)
    }
    /// Get network_resource_relationships resource handler
    pub fn network_resource_relationships(&self) -> resources::Network_resource_relationships<'_> {
        resources::Network_resource_relationships::new(self.provider)
    }
    /// Get network_telemetry resource handler
    pub fn network_telemetry(&self) -> resources::Network_telemetry<'_> {
        resources::Network_telemetry::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get link_associations resource handler
    pub fn link_associations(&self) -> resources::Link_associations<'_> {
        resources::Link_associations::new(self.provider)
    }
    /// Get site resource handler
    pub fn site(&self) -> resources::Site<'_> {
        resources::Site::new(self.provider)
    }
    /// Get global_network resource handler
    pub fn global_network(&self) -> resources::Global_network<'_> {
        resources::Global_network::new(self.provider)
    }
    /// Get connect_peer_associations resource handler
    pub fn connect_peer_associations(&self) -> resources::Connect_peer_associations<'_> {
        resources::Connect_peer_associations::new(self.provider)
    }
    /// Get core_network_change_events resource handler
    pub fn core_network_change_events(&self) -> resources::Core_network_change_events<'_> {
        resources::Core_network_change_events::new(self.provider)
    }
    /// Get network_resources resource handler
    pub fn network_resources(&self) -> resources::Network_resources<'_> {
        resources::Network_resources::new(self.provider)
    }
    /// Get transit_gateway_registrations resource handler
    pub fn transit_gateway_registrations(&self) -> resources::Transit_gateway_registrations<'_> {
        resources::Transit_gateway_registrations::new(self.provider)
    }
    /// Get network_routes resource handler
    pub fn network_routes(&self) -> resources::Network_routes<'_> {
        resources::Network_routes::new(self.provider)
    }
    /// Get transit_gateway_connect_peer_associations resource handler
    pub fn transit_gateway_connect_peer_associations(&self) -> resources::Transit_gateway_connect_peer_associations<'_> {
        resources::Transit_gateway_connect_peer_associations::new(self.provider)
    }
    /// Get core_network resource handler
    pub fn core_network(&self) -> resources::Core_network<'_> {
        resources::Core_network::new(self.provider)
    }
    /// Get link resource handler
    pub fn link(&self) -> resources::Link<'_> {
        resources::Link::new(self.provider)
    }
    /// Get network_resource_counts resource handler
    pub fn network_resource_counts(&self) -> resources::Network_resource_counts<'_> {
        resources::Network_resource_counts::new(self.provider)
    }
    /// Get connect_attachment resource handler
    pub fn connect_attachment(&self) -> resources::Connect_attachment<'_> {
        resources::Connect_attachment::new(self.provider)
    }
    /// Get connect_peer resource handler
    pub fn connect_peer(&self) -> resources::Connect_peer<'_> {
        resources::Connect_peer::new(self.provider)
    }
    /// Get global_networks resource handler
    pub fn global_networks(&self) -> resources::Global_networks<'_> {
        resources::Global_networks::new(self.provider)
    }
    /// Get transit_gateway_route_table_attachment resource handler
    pub fn transit_gateway_route_table_attachment(&self) -> resources::Transit_gateway_route_table_attachment<'_> {
        resources::Transit_gateway_route_table_attachment::new(self.provider)
    }
    /// Get peering resource handler
    pub fn peering(&self) -> resources::Peering<'_> {
        resources::Peering::new(self.provider)
    }
    /// Get network_resource_metadata resource handler
    pub fn network_resource_metadata(&self) -> resources::Network_resource_metadata<'_> {
        resources::Network_resource_metadata::new(self.provider)
    }
    /// Get customer_gateway_associations resource handler
    pub fn customer_gateway_associations(&self) -> resources::Customer_gateway_associations<'_> {
        resources::Customer_gateway_associations::new(self.provider)
    }
    /// Get core_network_policy resource handler
    pub fn core_network_policy(&self) -> resources::Core_network_policy<'_> {
        resources::Core_network_policy::new(self.provider)
    }
    /// Get route_analysis resource handler
    pub fn route_analysis(&self) -> resources::Route_analysis<'_> {
        resources::Route_analysis::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get connections resource handler
    pub fn connections(&self) -> resources::Connections<'_> {
        resources::Connections::new(self.provider)
    }
    /// Get core_network_change_set resource handler
    pub fn core_network_change_set(&self) -> resources::Core_network_change_set<'_> {
        resources::Core_network_change_set::new(self.provider)
    }
    /// Get site_to_site_vpn_attachment resource handler
    pub fn site_to_site_vpn_attachment(&self) -> resources::Site_to_site_vpn_attachment<'_> {
        resources::Site_to_site_vpn_attachment::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get devices resource handler
    pub fn devices(&self) -> resources::Devices<'_> {
        resources::Devices::new(self.provider)
    }
    /// Get vpc_attachment resource handler
    pub fn vpc_attachment(&self) -> resources::Vpc_attachment<'_> {
        resources::Vpc_attachment::new(self.provider)
    }
    /// Get sites resource handler
    pub fn sites(&self) -> resources::Sites<'_> {
        resources::Sites::new(self.provider)
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
