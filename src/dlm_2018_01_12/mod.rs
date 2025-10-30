//! Dlm_2018_01_12 Service
//!
//! Auto-generated service module for dlm_2018_01_12

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dlm_2018_01_12
pub struct Dlm_2018_01_12Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Dlm_2018_01_12Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get lifecycle_policy resource handler
    pub fn lifecycle_policy(&self) -> resources::Lifecycle_policy<'_> {
        resources::Lifecycle_policy::new(self.provider)
    }
    /// Get lifecycle_policies resource handler
    pub fn lifecycle_policies(&self) -> resources::Lifecycle_policies<'_> {
        resources::Lifecycle_policies::new(self.provider)
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
