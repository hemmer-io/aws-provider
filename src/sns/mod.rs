//! Sns Service
//!
//! Auto-generated service module for sns

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sns
pub struct SnsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SnsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get smssandbox_phone_number resource handler
    pub fn smssandbox_phone_number(&self) -> resources::Smssandbox_phone_number<'_> {
        resources::Smssandbox_phone_number::new(self.provider)
    }
    /// Get subscription_attributes resource handler
    pub fn subscription_attributes(&self) -> resources::Subscription_attributes<'_> {
        resources::Subscription_attributes::new(self.provider)
    }
    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }
    /// Get platform_application_attributes resource handler
    pub fn platform_application_attributes(&self) -> resources::Platform_application_attributes<'_> {
        resources::Platform_application_attributes::new(self.provider)
    }
    /// Get platform_application resource handler
    pub fn platform_application(&self) -> resources::Platform_application<'_> {
        resources::Platform_application::new(self.provider)
    }
    /// Get data_protection_policy resource handler
    pub fn data_protection_policy(&self) -> resources::Data_protection_policy<'_> {
        resources::Data_protection_policy::new(self.provider)
    }
    /// Get smsattributes resource handler
    pub fn smsattributes(&self) -> resources::Smsattributes<'_> {
        resources::Smsattributes::new(self.provider)
    }
    /// Get topic resource handler
    pub fn topic(&self) -> resources::Topic<'_> {
        resources::Topic::new(self.provider)
    }
    /// Get platform_endpoint resource handler
    pub fn platform_endpoint(&self) -> resources::Platform_endpoint<'_> {
        resources::Platform_endpoint::new(self.provider)
    }
    /// Get endpoint_attributes resource handler
    pub fn endpoint_attributes(&self) -> resources::Endpoint_attributes<'_> {
        resources::Endpoint_attributes::new(self.provider)
    }
    /// Get smssandbox_account_status resource handler
    pub fn smssandbox_account_status(&self) -> resources::Smssandbox_account_status<'_> {
        resources::Smssandbox_account_status::new(self.provider)
    }
    /// Get topic_attributes resource handler
    pub fn topic_attributes(&self) -> resources::Topic_attributes<'_> {
        resources::Topic_attributes::new(self.provider)
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
