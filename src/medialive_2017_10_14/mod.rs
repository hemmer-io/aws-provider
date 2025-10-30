//! Medialive_2017_10_14 Service
//!
//! Auto-generated service module for medialive_2017_10_14

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for medialive_2017_10_14
pub struct Medialive_2017_10_14Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Medialive_2017_10_14Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get channel_class resource handler
    pub fn channel_class(&self) -> resources::Channel_class<'_> {
        resources::Channel_class::new(self.provider)
    }
    /// Get partner_input resource handler
    pub fn partner_input(&self) -> resources::Partner_input<'_> {
        resources::Partner_input::new(self.provider)
    }
    /// Get channel_placement_group resource handler
    pub fn channel_placement_group(&self) -> resources::Channel_placement_group<'_> {
        resources::Channel_placement_group::new(self.provider)
    }
    /// Get multiplex resource handler
    pub fn multiplex(&self) -> resources::Multiplex<'_> {
        resources::Multiplex::new(self.provider)
    }
    /// Get schedule resource handler
    pub fn schedule(&self) -> resources::Schedule<'_> {
        resources::Schedule::new(self.provider)
    }
    /// Get multiplex_program resource handler
    pub fn multiplex_program(&self) -> resources::Multiplex_program<'_> {
        resources::Multiplex_program::new(self.provider)
    }
    /// Get network resource handler
    pub fn network(&self) -> resources::Network<'_> {
        resources::Network::new(self.provider)
    }
    /// Get offering resource handler
    pub fn offering(&self) -> resources::Offering<'_> {
        resources::Offering::new(self.provider)
    }
    /// Get node resource handler
    pub fn node(&self) -> resources::Node<'_> {
        resources::Node::new(self.provider)
    }
    /// Get thumbnails resource handler
    pub fn thumbnails(&self) -> resources::Thumbnails<'_> {
        resources::Thumbnails::new(self.provider)
    }
    /// Get input_security_group resource handler
    pub fn input_security_group(&self) -> resources::Input_security_group<'_> {
        resources::Input_security_group::new(self.provider)
    }
    /// Get reservation resource handler
    pub fn reservation(&self) -> resources::Reservation<'_> {
        resources::Reservation::new(self.provider)
    }
    /// Get event_bridge_rule_template resource handler
    pub fn event_bridge_rule_template(&self) -> resources::Event_bridge_rule_template<'_> {
        resources::Event_bridge_rule_template::new(self.provider)
    }
    /// Get input_device resource handler
    pub fn input_device(&self) -> resources::Input_device<'_> {
        resources::Input_device::new(self.provider)
    }
    /// Get node_registration_script resource handler
    pub fn node_registration_script(&self) -> resources::Node_registration_script<'_> {
        resources::Node_registration_script::new(self.provider)
    }
    /// Get cloud_watch_alarm_template resource handler
    pub fn cloud_watch_alarm_template(&self) -> resources::Cloud_watch_alarm_template<'_> {
        resources::Cloud_watch_alarm_template::new(self.provider)
    }
    /// Get input resource handler
    pub fn input(&self) -> resources::Input<'_> {
        resources::Input::new(self.provider)
    }
    /// Get node_state resource handler
    pub fn node_state(&self) -> resources::Node_state<'_> {
        resources::Node_state::new(self.provider)
    }
    /// Get cloud_watch_alarm_template_group resource handler
    pub fn cloud_watch_alarm_template_group(&self) -> resources::Cloud_watch_alarm_template_group<'_> {
        resources::Cloud_watch_alarm_template_group::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get account_configuration resource handler
    pub fn account_configuration(&self) -> resources::Account_configuration<'_> {
        resources::Account_configuration::new(self.provider)
    }
    /// Get sdi_source resource handler
    pub fn sdi_source(&self) -> resources::Sdi_source<'_> {
        resources::Sdi_source::new(self.provider)
    }
    /// Get event_bridge_rule_template_group resource handler
    pub fn event_bridge_rule_template_group(&self) -> resources::Event_bridge_rule_template_group<'_> {
        resources::Event_bridge_rule_template_group::new(self.provider)
    }
    /// Get signal_map resource handler
    pub fn signal_map(&self) -> resources::Signal_map<'_> {
        resources::Signal_map::new(self.provider)
    }
    /// Get input_device_thumbnail resource handler
    pub fn input_device_thumbnail(&self) -> resources::Input_device_thumbnail<'_> {
        resources::Input_device_thumbnail::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
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
