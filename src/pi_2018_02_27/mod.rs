//! Pi_2018_02_27 Service
//!
//! Auto-generated service module for pi_2018_02_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pi_2018_02_27
pub struct Pi_2018_02_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Pi_2018_02_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get performance_analysis_report resource handler
    pub fn performance_analysis_report(&self) -> resources::Performance_analysis_report<'_> {
        resources::Performance_analysis_report::new(self.provider)
    }
    /// Get dimension_keys resource handler
    pub fn dimension_keys(&self) -> resources::Dimension_keys<'_> {
        resources::Dimension_keys::new(self.provider)
    }
    /// Get dimension_key_details resource handler
    pub fn dimension_key_details(&self) -> resources::Dimension_key_details<'_> {
        resources::Dimension_key_details::new(self.provider)
    }
    /// Get resource_metrics resource handler
    pub fn resource_metrics(&self) -> resources::Resource_metrics<'_> {
        resources::Resource_metrics::new(self.provider)
    }
    /// Get resource_metadata resource handler
    pub fn resource_metadata(&self) -> resources::Resource_metadata<'_> {
        resources::Resource_metadata::new(self.provider)
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
