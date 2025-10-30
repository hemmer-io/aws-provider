//! Iotdeviceadvisor_2020_09_18 Service
//!
//! Auto-generated service module for iotdeviceadvisor_2020_09_18

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iotdeviceadvisor_2020_09_18
pub struct Iotdeviceadvisor_2020_09_18Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iotdeviceadvisor_2020_09_18Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get suite_run_report resource handler
    pub fn suite_run_report(&self) -> resources::Suite_run_report<'_> {
        resources::Suite_run_report::new(self.provider)
    }
    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }
    /// Get suite_definition resource handler
    pub fn suite_definition(&self) -> resources::Suite_definition<'_> {
        resources::Suite_definition::new(self.provider)
    }
    /// Get suite_run resource handler
    pub fn suite_run(&self) -> resources::Suite_run<'_> {
        resources::Suite_run::new(self.provider)
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
