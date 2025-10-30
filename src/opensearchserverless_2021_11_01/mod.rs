//! Opensearchserverless_2021_11_01 Service
//!
//! Auto-generated service module for opensearchserverless_2021_11_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for opensearchserverless_2021_11_01
pub struct Opensearchserverless_2021_11_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Opensearchserverless_2021_11_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get policies_stats resource handler
    pub fn policies_stats(&self) -> resources::Policies_stats<'_> {
        resources::Policies_stats::new(self.provider)
    }
    /// Get security_policy resource handler
    pub fn security_policy(&self) -> resources::Security_policy<'_> {
        resources::Security_policy::new(self.provider)
    }
    /// Get vpc_endpoint resource handler
    pub fn vpc_endpoint(&self) -> resources::Vpc_endpoint<'_> {
        resources::Vpc_endpoint::new(self.provider)
    }
    /// Get lifecycle_policy resource handler
    pub fn lifecycle_policy(&self) -> resources::Lifecycle_policy<'_> {
        resources::Lifecycle_policy::new(self.provider)
    }
    /// Get account_settings resource handler
    pub fn account_settings(&self) -> resources::Account_settings<'_> {
        resources::Account_settings::new(self.provider)
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
