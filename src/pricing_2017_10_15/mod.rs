//! Pricing_2017_10_15 Service
//!
//! Auto-generated service module for pricing_2017_10_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pricing_2017_10_15
pub struct Pricing_2017_10_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Pricing_2017_10_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get attribute_values resource handler
    pub fn attribute_values(&self) -> resources::Attribute_values<'_> {
        resources::Attribute_values::new(self.provider)
    }
    /// Get price_list_file_url resource handler
    pub fn price_list_file_url(&self) -> resources::Price_list_file_url<'_> {
        resources::Price_list_file_url::new(self.provider)
    }
    /// Get services resource handler
    pub fn services(&self) -> resources::Services<'_> {
        resources::Services::new(self.provider)
    }
    /// Get products resource handler
    pub fn products(&self) -> resources::Products<'_> {
        resources::Products::new(self.provider)
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
