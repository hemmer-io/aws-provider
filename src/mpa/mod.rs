//! Mpa Service
//!
//! Auto-generated service module for mpa

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mpa
pub struct MpaService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MpaService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get policy_version resource handler
    pub fn policy_version(&self) -> resources::Policy_version<'_> {
        resources::Policy_version::new(self.provider)
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
