//! Iot_managed_integrations_2025_03_03 Service
//!
//! Auto-generated service module for iot_managed_integrations_2025_03_03

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iot_managed_integrations_2025_03_03
pub struct Iot_managed_integrations_2025_03_03Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iot_managed_integrations_2025_03_03Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get custom_endpoint resource handler
    pub fn custom_endpoint(&self) -> resources::Custom_endpoint<'_> {
        resources::Custom_endpoint::new(self.provider)
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
