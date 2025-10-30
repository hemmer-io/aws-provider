//! Marketplace_agreement_2020_03_01 Service
//!
//! Auto-generated service module for marketplace_agreement_2020_03_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for marketplace_agreement_2020_03_01
pub struct Marketplace_agreement_2020_03_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Marketplace_agreement_2020_03_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get agreement resource handler
    pub fn agreement(&self) -> resources::Agreement<'_> {
        resources::Agreement::new(self.provider)
    }
    /// Get agreement_terms resource handler
    pub fn agreement_terms(&self) -> resources::Agreement_terms<'_> {
        resources::Agreement_terms::new(self.provider)
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
