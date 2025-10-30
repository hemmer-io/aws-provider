//! License_manager_user_subscriptions_2018_05_10 Service
//!
//! Auto-generated service module for license_manager_user_subscriptions_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for license_manager_user_subscriptions_2018_05_10
pub struct License_manager_user_subscriptions_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> License_manager_user_subscriptions_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get license_server_endpoint resource handler
    pub fn license_server_endpoint(&self) -> resources::License_server_endpoint<'_> {
        resources::License_server_endpoint::new(self.provider)
    }
    /// Get identity_provider_settings resource handler
    pub fn identity_provider_settings(&self) -> resources::Identity_provider_settings<'_> {
        resources::Identity_provider_settings::new(self.provider)
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
