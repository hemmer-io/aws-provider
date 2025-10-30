//! Appconfigdata_2021_11_11 Service
//!
//! Auto-generated service module for appconfigdata_2021_11_11

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appconfigdata_2021_11_11
pub struct Appconfigdata_2021_11_11Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Appconfigdata_2021_11_11Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get latest_configuration resource handler
    pub fn latest_configuration(&self) -> resources::Latest_configuration<'_> {
        resources::Latest_configuration::new(self.provider)
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
