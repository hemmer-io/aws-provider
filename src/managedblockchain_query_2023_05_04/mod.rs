//! Managedblockchain_query_2023_05_04 Service
//!
//! Auto-generated service module for managedblockchain_query_2023_05_04

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for managedblockchain_query_2023_05_04
pub struct Managedblockchain_query_2023_05_04Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Managedblockchain_query_2023_05_04Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get asset_contract resource handler
    pub fn asset_contract(&self) -> resources::Asset_contract<'_> {
        resources::Asset_contract::new(self.provider)
    }
    /// Get transaction resource handler
    pub fn transaction(&self) -> resources::Transaction<'_> {
        resources::Transaction::new(self.provider)
    }
    /// Get token_balance resource handler
    pub fn token_balance(&self) -> resources::Token_balance<'_> {
        resources::Token_balance::new(self.provider)
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
