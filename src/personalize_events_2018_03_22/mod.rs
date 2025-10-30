//! Personalize_events_2018_03_22 Service
//!
//! Auto-generated service module for personalize_events_2018_03_22

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for personalize_events_2018_03_22
pub struct Personalize_events_2018_03_22Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Personalize_events_2018_03_22Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get action_interactions resource handler
    pub fn action_interactions(&self) -> resources::Action_interactions<'_> {
        resources::Action_interactions::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get items resource handler
    pub fn items(&self) -> resources::Items<'_> {
        resources::Items::new(self.provider)
    }
    /// Get users resource handler
    pub fn users(&self) -> resources::Users<'_> {
        resources::Users::new(self.provider)
    }
    /// Get actions resource handler
    pub fn actions(&self) -> resources::Actions<'_> {
        resources::Actions::new(self.provider)
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
