//! Supplychain_2024_01_01 Service
//!
//! Auto-generated service module for supplychain_2024_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for supplychain_2024_01_01
pub struct Supplychain_2024_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Supplychain_2024_01_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get data_integration_event resource handler
    pub fn data_integration_event(&self) -> resources::Data_integration_event<'_> {
        resources::Data_integration_event::new(self.provider)
    }
    /// Get data_integration_flow_execution resource handler
    pub fn data_integration_flow_execution(&self) -> resources::Data_integration_flow_execution<'_> {
        resources::Data_integration_flow_execution::new(self.provider)
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
