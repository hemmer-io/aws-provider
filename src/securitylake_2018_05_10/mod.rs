//! Securitylake_2018_05_10 Service
//!
//! Auto-generated service module for securitylake_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for securitylake_2018_05_10
pub struct Securitylake_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Securitylake_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get data_lake_exception_subscription resource handler
    pub fn data_lake_exception_subscription(&self) -> resources::Data_lake_exception_subscription<'_> {
        resources::Data_lake_exception_subscription::new(self.provider)
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
