//! Observabilityadmin_2018_05_10 Service
//!
//! Auto-generated service module for observabilityadmin_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for observabilityadmin_2018_05_10
pub struct Observabilityadmin_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Observabilityadmin_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get telemetry_rule_for_organization resource handler
    pub fn telemetry_rule_for_organization(&self) -> resources::Telemetry_rule_for_organization<'_> {
        resources::Telemetry_rule_for_organization::new(self.provider)
    }
    /// Get telemetry_evaluation_status_for_organization resource handler
    pub fn telemetry_evaluation_status_for_organization(&self) -> resources::Telemetry_evaluation_status_for_organization<'_> {
        resources::Telemetry_evaluation_status_for_organization::new(self.provider)
    }
    /// Get telemetry_rule resource handler
    pub fn telemetry_rule(&self) -> resources::Telemetry_rule<'_> {
        resources::Telemetry_rule::new(self.provider)
    }
    /// Get telemetry_enrichment_status resource handler
    pub fn telemetry_enrichment_status(&self) -> resources::Telemetry_enrichment_status<'_> {
        resources::Telemetry_enrichment_status::new(self.provider)
    }
    /// Get centralization_rule_for_organization resource handler
    pub fn centralization_rule_for_organization(&self) -> resources::Centralization_rule_for_organization<'_> {
        resources::Centralization_rule_for_organization::new(self.provider)
    }
    /// Get telemetry_evaluation_status resource handler
    pub fn telemetry_evaluation_status(&self) -> resources::Telemetry_evaluation_status<'_> {
        resources::Telemetry_evaluation_status::new(self.provider)
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
