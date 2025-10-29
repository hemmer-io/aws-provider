//! Kafkaconnect Service
//!
//! Auto-generated service module for kafkaconnect

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kafkaconnect
pub struct KafkaconnectService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> KafkaconnectService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get connector_operation resource handler
    pub fn connector_operation(&self) -> resources::Connector_operation<'_> {
        resources::Connector_operation::new(self.provider)
    }
    /// Get connector resource handler
    pub fn connector(&self) -> resources::Connector<'_> {
        resources::Connector::new(self.provider)
    }
    /// Get custom_plugin resource handler
    pub fn custom_plugin(&self) -> resources::Custom_plugin<'_> {
        resources::Custom_plugin::new(self.provider)
    }
    /// Get worker_configuration resource handler
    pub fn worker_configuration(&self) -> resources::Worker_configuration<'_> {
        resources::Worker_configuration::new(self.provider)
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
