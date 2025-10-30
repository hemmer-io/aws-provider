//! Sso_oidc_2019_06_10 Service
//!
//! Auto-generated service module for sso_oidc_2019_06_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sso_oidc_2019_06_10
pub struct Sso_oidc_2019_06_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sso_oidc_2019_06_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get token resource handler
    pub fn token(&self) -> resources::Token<'_> {
        resources::Token::new(self.provider)
    }
    /// Get token_with_iam resource handler
    pub fn token_with_iam(&self) -> resources::Token_with_iam<'_> {
        resources::Token_with_iam::new(self.provider)
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
