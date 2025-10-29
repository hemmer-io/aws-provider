//! Pi Service
//!
//! Auto-generated service module for pi

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pi
pub struct PiService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> PiService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get performance_analysis_report resource handler
    pub fn performance_analysis_report(&self) -> resources::Performance_analysis_report<'_> {
        resources::Performance_analysis_report::new(self.provider)
    }
    /// Get resource_metadata resource handler
    pub fn resource_metadata(&self) -> resources::Resource_metadata<'_> {
        resources::Resource_metadata::new(self.provider)
    }
    /// Get resource_metrics resource handler
    pub fn resource_metrics(&self) -> resources::Resource_metrics<'_> {
        resources::Resource_metrics::new(self.provider)
    }
    /// Get dimension_key_details resource handler
    pub fn dimension_key_details(&self) -> resources::Dimension_key_details<'_> {
        resources::Dimension_key_details::new(self.provider)
    }
    /// Get dimension_keys resource handler
    pub fn dimension_keys(&self) -> resources::Dimension_keys<'_> {
        resources::Dimension_keys::new(self.provider)
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
