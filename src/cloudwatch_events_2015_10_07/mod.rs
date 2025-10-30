//! Cloudwatch_events_2015_10_07 Service
//!
//! Auto-generated service module for cloudwatch_events_2015_10_07

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudwatch_events_2015_10_07
pub struct Cloudwatch_events_2015_10_07Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudwatch_events_2015_10_07Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get partner_event_source resource handler
    pub fn partner_event_source(&self) -> resources::Partner_event_source<'_> {
        resources::Partner_event_source::new(self.provider)
    }
    /// Get event_source resource handler
    pub fn event_source(&self) -> resources::Event_source<'_> {
        resources::Event_source::new(self.provider)
    }
    /// Get partner_events resource handler
    pub fn partner_events(&self) -> resources::Partner_events<'_> {
        resources::Partner_events::new(self.provider)
    }
    /// Get targets resource handler
    pub fn targets(&self) -> resources::Targets<'_> {
        resources::Targets::new(self.provider)
    }
    /// Get archive resource handler
    pub fn archive(&self) -> resources::Archive<'_> {
        resources::Archive::new(self.provider)
    }
    /// Get api_destination resource handler
    pub fn api_destination(&self) -> resources::Api_destination<'_> {
        resources::Api_destination::new(self.provider)
    }
    /// Get event_bus resource handler
    pub fn event_bus(&self) -> resources::Event_bus<'_> {
        resources::Event_bus::new(self.provider)
    }
    /// Get rule resource handler
    pub fn rule(&self) -> resources::Rule<'_> {
        resources::Rule::new(self.provider)
    }
    /// Get replay resource handler
    pub fn replay(&self) -> resources::Replay<'_> {
        resources::Replay::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get permission resource handler
    pub fn permission(&self) -> resources::Permission<'_> {
        resources::Permission::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
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
