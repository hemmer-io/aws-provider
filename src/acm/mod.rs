//! Acm Service
//!
//! Auto-generated service module for acm

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for acm
pub struct AcmService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AcmService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get certificate_options resource handler
    pub fn certificate_options(&self) -> resources::Certificate_options<'_> {
        resources::Certificate_options::new(self.provider)
    }
    /// Get certificate resource handler
    pub fn certificate(&self) -> resources::Certificate<'_> {
        resources::Certificate::new(self.provider)
    }
    /// Get account_configuration resource handler
    pub fn account_configuration(&self) -> resources::Account_configuration<'_> {
        resources::Account_configuration::new(self.provider)
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
