//! Freetier Service
//!
//! Auto-generated service module for freetier

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for freetier
pub struct FreetierService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> FreetierService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get account_activity resource handler
    pub fn account_activity(&self) -> resources::Account_activity<'_> {
        resources::Account_activity::new(self.provider)
    }
    /// Get free_tier_usage resource handler
    pub fn free_tier_usage(&self) -> resources::Free_tier_usage<'_> {
        resources::Free_tier_usage::new(self.provider)
    }
    /// Get account_plan_state resource handler
    pub fn account_plan_state(&self) -> resources::Account_plan_state<'_> {
        resources::Account_plan_state::new(self.provider)
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
