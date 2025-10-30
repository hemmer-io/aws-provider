//! Elastic_load_balancing_2012_06_01 Service
//!
//! Auto-generated service module for elastic_load_balancing_2012_06_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for elastic_load_balancing_2012_06_01
pub struct Elastic_load_balancing_2012_06_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Elastic_load_balancing_2012_06_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get account_limits resource handler
    pub fn account_limits(&self) -> resources::Account_limits<'_> {
        resources::Account_limits::new(self.provider)
    }
    /// Get load_balancer_attributes resource handler
    pub fn load_balancer_attributes(&self) -> resources::Load_balancer_attributes<'_> {
        resources::Load_balancer_attributes::new(self.provider)
    }
    /// Get load_balancer resource handler
    pub fn load_balancer(&self) -> resources::Load_balancer<'_> {
        resources::Load_balancer::new(self.provider)
    }
    /// Get load_balancer_policies resource handler
    pub fn load_balancer_policies(&self) -> resources::Load_balancer_policies<'_> {
        resources::Load_balancer_policies::new(self.provider)
    }
    /// Get load_balancer_listeners resource handler
    pub fn load_balancer_listeners(&self) -> resources::Load_balancer_listeners<'_> {
        resources::Load_balancer_listeners::new(self.provider)
    }
    /// Get lb_cookie_stickiness_policy resource handler
    pub fn lb_cookie_stickiness_policy(&self) -> resources::Lb_cookie_stickiness_policy<'_> {
        resources::Lb_cookie_stickiness_policy::new(self.provider)
    }
    /// Get load_balancer_policy resource handler
    pub fn load_balancer_policy(&self) -> resources::Load_balancer_policy<'_> {
        resources::Load_balancer_policy::new(self.provider)
    }
    /// Get instance_health resource handler
    pub fn instance_health(&self) -> resources::Instance_health<'_> {
        resources::Instance_health::new(self.provider)
    }
    /// Get load_balancer_policy_types resource handler
    pub fn load_balancer_policy_types(&self) -> resources::Load_balancer_policy_types<'_> {
        resources::Load_balancer_policy_types::new(self.provider)
    }
    /// Get load_balancers resource handler
    pub fn load_balancers(&self) -> resources::Load_balancers<'_> {
        resources::Load_balancers::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get app_cookie_stickiness_policy resource handler
    pub fn app_cookie_stickiness_policy(&self) -> resources::App_cookie_stickiness_policy<'_> {
        resources::App_cookie_stickiness_policy::new(self.provider)
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
