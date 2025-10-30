//! Network_firewall_2020_11_12 Service
//!
//! Auto-generated service module for network_firewall_2020_11_12

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for network_firewall_2020_11_12
pub struct Network_firewall_2020_11_12Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Network_firewall_2020_11_12Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get tls_inspection_configuration resource handler
    pub fn tls_inspection_configuration(&self) -> resources::Tls_inspection_configuration<'_> {
        resources::Tls_inspection_configuration::new(self.provider)
    }
    /// Get logging_configuration resource handler
    pub fn logging_configuration(&self) -> resources::Logging_configuration<'_> {
        resources::Logging_configuration::new(self.provider)
    }
    /// Get firewall_encryption_configuration resource handler
    pub fn firewall_encryption_configuration(&self) -> resources::Firewall_encryption_configuration<'_> {
        resources::Firewall_encryption_configuration::new(self.provider)
    }
    /// Get firewall_policy_change_protection resource handler
    pub fn firewall_policy_change_protection(&self) -> resources::Firewall_policy_change_protection<'_> {
        resources::Firewall_policy_change_protection::new(self.provider)
    }
    /// Get network_firewall_transit_gateway_attachment resource handler
    pub fn network_firewall_transit_gateway_attachment(&self) -> resources::Network_firewall_transit_gateway_attachment<'_> {
        resources::Network_firewall_transit_gateway_attachment::new(self.provider)
    }
    /// Get flow_operation resource handler
    pub fn flow_operation(&self) -> resources::Flow_operation<'_> {
        resources::Flow_operation::new(self.provider)
    }
    /// Get firewall_policy resource handler
    pub fn firewall_policy(&self) -> resources::Firewall_policy<'_> {
        resources::Firewall_policy::new(self.provider)
    }
    /// Get analysis_report_results resource handler
    pub fn analysis_report_results(&self) -> resources::Analysis_report_results<'_> {
        resources::Analysis_report_results::new(self.provider)
    }
    /// Get firewall_metadata resource handler
    pub fn firewall_metadata(&self) -> resources::Firewall_metadata<'_> {
        resources::Firewall_metadata::new(self.provider)
    }
    /// Get availability_zone_change_protection resource handler
    pub fn availability_zone_change_protection(&self) -> resources::Availability_zone_change_protection<'_> {
        resources::Availability_zone_change_protection::new(self.provider)
    }
    /// Get firewall_analysis_settings resource handler
    pub fn firewall_analysis_settings(&self) -> resources::Firewall_analysis_settings<'_> {
        resources::Firewall_analysis_settings::new(self.provider)
    }
    /// Get firewall_description resource handler
    pub fn firewall_description(&self) -> resources::Firewall_description<'_> {
        resources::Firewall_description::new(self.provider)
    }
    /// Get rule_group_summary resource handler
    pub fn rule_group_summary(&self) -> resources::Rule_group_summary<'_> {
        resources::Rule_group_summary::new(self.provider)
    }
    /// Get firewall_delete_protection resource handler
    pub fn firewall_delete_protection(&self) -> resources::Firewall_delete_protection<'_> {
        resources::Firewall_delete_protection::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get rule_group resource handler
    pub fn rule_group(&self) -> resources::Rule_group<'_> {
        resources::Rule_group::new(self.provider)
    }
    /// Get rule_group_metadata resource handler
    pub fn rule_group_metadata(&self) -> resources::Rule_group_metadata<'_> {
        resources::Rule_group_metadata::new(self.provider)
    }
    /// Get firewall resource handler
    pub fn firewall(&self) -> resources::Firewall<'_> {
        resources::Firewall::new(self.provider)
    }
    /// Get vpc_endpoint_association resource handler
    pub fn vpc_endpoint_association(&self) -> resources::Vpc_endpoint_association<'_> {
        resources::Vpc_endpoint_association::new(self.provider)
    }
    /// Get subnet_change_protection resource handler
    pub fn subnet_change_protection(&self) -> resources::Subnet_change_protection<'_> {
        resources::Subnet_change_protection::new(self.provider)
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
