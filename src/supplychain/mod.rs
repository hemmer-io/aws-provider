//! Supplychain Service
//!
//! Auto-generated service module for supplychain

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for supplychain
pub struct SupplychainService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SupplychainService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get data_integration_flow_execution resource handler
    pub fn data_integration_flow_execution(&self) -> resources::Data_integration_flow_execution<'_> {
        resources::Data_integration_flow_execution::new(self.provider)
    }
    /// Get data_integration_event resource handler
    pub fn data_integration_event(&self) -> resources::Data_integration_event<'_> {
        resources::Data_integration_event::new(self.provider)
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
