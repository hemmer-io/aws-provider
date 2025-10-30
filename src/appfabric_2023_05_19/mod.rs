//! Appfabric_2023_05_19 Service
//!
//! Auto-generated service module for appfabric_2023_05_19

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appfabric_2023_05_19
pub struct Appfabric_2023_05_19Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Appfabric_2023_05_19Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get app_authorization resource handler
    pub fn app_authorization(&self) -> resources::App_authorization<'_> {
        resources::App_authorization::new(self.provider)
    }
    /// Get app_bundle resource handler
    pub fn app_bundle(&self) -> resources::App_bundle<'_> {
        resources::App_bundle::new(self.provider)
    }
    /// Get ingestion resource handler
    pub fn ingestion(&self) -> resources::Ingestion<'_> {
        resources::Ingestion::new(self.provider)
    }
    /// Get ingestion_destination resource handler
    pub fn ingestion_destination(&self) -> resources::Ingestion_destination<'_> {
        resources::Ingestion_destination::new(self.provider)
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
