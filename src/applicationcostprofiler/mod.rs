//! Applicationcostprofiler Service
//!
//! Auto-generated service module for applicationcostprofiler

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for applicationcostprofiler
pub struct ApplicationcostprofilerService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ApplicationcostprofilerService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get report_definition resource handler
    pub fn report_definition(&self) -> resources::Report_definition<'_> {
        resources::Report_definition::new(self.provider)
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
