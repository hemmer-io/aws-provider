//! License_manager_linux_subscriptions_2018_05_10 Service
//!
//! Auto-generated service module for license_manager_linux_subscriptions_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for license_manager_linux_subscriptions_2018_05_10
pub struct License_manager_linux_subscriptions_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> License_manager_linux_subscriptions_2018_05_10Service<'a> {
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
