//! Transfer_2018_11_05 Service
//!
//! Auto-generated service module for transfer_2018_11_05

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for transfer_2018_11_05
pub struct Transfer_2018_11_05Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Transfer_2018_11_05Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get ssh_public_key resource handler
    pub fn ssh_public_key(&self) -> resources::Ssh_public_key<'_> {
        resources::Ssh_public_key::new(self.provider)
    }
    /// Get execution resource handler
    pub fn execution(&self) -> resources::Execution<'_> {
        resources::Execution::new(self.provider)
    }
    /// Get access resource handler
    pub fn access(&self) -> resources::Access<'_> {
        resources::Access::new(self.provider)
    }
    /// Get host_key resource handler
    pub fn host_key(&self) -> resources::Host_key<'_> {
        resources::Host_key::new(self.provider)
    }
    /// Get security_policy resource handler
    pub fn security_policy(&self) -> resources::Security_policy<'_> {
        resources::Security_policy::new(self.provider)
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
