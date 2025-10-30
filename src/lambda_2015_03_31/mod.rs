//! Lambda_2015_03_31 Service
//!
//! Auto-generated service module for lambda_2015_03_31

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for lambda_2015_03_31
pub struct Lambda_2015_03_31Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Lambda_2015_03_31Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get account_settings resource handler
    pub fn account_settings(&self) -> resources::Account_settings<'_> {
        resources::Account_settings::new(self.provider)
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
