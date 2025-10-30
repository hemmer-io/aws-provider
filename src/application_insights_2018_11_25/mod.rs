//! Application_insights_2018_11_25 Service
//!
//! Auto-generated service module for application_insights_2018_11_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for application_insights_2018_11_25
pub struct Application_insights_2018_11_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Application_insights_2018_11_25Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get component_configuration_recommendation resource handler
    pub fn component_configuration_recommendation(&self) -> resources::Component_configuration_recommendation<'_> {
        resources::Component_configuration_recommendation::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get workload resource handler
    pub fn workload(&self) -> resources::Workload<'_> {
        resources::Workload::new(self.provider)
    }
    /// Get observation resource handler
    pub fn observation(&self) -> resources::Observation<'_> {
        resources::Observation::new(self.provider)
    }
    /// Get log_pattern resource handler
    pub fn log_pattern(&self) -> resources::Log_pattern<'_> {
        resources::Log_pattern::new(self.provider)
    }
    /// Get component resource handler
    pub fn component(&self) -> resources::Component<'_> {
        resources::Component::new(self.provider)
    }
    /// Get component_configuration resource handler
    pub fn component_configuration(&self) -> resources::Component_configuration<'_> {
        resources::Component_configuration::new(self.provider)
    }
    /// Get problem_observations resource handler
    pub fn problem_observations(&self) -> resources::Problem_observations<'_> {
        resources::Problem_observations::new(self.provider)
    }
    /// Get problem resource handler
    pub fn problem(&self) -> resources::Problem<'_> {
        resources::Problem::new(self.provider)
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
