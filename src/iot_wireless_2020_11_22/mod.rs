//! Iot_wireless_2020_11_22 Service
//!
//! Auto-generated service module for iot_wireless_2020_11_22

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iot_wireless_2020_11_22
pub struct Iot_wireless_2020_11_22Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iot_wireless_2020_11_22Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get fuota_task resource handler
    pub fn fuota_task(&self) -> resources::Fuota_task<'_> {
        resources::Fuota_task::new(self.provider)
    }
    /// Get partner_account resource handler
    pub fn partner_account(&self) -> resources::Partner_account<'_> {
        resources::Partner_account::new(self.provider)
    }
    /// Get multicast_group_session resource handler
    pub fn multicast_group_session(&self) -> resources::Multicast_group_session<'_> {
        resources::Multicast_group_session::new(self.provider)
    }
    /// Get queued_messages resource handler
    pub fn queued_messages(&self) -> resources::Queued_messages<'_> {
        resources::Queued_messages::new(self.provider)
    }
    /// Get multicast_group resource handler
    pub fn multicast_group(&self) -> resources::Multicast_group<'_> {
        resources::Multicast_group::new(self.provider)
    }
    /// Get metric_configuration resource handler
    pub fn metric_configuration(&self) -> resources::Metric_configuration<'_> {
        resources::Metric_configuration::new(self.provider)
    }
    /// Get metrics resource handler
    pub fn metrics(&self) -> resources::Metrics<'_> {
        resources::Metrics::new(self.provider)
    }
    /// Get wireless_device resource handler
    pub fn wireless_device(&self) -> resources::Wireless_device<'_> {
        resources::Wireless_device::new(self.provider)
    }
    /// Get network_analyzer_configuration resource handler
    pub fn network_analyzer_configuration(&self) -> resources::Network_analyzer_configuration<'_> {
        resources::Network_analyzer_configuration::new(self.provider)
    }
    /// Get destination resource handler
    pub fn destination(&self) -> resources::Destination<'_> {
        resources::Destination::new(self.provider)
    }
    /// Get resource_log_level resource handler
    pub fn resource_log_level(&self) -> resources::Resource_log_level<'_> {
        resources::Resource_log_level::new(self.provider)
    }
    /// Get resource_position resource handler
    pub fn resource_position(&self) -> resources::Resource_position<'_> {
        resources::Resource_position::new(self.provider)
    }
    /// Get position resource handler
    pub fn position(&self) -> resources::Position<'_> {
        resources::Position::new(self.provider)
    }
    /// Get wireless_gateway_statistics resource handler
    pub fn wireless_gateway_statistics(&self) -> resources::Wireless_gateway_statistics<'_> {
        resources::Wireless_gateway_statistics::new(self.provider)
    }
    /// Get service_profile resource handler
    pub fn service_profile(&self) -> resources::Service_profile<'_> {
        resources::Service_profile::new(self.provider)
    }
    /// Get wireless_gateway_certificate resource handler
    pub fn wireless_gateway_certificate(&self) -> resources::Wireless_gateway_certificate<'_> {
        resources::Wireless_gateway_certificate::new(self.provider)
    }
    /// Get log_levels_by_resource_types resource handler
    pub fn log_levels_by_resource_types(&self) -> resources::Log_levels_by_resource_types<'_> {
        resources::Log_levels_by_resource_types::new(self.provider)
    }
    /// Get position_estimate resource handler
    pub fn position_estimate(&self) -> resources::Position_estimate<'_> {
        resources::Position_estimate::new(self.provider)
    }
    /// Get service_endpoint resource handler
    pub fn service_endpoint(&self) -> resources::Service_endpoint<'_> {
        resources::Service_endpoint::new(self.provider)
    }
    /// Get wireless_gateway_task_definition resource handler
    pub fn wireless_gateway_task_definition(&self) -> resources::Wireless_gateway_task_definition<'_> {
        resources::Wireless_gateway_task_definition::new(self.provider)
    }
    /// Get wireless_gateway_task resource handler
    pub fn wireless_gateway_task(&self) -> resources::Wireless_gateway_task<'_> {
        resources::Wireless_gateway_task::new(self.provider)
    }
    /// Get position_configuration resource handler
    pub fn position_configuration(&self) -> resources::Position_configuration<'_> {
        resources::Position_configuration::new(self.provider)
    }
    /// Get device_profile resource handler
    pub fn device_profile(&self) -> resources::Device_profile<'_> {
        resources::Device_profile::new(self.provider)
    }
    /// Get wireless_device_statistics resource handler
    pub fn wireless_device_statistics(&self) -> resources::Wireless_device_statistics<'_> {
        resources::Wireless_device_statistics::new(self.provider)
    }
    /// Get wireless_gateway_firmware_information resource handler
    pub fn wireless_gateway_firmware_information(&self) -> resources::Wireless_gateway_firmware_information<'_> {
        resources::Wireless_gateway_firmware_information::new(self.provider)
    }
    /// Get event_configuration_by_resource_types resource handler
    pub fn event_configuration_by_resource_types(&self) -> resources::Event_configuration_by_resource_types<'_> {
        resources::Event_configuration_by_resource_types::new(self.provider)
    }
    /// Get wireless_device_import_task resource handler
    pub fn wireless_device_import_task(&self) -> resources::Wireless_device_import_task<'_> {
        resources::Wireless_device_import_task::new(self.provider)
    }
    /// Get wireless_gateway resource handler
    pub fn wireless_gateway(&self) -> resources::Wireless_gateway<'_> {
        resources::Wireless_gateway::new(self.provider)
    }
    /// Get resource_event_configuration resource handler
    pub fn resource_event_configuration(&self) -> resources::Resource_event_configuration<'_> {
        resources::Resource_event_configuration::new(self.provider)
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
