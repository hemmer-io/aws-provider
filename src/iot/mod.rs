//! Iot Service
//!
//! Auto-generated service module for iot

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iot
pub struct IotService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> IotService<'a> {
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
