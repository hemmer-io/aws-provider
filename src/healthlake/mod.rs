//! Healthlake Service
//!
//! Auto-generated service module for healthlake

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for healthlake
pub struct HealthlakeService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> HealthlakeService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get fhirdatastore resource handler
    pub fn fhirdatastore(&self) -> resources::Fhirdatastore<'_> {
        resources::Fhirdatastore::new(self.provider)
    }
    /// Get fhirexport_job resource handler
    pub fn fhirexport_job(&self) -> resources::Fhirexport_job<'_> {
        resources::Fhirexport_job::new(self.provider)
    }
    /// Get fhirimport_job resource handler
    pub fn fhirimport_job(&self) -> resources::Fhirimport_job<'_> {
        resources::Fhirimport_job::new(self.provider)
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
