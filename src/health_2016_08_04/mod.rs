//! Health_2016_08_04 Service
//!
//! Auto-generated service module for health_2016_08_04

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for health_2016_08_04
pub struct Health_2016_08_04Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Health_2016_08_04Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get event_types resource handler
    pub fn event_types(&self) -> resources::Event_types<'_> {
        resources::Event_types::new(self.provider)
    }
    /// Get affected_entities_for_organization resource handler
    pub fn affected_entities_for_organization(&self) -> resources::Affected_entities_for_organization<'_> {
        resources::Affected_entities_for_organization::new(self.provider)
    }
    /// Get event_details_for_organization resource handler
    pub fn event_details_for_organization(&self) -> resources::Event_details_for_organization<'_> {
        resources::Event_details_for_organization::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get entity_aggregates_for_organization resource handler
    pub fn entity_aggregates_for_organization(&self) -> resources::Entity_aggregates_for_organization<'_> {
        resources::Entity_aggregates_for_organization::new(self.provider)
    }
    /// Get event_aggregates resource handler
    pub fn event_aggregates(&self) -> resources::Event_aggregates<'_> {
        resources::Event_aggregates::new(self.provider)
    }
    /// Get entity_aggregates resource handler
    pub fn entity_aggregates(&self) -> resources::Entity_aggregates<'_> {
        resources::Entity_aggregates::new(self.provider)
    }
    /// Get health_service_status_for_organization resource handler
    pub fn health_service_status_for_organization(&self) -> resources::Health_service_status_for_organization<'_> {
        resources::Health_service_status_for_organization::new(self.provider)
    }
    /// Get affected_entities resource handler
    pub fn affected_entities(&self) -> resources::Affected_entities<'_> {
        resources::Affected_entities::new(self.provider)
    }
    /// Get affected_accounts_for_organization resource handler
    pub fn affected_accounts_for_organization(&self) -> resources::Affected_accounts_for_organization<'_> {
        resources::Affected_accounts_for_organization::new(self.provider)
    }
    /// Get event_details resource handler
    pub fn event_details(&self) -> resources::Event_details<'_> {
        resources::Event_details::new(self.provider)
    }
    /// Get events_for_organization resource handler
    pub fn events_for_organization(&self) -> resources::Events_for_organization<'_> {
        resources::Events_for_organization::new(self.provider)
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
