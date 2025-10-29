//! Appconfigdata Service
//!
//! Auto-generated service module for appconfigdata

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appconfigdata
pub struct AppconfigdataService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AppconfigdataService<'a> {
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
