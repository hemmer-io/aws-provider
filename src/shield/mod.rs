//! Shield Service
//!
//! Auto-generated service module for shield

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for shield
pub struct ShieldService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ShieldService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get protection resource handler
    pub fn protection(&self) -> resources::Protection<'_> {
        resources::Protection::new(self.provider)
    }
    /// Get emergency_contact_settings resource handler
    pub fn emergency_contact_settings(&self) -> resources::Emergency_contact_settings<'_> {
        resources::Emergency_contact_settings::new(self.provider)
    }
    /// Get attack resource handler
    pub fn attack(&self) -> resources::Attack<'_> {
        resources::Attack::new(self.provider)
    }
    /// Get drtaccess resource handler
    pub fn drtaccess(&self) -> resources::Drtaccess<'_> {
        resources::Drtaccess::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get attack_statistics resource handler
    pub fn attack_statistics(&self) -> resources::Attack_statistics<'_> {
        resources::Attack_statistics::new(self.provider)
    }
    /// Get protection_group resource handler
    pub fn protection_group(&self) -> resources::Protection_group<'_> {
        resources::Protection_group::new(self.provider)
    }
    /// Get subscription_state resource handler
    pub fn subscription_state(&self) -> resources::Subscription_state<'_> {
        resources::Subscription_state::new(self.provider)
    }
    /// Get application_layer_automatic_response resource handler
    pub fn application_layer_automatic_response(&self) -> resources::Application_layer_automatic_response<'_> {
        resources::Application_layer_automatic_response::new(self.provider)
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
