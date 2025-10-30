//! Route53_recovery_readiness_2019_12_02 Service
//!
//! Auto-generated service module for route53_recovery_readiness_2019_12_02

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for route53_recovery_readiness_2019_12_02
pub struct Route53_recovery_readiness_2019_12_02Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Route53_recovery_readiness_2019_12_02Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get readiness_check_resource_status resource handler
    pub fn readiness_check_resource_status(&self) -> resources::Readiness_check_resource_status<'_> {
        resources::Readiness_check_resource_status::new(self.provider)
    }
    /// Get cell_readiness_summary resource handler
    pub fn cell_readiness_summary(&self) -> resources::Cell_readiness_summary<'_> {
        resources::Cell_readiness_summary::new(self.provider)
    }
    /// Get resource_set resource handler
    pub fn resource_set(&self) -> resources::Resource_set<'_> {
        resources::Resource_set::new(self.provider)
    }
    /// Get cross_account_authorization resource handler
    pub fn cross_account_authorization(&self) -> resources::Cross_account_authorization<'_> {
        resources::Cross_account_authorization::new(self.provider)
    }
    /// Get readiness_check resource handler
    pub fn readiness_check(&self) -> resources::Readiness_check<'_> {
        resources::Readiness_check::new(self.provider)
    }
    /// Get recovery_group resource handler
    pub fn recovery_group(&self) -> resources::Recovery_group<'_> {
        resources::Recovery_group::new(self.provider)
    }
    /// Get readiness_check_status resource handler
    pub fn readiness_check_status(&self) -> resources::Readiness_check_status<'_> {
        resources::Readiness_check_status::new(self.provider)
    }
    /// Get cell resource handler
    pub fn cell(&self) -> resources::Cell<'_> {
        resources::Cell::new(self.provider)
    }
    /// Get recovery_group_readiness_summary resource handler
    pub fn recovery_group_readiness_summary(&self) -> resources::Recovery_group_readiness_summary<'_> {
        resources::Recovery_group_readiness_summary::new(self.provider)
    }
    /// Get architecture_recommendations resource handler
    pub fn architecture_recommendations(&self) -> resources::Architecture_recommendations<'_> {
        resources::Architecture_recommendations::new(self.provider)
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
