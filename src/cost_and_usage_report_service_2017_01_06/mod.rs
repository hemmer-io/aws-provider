//! Cost_and_usage_report_service_2017_01_06 Service
//!
//! Auto-generated service module for cost_and_usage_report_service_2017_01_06

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cost_and_usage_report_service_2017_01_06
pub struct Cost_and_usage_report_service_2017_01_06Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cost_and_usage_report_service_2017_01_06Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get report_definition resource handler
    pub fn report_definition(&self) -> resources::Report_definition<'_> {
        resources::Report_definition::new(self.provider)
    }
    /// Get report_definitions resource handler
    pub fn report_definitions(&self) -> resources::Report_definitions<'_> {
        resources::Report_definitions::new(self.provider)
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
