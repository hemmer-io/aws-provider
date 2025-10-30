//! Sso_2019_06_10 Service
//!
//! Auto-generated service module for sso_2019_06_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sso_2019_06_10
pub struct Sso_2019_06_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sso_2019_06_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get role_credentials resource handler
    pub fn role_credentials(&self) -> resources::Role_credentials<'_> {
        resources::Role_credentials::new(self.provider)
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
