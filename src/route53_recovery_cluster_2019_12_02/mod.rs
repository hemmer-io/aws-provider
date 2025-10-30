//! Route53_recovery_cluster_2019_12_02 Service
//!
//! Auto-generated service module for route53_recovery_cluster_2019_12_02

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for route53_recovery_cluster_2019_12_02
pub struct Route53_recovery_cluster_2019_12_02Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Route53_recovery_cluster_2019_12_02Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get routing_control_state resource handler
    pub fn routing_control_state(&self) -> resources::Routing_control_state<'_> {
        resources::Routing_control_state::new(self.provider)
    }
    /// Get routing_control_states resource handler
    pub fn routing_control_states(&self) -> resources::Routing_control_states<'_> {
        resources::Routing_control_states::new(self.provider)
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
