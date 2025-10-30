//! Groundstation_2019_05_23 Service
//!
//! Auto-generated service module for groundstation_2019_05_23

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for groundstation_2019_05_23
pub struct Groundstation_2019_05_23Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Groundstation_2019_05_23Service<'a> {
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
