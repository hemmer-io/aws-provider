//! Marketplace_entitlement_service_2017_01_11 Service
//!
//! Auto-generated service module for marketplace_entitlement_service_2017_01_11

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for marketplace_entitlement_service_2017_01_11
pub struct Marketplace_entitlement_service_2017_01_11Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Marketplace_entitlement_service_2017_01_11Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get entitlements resource handler
    pub fn entitlements(&self) -> resources::Entitlements<'_> {
        resources::Entitlements::new(self.provider)
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
