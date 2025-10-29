//! License Service
//!
//! Auto-generated service module for license

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for license
pub struct LicenseService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> LicenseService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get registered_subscription_provider resource handler
    pub fn registered_subscription_provider(&self) -> resources::Registered_subscription_provider<'_> {
        resources::Registered_subscription_provider::new(self.provider)
    }
    /// Get service_settings resource handler
    pub fn service_settings(&self) -> resources::Service_settings<'_> {
        resources::Service_settings::new(self.provider)
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
