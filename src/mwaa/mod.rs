//! Mwaa Service
//!
//! Auto-generated service module for mwaa

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mwaa
pub struct MwaaService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MwaaService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get cli_token resource handler
    pub fn cli_token(&self) -> resources::Cli_token<'_> {
        resources::Cli_token::new(self.provider)
    }
    /// Get web_login_token resource handler
    pub fn web_login_token(&self) -> resources::Web_login_token<'_> {
        resources::Web_login_token::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
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
