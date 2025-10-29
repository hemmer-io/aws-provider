//! Groundstation Service
//!
//! Auto-generated service module for groundstation

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for groundstation
pub struct GroundstationService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> GroundstationService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get minute_usage resource handler
    pub fn minute_usage(&self) -> resources::Minute_usage<'_> {
        resources::Minute_usage::new(self.provider)
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
