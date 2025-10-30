//! Secrets_manager_2017_10_17 Service
//!
//! Auto-generated service module for secrets_manager_2017_10_17

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for secrets_manager_2017_10_17
pub struct Secrets_manager_2017_10_17Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Secrets_manager_2017_10_17Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get secret resource handler
    pub fn secret(&self) -> resources::Secret<'_> {
        resources::Secret::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get random_password resource handler
    pub fn random_password(&self) -> resources::Random_password<'_> {
        resources::Random_password::new(self.provider)
    }
    /// Get secret_version_stage resource handler
    pub fn secret_version_stage(&self) -> resources::Secret_version_stage<'_> {
        resources::Secret_version_stage::new(self.provider)
    }
    /// Get secret_value resource handler
    pub fn secret_value(&self) -> resources::Secret_value<'_> {
        resources::Secret_value::new(self.provider)
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
