//! Billing Service
//!
//! Auto-generated service module for billing

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for billing
pub struct BillingService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> BillingService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get billing_view resource handler
    pub fn billing_view(&self) -> resources::Billing_view<'_> {
        resources::Billing_view::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
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
