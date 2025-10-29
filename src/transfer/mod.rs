//! Transfer Service
//!
//! Auto-generated service module for transfer

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for transfer
pub struct TransferService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> TransferService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get host_key resource handler
    pub fn host_key(&self) -> resources::Host_key<'_> {
        resources::Host_key::new(self.provider)
    }
    /// Get access resource handler
    pub fn access(&self) -> resources::Access<'_> {
        resources::Access::new(self.provider)
    }
    /// Get execution resource handler
    pub fn execution(&self) -> resources::Execution<'_> {
        resources::Execution::new(self.provider)
    }
    /// Get ssh_public_key resource handler
    pub fn ssh_public_key(&self) -> resources::Ssh_public_key<'_> {
        resources::Ssh_public_key::new(self.provider)
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
