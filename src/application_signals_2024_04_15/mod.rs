//! Application_signals_2024_04_15 Service
//!
//! Auto-generated service module for application_signals_2024_04_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for application_signals_2024_04_15
pub struct Application_signals_2024_04_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Application_signals_2024_04_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get grouping_configuration resource handler
    pub fn grouping_configuration(&self) -> resources::Grouping_configuration<'_> {
        resources::Grouping_configuration::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
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
