//! Rbin_2021_06_15 Service
//!
//! Auto-generated service module for rbin_2021_06_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rbin_2021_06_15
pub struct Rbin_2021_06_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Rbin_2021_06_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get rule resource handler
    pub fn rule(&self) -> resources::Rule<'_> {
        resources::Rule::new(self.provider)
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
