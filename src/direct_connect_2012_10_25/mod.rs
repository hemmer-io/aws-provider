//! Direct_connect_2012_10_25 Service
//!
//! Auto-generated service module for direct_connect_2012_10_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for direct_connect_2012_10_25
pub struct Direct_connect_2012_10_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Direct_connect_2012_10_25Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get lag resource handler
    pub fn lag(&self) -> resources::Lag<'_> {
        resources::Lag::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get bgp_peer resource handler
    pub fn bgp_peer(&self) -> resources::Bgp_peer<'_> {
        resources::Bgp_peer::new(self.provider)
    }
    /// Get public_virtual_interface resource handler
    pub fn public_virtual_interface(&self) -> resources::Public_virtual_interface<'_> {
        resources::Public_virtual_interface::new(self.provider)
    }
    /// Get direct_connect_gateway_association_proposals resource handler
    pub fn direct_connect_gateway_association_proposals(&self) -> resources::Direct_connect_gateway_association_proposals<'_> {
        resources::Direct_connect_gateway_association_proposals::new(self.provider)
    }
    /// Get hosted_connections resource handler
    pub fn hosted_connections(&self) -> resources::Hosted_connections<'_> {
        resources::Hosted_connections::new(self.provider)
    }
    /// Get loa resource handler
    pub fn loa(&self) -> resources::Loa<'_> {
        resources::Loa::new(self.provider)
    }
    /// Get virtual_interface resource handler
    pub fn virtual_interface(&self) -> resources::Virtual_interface<'_> {
        resources::Virtual_interface::new(self.provider)
    }
    /// Get interconnect resource handler
    pub fn interconnect(&self) -> resources::Interconnect<'_> {
        resources::Interconnect::new(self.provider)
    }
    /// Get direct_connect_gateways resource handler
    pub fn direct_connect_gateways(&self) -> resources::Direct_connect_gateways<'_> {
        resources::Direct_connect_gateways::new(self.provider)
    }
    /// Get direct_connect_gateway_attachments resource handler
    pub fn direct_connect_gateway_attachments(&self) -> resources::Direct_connect_gateway_attachments<'_> {
        resources::Direct_connect_gateway_attachments::new(self.provider)
    }
    /// Get direct_connect_gateway_associations resource handler
    pub fn direct_connect_gateway_associations(&self) -> resources::Direct_connect_gateway_associations<'_> {
        resources::Direct_connect_gateway_associations::new(self.provider)
    }
    /// Get connections resource handler
    pub fn connections(&self) -> resources::Connections<'_> {
        resources::Connections::new(self.provider)
    }
    /// Get virtual_interfaces resource handler
    pub fn virtual_interfaces(&self) -> resources::Virtual_interfaces<'_> {
        resources::Virtual_interfaces::new(self.provider)
    }
    /// Get virtual_interface_attributes resource handler
    pub fn virtual_interface_attributes(&self) -> resources::Virtual_interface_attributes<'_> {
        resources::Virtual_interface_attributes::new(self.provider)
    }
    /// Get interconnect_loa resource handler
    pub fn interconnect_loa(&self) -> resources::Interconnect_loa<'_> {
        resources::Interconnect_loa::new(self.provider)
    }
    /// Get locations resource handler
    pub fn locations(&self) -> resources::Locations<'_> {
        resources::Locations::new(self.provider)
    }
    /// Get interconnects resource handler
    pub fn interconnects(&self) -> resources::Interconnects<'_> {
        resources::Interconnects::new(self.provider)
    }
    /// Get virtual_gateways resource handler
    pub fn virtual_gateways(&self) -> resources::Virtual_gateways<'_> {
        resources::Virtual_gateways::new(self.provider)
    }
    /// Get lags resource handler
    pub fn lags(&self) -> resources::Lags<'_> {
        resources::Lags::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get transit_virtual_interface resource handler
    pub fn transit_virtual_interface(&self) -> resources::Transit_virtual_interface<'_> {
        resources::Transit_virtual_interface::new(self.provider)
    }
    /// Get customer_metadata resource handler
    pub fn customer_metadata(&self) -> resources::Customer_metadata<'_> {
        resources::Customer_metadata::new(self.provider)
    }
    /// Get private_virtual_interface resource handler
    pub fn private_virtual_interface(&self) -> resources::Private_virtual_interface<'_> {
        resources::Private_virtual_interface::new(self.provider)
    }
    /// Get router_configuration resource handler
    pub fn router_configuration(&self) -> resources::Router_configuration<'_> {
        resources::Router_configuration::new(self.provider)
    }
    /// Get direct_connect_gateway_association resource handler
    pub fn direct_connect_gateway_association(&self) -> resources::Direct_connect_gateway_association<'_> {
        resources::Direct_connect_gateway_association::new(self.provider)
    }
    /// Get connections_on_interconnect resource handler
    pub fn connections_on_interconnect(&self) -> resources::Connections_on_interconnect<'_> {
        resources::Connections_on_interconnect::new(self.provider)
    }
    /// Get direct_connect_gateway resource handler
    pub fn direct_connect_gateway(&self) -> resources::Direct_connect_gateway<'_> {
        resources::Direct_connect_gateway::new(self.provider)
    }
    /// Get direct_connect_gateway_association_proposal resource handler
    pub fn direct_connect_gateway_association_proposal(&self) -> resources::Direct_connect_gateway_association_proposal<'_> {
        resources::Direct_connect_gateway_association_proposal::new(self.provider)
    }
    /// Get connection_loa resource handler
    pub fn connection_loa(&self) -> resources::Connection_loa<'_> {
        resources::Connection_loa::new(self.provider)
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
