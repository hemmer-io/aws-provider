//! Glacier_2012_06_01 Service
//!
//! Auto-generated service module for glacier_2012_06_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for glacier_2012_06_01
pub struct Glacier_2012_06_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Glacier_2012_06_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get vault_access_policy resource handler
    pub fn vault_access_policy(&self) -> resources::Vault_access_policy<'_> {
        resources::Vault_access_policy::new(self.provider)
    }
    /// Get data_retrieval_policy resource handler
    pub fn data_retrieval_policy(&self) -> resources::Data_retrieval_policy<'_> {
        resources::Data_retrieval_policy::new(self.provider)
    }
    /// Get vault_notifications resource handler
    pub fn vault_notifications(&self) -> resources::Vault_notifications<'_> {
        resources::Vault_notifications::new(self.provider)
    }
    /// Get archive resource handler
    pub fn archive(&self) -> resources::Archive<'_> {
        resources::Archive::new(self.provider)
    }
    /// Get vault resource handler
    pub fn vault(&self) -> resources::Vault<'_> {
        resources::Vault::new(self.provider)
    }
    /// Get job_output resource handler
    pub fn job_output(&self) -> resources::Job_output<'_> {
        resources::Job_output::new(self.provider)
    }
    /// Get vault_lock resource handler
    pub fn vault_lock(&self) -> resources::Vault_lock<'_> {
        resources::Vault_lock::new(self.provider)
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
