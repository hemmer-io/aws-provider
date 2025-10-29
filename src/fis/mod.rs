//! Fis Service
//!
//! Auto-generated service module for fis

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for fis
pub struct FisService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> FisService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get experiment_template resource handler
    pub fn experiment_template(&self) -> resources::Experiment_template<'_> {
        resources::Experiment_template::new(self.provider)
    }
    /// Get target_resource_type resource handler
    pub fn target_resource_type(&self) -> resources::Target_resource_type<'_> {
        resources::Target_resource_type::new(self.provider)
    }
    /// Get safety_lever resource handler
    pub fn safety_lever(&self) -> resources::Safety_lever<'_> {
        resources::Safety_lever::new(self.provider)
    }
    /// Get experiment resource handler
    pub fn experiment(&self) -> resources::Experiment<'_> {
        resources::Experiment::new(self.provider)
    }
    /// Get experiment_target_account_configuration resource handler
    pub fn experiment_target_account_configuration(&self) -> resources::Experiment_target_account_configuration<'_> {
        resources::Experiment_target_account_configuration::new(self.provider)
    }
    /// Get target_account_configuration resource handler
    pub fn target_account_configuration(&self) -> resources::Target_account_configuration<'_> {
        resources::Target_account_configuration::new(self.provider)
    }
    /// Get action resource handler
    pub fn action(&self) -> resources::Action<'_> {
        resources::Action::new(self.provider)
    }
    /// Get safety_lever_state resource handler
    pub fn safety_lever_state(&self) -> resources::Safety_lever_state<'_> {
        resources::Safety_lever_state::new(self.provider)
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
