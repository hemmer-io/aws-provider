//! Codeguruprofiler Service
//!
//! Auto-generated service module for codeguruprofiler

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codeguruprofiler
pub struct CodeguruprofilerService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CodeguruprofilerService<'a> {
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
