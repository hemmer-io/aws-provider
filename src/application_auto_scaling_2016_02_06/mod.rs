//! Application_auto_scaling_2016_02_06 Service
//!
//! Auto-generated service module for application_auto_scaling_2016_02_06

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for application_auto_scaling_2016_02_06
pub struct Application_auto_scaling_2016_02_06Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Application_auto_scaling_2016_02_06Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get scaling_policy resource handler
    pub fn scaling_policy(&self) -> resources::Scaling_policy<'_> {
        resources::Scaling_policy::new(self.provider)
    }
    /// Get scheduled_actions resource handler
    pub fn scheduled_actions(&self) -> resources::Scheduled_actions<'_> {
        resources::Scheduled_actions::new(self.provider)
    }
    /// Get scheduled_action resource handler
    pub fn scheduled_action(&self) -> resources::Scheduled_action<'_> {
        resources::Scheduled_action::new(self.provider)
    }
    /// Get scalable_targets resource handler
    pub fn scalable_targets(&self) -> resources::Scalable_targets<'_> {
        resources::Scalable_targets::new(self.provider)
    }
    /// Get predictive_scaling_forecast resource handler
    pub fn predictive_scaling_forecast(&self) -> resources::Predictive_scaling_forecast<'_> {
        resources::Predictive_scaling_forecast::new(self.provider)
    }
    /// Get scaling_activities resource handler
    pub fn scaling_activities(&self) -> resources::Scaling_activities<'_> {
        resources::Scaling_activities::new(self.provider)
    }
    /// Get scaling_policies resource handler
    pub fn scaling_policies(&self) -> resources::Scaling_policies<'_> {
        resources::Scaling_policies::new(self.provider)
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
