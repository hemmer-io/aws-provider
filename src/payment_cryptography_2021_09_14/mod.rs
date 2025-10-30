//! Payment_cryptography_2021_09_14 Service
//!
//! Auto-generated service module for payment_cryptography_2021_09_14

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for payment_cryptography_2021_09_14
pub struct Payment_cryptography_2021_09_14Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Payment_cryptography_2021_09_14Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get public_key_certificate resource handler
    pub fn public_key_certificate(&self) -> resources::Public_key_certificate<'_> {
        resources::Public_key_certificate::new(self.provider)
    }
    /// Get certificate_signing_request resource handler
    pub fn certificate_signing_request(&self) -> resources::Certificate_signing_request<'_> {
        resources::Certificate_signing_request::new(self.provider)
    }
    /// Get default_key_replication_regions resource handler
    pub fn default_key_replication_regions(&self) -> resources::Default_key_replication_regions<'_> {
        resources::Default_key_replication_regions::new(self.provider)
    }
    /// Get parameters_for_import resource handler
    pub fn parameters_for_import(&self) -> resources::Parameters_for_import<'_> {
        resources::Parameters_for_import::new(self.provider)
    }
    /// Get parameters_for_export resource handler
    pub fn parameters_for_export(&self) -> resources::Parameters_for_export<'_> {
        resources::Parameters_for_export::new(self.provider)
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
