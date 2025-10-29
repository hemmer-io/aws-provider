//! Vpc Service
//!
//! Auto-generated service module for vpc

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for vpc
pub struct VpcService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> VpcService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get auth_policy resource handler
    pub fn auth_policy(&self) -> resources::Auth_policy<'_> {
        resources::Auth_policy::new(self.provider)
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
