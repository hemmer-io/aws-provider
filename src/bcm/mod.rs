//! Bcm Service
//!
//! Auto-generated service module for bcm

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bcm
pub struct BcmService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> BcmService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get dashboard resource handler
    pub fn dashboard(&self) -> resources::Dashboard<'_> {
        resources::Dashboard::new(self.provider)
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
