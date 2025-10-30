//! Healthlake_2017_07_01 Service
//!
//! Auto-generated service module for healthlake_2017_07_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for healthlake_2017_07_01
pub struct Healthlake_2017_07_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Healthlake_2017_07_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get fhir_export_job resource handler
    pub fn fhir_export_job(&self) -> resources::Fhir_export_job<'_> {
        resources::Fhir_export_job::new(self.provider)
    }
    /// Get fhir_datastore resource handler
    pub fn fhir_datastore(&self) -> resources::Fhir_datastore<'_> {
        resources::Fhir_datastore::new(self.provider)
    }
    /// Get fhir_import_job resource handler
    pub fn fhir_import_job(&self) -> resources::Fhir_import_job<'_> {
        resources::Fhir_import_job::new(self.provider)
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
