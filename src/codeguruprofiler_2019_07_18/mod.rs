//! Codeguruprofiler_2019_07_18 Service
//!
//! Auto-generated service module for codeguruprofiler_2019_07_18

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codeguruprofiler_2019_07_18
pub struct Codeguruprofiler_2019_07_18Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Codeguruprofiler_2019_07_18Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get findings_report_account_summary resource handler
    pub fn findings_report_account_summary(&self) -> resources::Findings_report_account_summary<'_> {
        resources::Findings_report_account_summary::new(self.provider)
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
