//! Odb_2024_08_20 Service
//!
//! Auto-generated service module for odb_2024_08_20

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for odb_2024_08_20
pub struct Odb_2024_08_20Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Odb_2024_08_20Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get oci_onboarding_status resource handler
    pub fn oci_onboarding_status(&self) -> resources::Oci_onboarding_status<'_> {
        resources::Oci_onboarding_status::new(self.provider)
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
