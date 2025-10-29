//! Billingconductor Service
//!
//! Auto-generated service module for billingconductor

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for billingconductor
pub struct BillingconductorService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> BillingconductorService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get billing_group_cost_report resource handler
    pub fn billing_group_cost_report(&self) -> resources::Billing_group_cost_report<'_> {
        resources::Billing_group_cost_report::new(self.provider)
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
