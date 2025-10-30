//! Bcm_pricing_calculator_2024_06_19 Service
//!
//! Auto-generated service module for bcm_pricing_calculator_2024_06_19

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bcm_pricing_calculator_2024_06_19
pub struct Bcm_pricing_calculator_2024_06_19Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Bcm_pricing_calculator_2024_06_19Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get preferences resource handler
    pub fn preferences(&self) -> resources::Preferences<'_> {
        resources::Preferences::new(self.provider)
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
