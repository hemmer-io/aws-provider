//! Appflow_2020_08_23 Service
//!
//! Auto-generated service module for appflow_2020_08_23

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appflow_2020_08_23
pub struct Appflow_2020_08_23Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Appflow_2020_08_23Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get connector_profile resource handler
    pub fn connector_profile(&self) -> resources::Connector_profile<'_> {
        resources::Connector_profile::new(self.provider)
    }
    /// Get connector resource handler
    pub fn connector(&self) -> resources::Connector<'_> {
        resources::Connector::new(self.provider)
    }
    /// Get connectors resource handler
    pub fn connectors(&self) -> resources::Connectors<'_> {
        resources::Connectors::new(self.provider)
    }
    /// Get flow_execution_records resource handler
    pub fn flow_execution_records(&self) -> resources::Flow_execution_records<'_> {
        resources::Flow_execution_records::new(self.provider)
    }
    /// Get connector_registration resource handler
    pub fn connector_registration(&self) -> resources::Connector_registration<'_> {
        resources::Connector_registration::new(self.provider)
    }
    /// Get connector_entity resource handler
    pub fn connector_entity(&self) -> resources::Connector_entity<'_> {
        resources::Connector_entity::new(self.provider)
    }
    /// Get flow resource handler
    pub fn flow(&self) -> resources::Flow<'_> {
        resources::Flow::new(self.provider)
    }
    /// Get connector_profiles resource handler
    pub fn connector_profiles(&self) -> resources::Connector_profiles<'_> {
        resources::Connector_profiles::new(self.provider)
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
