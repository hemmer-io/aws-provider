//! Savingsplans Service
//!
//! Auto-generated service module for savingsplans

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for savingsplans
pub struct SavingsplansService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SavingsplansService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get savings_plan_rates resource handler
    pub fn savings_plan_rates(&self) -> resources::Savings_plan_rates<'_> {
        resources::Savings_plan_rates::new(self.provider)
    }
    /// Get savings_plans resource handler
    pub fn savings_plans(&self) -> resources::Savings_plans<'_> {
        resources::Savings_plans::new(self.provider)
    }
    /// Get queued_savings_plan resource handler
    pub fn queued_savings_plan(&self) -> resources::Queued_savings_plan<'_> {
        resources::Queued_savings_plan::new(self.provider)
    }
    /// Get savings_plan resource handler
    pub fn savings_plan(&self) -> resources::Savings_plan<'_> {
        resources::Savings_plan::new(self.provider)
    }
    /// Get savings_plans_offering_rates resource handler
    pub fn savings_plans_offering_rates(&self) -> resources::Savings_plans_offering_rates<'_> {
        resources::Savings_plans_offering_rates::new(self.provider)
    }
    /// Get savings_plans_offerings resource handler
    pub fn savings_plans_offerings(&self) -> resources::Savings_plans_offerings<'_> {
        resources::Savings_plans_offerings::new(self.provider)
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
