//! Route53_recovery_control_config_2020_11_02 Service
//!
//! Auto-generated service module for route53_recovery_control_config_2020_11_02

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for route53_recovery_control_config_2020_11_02
pub struct Route53_recovery_control_config_2020_11_02Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Route53_recovery_control_config_2020_11_02Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get safety_rule resource handler
    pub fn safety_rule(&self) -> resources::Safety_rule<'_> {
        resources::Safety_rule::new(self.provider)
    }
    /// Get routing_control resource handler
    pub fn routing_control(&self) -> resources::Routing_control<'_> {
        resources::Routing_control::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get control_panel resource handler
    pub fn control_panel(&self) -> resources::Control_panel<'_> {
        resources::Control_panel::new(self.provider)
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
