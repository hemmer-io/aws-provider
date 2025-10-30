//! B2bi_2022_06_23 Service
//!
//! Auto-generated service module for b2bi_2022_06_23

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for b2bi_2022_06_23
pub struct B2bi_2022_06_23Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> B2bi_2022_06_23Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get transformer_job resource handler
    pub fn transformer_job(&self) -> resources::Transformer_job<'_> {
        resources::Transformer_job::new(self.provider)
    }
    /// Get starter_mapping_template resource handler
    pub fn starter_mapping_template(&self) -> resources::Starter_mapping_template<'_> {
        resources::Starter_mapping_template::new(self.provider)
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
