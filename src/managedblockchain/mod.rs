//! Managedblockchain Service
//!
//! Auto-generated service module for managedblockchain

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for managedblockchain
pub struct ManagedblockchainService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ManagedblockchainService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get proposal resource handler
    pub fn proposal(&self) -> resources::Proposal<'_> {
        resources::Proposal::new(self.provider)
    }
    /// Get member resource handler
    pub fn member(&self) -> resources::Member<'_> {
        resources::Member::new(self.provider)
    }
    /// Get node resource handler
    pub fn node(&self) -> resources::Node<'_> {
        resources::Node::new(self.provider)
    }
    /// Get accessor resource handler
    pub fn accessor(&self) -> resources::Accessor<'_> {
        resources::Accessor::new(self.provider)
    }
    /// Get network resource handler
    pub fn network(&self) -> resources::Network<'_> {
        resources::Network::new(self.provider)
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
