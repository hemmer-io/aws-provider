//! Migration_hub_2017_05_31 Service
//!
//! Auto-generated service module for migration_hub_2017_05_31

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for migration_hub_2017_05_31
pub struct Migration_hub_2017_05_31Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Migration_hub_2017_05_31Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get migration_task resource handler
    pub fn migration_task(&self) -> resources::Migration_task<'_> {
        resources::Migration_task::new(self.provider)
    }
    /// Get resource_attributes resource handler
    pub fn resource_attributes(&self) -> resources::Resource_attributes<'_> {
        resources::Resource_attributes::new(self.provider)
    }
    /// Get application_state resource handler
    pub fn application_state(&self) -> resources::Application_state<'_> {
        resources::Application_state::new(self.provider)
    }
    /// Get progress_update_stream resource handler
    pub fn progress_update_stream(&self) -> resources::Progress_update_stream<'_> {
        resources::Progress_update_stream::new(self.provider)
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
